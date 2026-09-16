<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    Plus,
    X,
    LayoutGrid,
    Kanban,
    Table as TableIcon,
    Search,
    Filter,
    Activity,
  } from '@lucide/svelte';

  import { page } from '$app/state';
  import { canManageNumbers } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';
  import { api } from '$lib/api/client';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';
  import { Input } from '$lib/components/ui/input';
  import { auth } from '$lib/stores/auth.svelte';

  // Sub-components
  import NumberCard, { type SongNumber } from './components/NumberCard.svelte';
  import NumberKanbanColumn from './components/NumberKanbanColumn.svelte';
  import NumberTable from './components/NumberTable.svelte';
  import NumberAddModal from './components/NumberAddModal.svelte';
  import NumberLineupModal from './components/NumberLineupModal.svelte';
  import NumberQcDrawer from './components/NumberQcDrawer.svelte';

  let { data } = $props();

  const showId = $derived(page.params.id || '');
  const activeUser = $derived(auth.user || page.data?.user || null);
  const userRole = $derived((activeUser?.role || 'member') as UserRole);
  const currentUserName = $derived(activeUser?.full_name || activeUser?.email || 'Guest');

  let numbers = $state<SongNumber[]>([]);

  $effect(() => {
    numbers = data?.numbers || [];
  });

  const availableRoster = $derived(
    data?.roster ? data.roster.map((m: any) => m.fullName) : []
  );

  let currentView = $state<'grid' | 'kanban' | 'table'>('grid');
  let searchQuery = $state('');
  let selectedStageFilter = $state<string>('all');
  let selectedPmFilter = $state<string>('all');

  $effect(() => {
    const q = page.url.searchParams.get('q');
    const stage = page.url.searchParams.get('stage');
    const view = page.url.searchParams.get('view') as 'grid' | 'kanban' | 'table' | null;
    if (q !== null) searchQuery = q;
    if (stage !== null) selectedStageFilter = stage;
    if (view !== null) currentView = view;
  });

  // Modal Dialog States
  let isQcDrawerOpen = $state(false);
  let activeSongForQc = $state<SongNumber | null>(null);

  let isLineupDrawerOpen = $state(false);
  let activeSongForLineup = $state<SongNumber | null>(null);

  let isAddModalOpen = $state(false);

  const stageStats = $derived({
    total: numbers.length,
    draft: numbers.filter((n) => n.stage === 'draft').length,
    in_practice: numbers.filter((n) => n.stage === 'in_practice').length,
    ready_for_qc: numbers.filter((n) => n.stage === 'ready_for_qc').length,
    qc_approved: numbers.filter((n) => n.stage === 'qc_approved').length,
    stage_ready: numbers.filter((n) => n.stage === 'stage_ready').length,
  });

  const uniquePms = $derived(
    Array.from(new Set(numbers.map((n) => n.pmName))).sort()
  );

  const filteredNumbers = $derived(
    numbers.filter((song) => {
      const matchesStage =
        selectedStageFilter === 'all' || song.stage === selectedStageFilter;
      const matchesPm =
        selectedPmFilter === 'all' || song.pmName === selectedPmFilter;

      if (!matchesStage || !matchesPm) return false;

      if (!searchQuery.trim()) return true;

      const q = searchQuery.toLowerCase().trim();
      const titleMatch = song.title.toLowerCase().includes(q);
      const genreMatch = song.genre?.toLowerCase().includes(q);
      const pmMatch = song.pmName.toLowerCase().includes(q);
      const reviewerMatch = song.qcReviewer.toLowerCase().includes(q);
      const performersMatch = Object.values(song.lineup || {}).some((p) =>
        p?.toLowerCase().includes(q)
      );

      return titleMatch || genreMatch || pmMatch || reviewerMatch || performersMatch;
    })
  );

  function openQcDrawer(song: SongNumber) {
    activeSongForQc = song;
    isQcDrawerOpen = true;
  }

  function openLineupDrawer(song: SongNumber) {
    activeSongForLineup = song;
    isLineupDrawerOpen = true;
  }

  async function handleCreateNumber(payload: {
    title: string;
    genre: string;
    pm_name: string;
    qc_reviewer: string;
  }) {
    try {
      const created = await api.shows.createNumber(showId, payload);
      numbers = [created, ...numbers];
    } catch {
      const newSong: SongNumber = {
        id: `song-${Date.now()}`,
        title: payload.title,
        genre: payload.genre,
        pmName: payload.pm_name,
        qcReviewer: payload.qc_reviewer,
        stage: 'draft',
        lineup: {},
      };
      numbers = [newSong, ...numbers];
    }
  }

  async function handleQcSubmit(songId: string, verdict: 'pass' | 'revision', notes: string) {
    try {
      const updated = await api.shows.submitQc(showId, songId, { verdict, notes });
      numbers = numbers.map((n) => (n.id === updated.id ? updated : n));
    } catch {
      const nextStage = verdict === 'pass' ? 'qc_approved' : 'in_practice';
      numbers = numbers.map((n) => {
        if (n.id === songId) {
          return {
            ...n,
            stage: nextStage,
            qcNotes: notes || (verdict === 'pass' ? 'Passed Quality Check.' : 'Revision requested by reviewer.'),
          };
        }
        return n;
      });
    }
  }

  async function handleLineupSubmit(songId: string, lineup: NonNullable<SongNumber['lineup']>) {
    try {
      const updated = await api.shows.updateLineup(showId, songId, lineup);
      numbers = numbers.map((n) => (n.id === updated.id ? updated : n));
    } catch {
      numbers = numbers.map((n) => {
        if (n.id === songId) {
          return { ...n, lineup };
        }
        return n;
      });
    }
  }

  async function advanceStatus(song: SongNumber, nextStage: SongNumber['stage']) {
    try {
      const updated = await api.shows.updateStage(showId, song.id, nextStage);
      numbers = numbers.map((n) => (n.id === updated.id ? updated : n));
    } catch {
      numbers = numbers.map((n) => {
        if (n.id === song.id) {
          return { ...n, stage: nextStage };
        }
        return n;
      });
    }
  }
</script>

<div class="flex flex-col gap-5">
  <!-- Interactive Header Bar with Stage Counts -->
  <Card class="p-5 flex flex-col gap-4 shadow-sm">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div>
        <div class="flex items-center gap-2 mb-1">
          <Badge variant="outline" class="bg-primary/10 text-primary border-primary/20 gap-1 font-bold">
            <Activity class="w-3 h-3 text-primary" />
            <span>{$tStore('studio_shows.tab_numbers')}</span>
          </Badge>
          <span class="text-xs text-muted-foreground font-semibold">
            {stageStats.total} {$tStore('studio_shows.pipeline_all')}
          </span>
        </div>
        <h2 class="text-xl font-extrabold text-foreground tracking-tight m-0">
          {$tStore('studio_shows.tab_numbers')}
        </h2>
      </div>

      <div class="flex items-center gap-2 flex-wrap">
        <!-- View Mode Switcher -->
        <div class="flex items-center bg-muted/60 border border-border/50 rounded-lg p-0.5 gap-0.5">
          <button
            type="button"
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold cursor-pointer transition-all {currentView === 'grid' ? 'bg-card text-primary shadow-xs font-bold border border-border/40' : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => (currentView = 'grid')}
            title="Bento Grid View"
          >
            <LayoutGrid class="w-3.5 h-3.5" />
            <span>{$tStore('studio_shows.view_grid')}</span>
          </button>
          <button
            type="button"
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold cursor-pointer transition-all {currentView === 'kanban' ? 'bg-card text-primary shadow-xs font-bold border border-border/40' : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => (currentView = 'kanban')}
            title="Kanban Board View"
          >
            <Kanban class="w-3.5 h-3.5" />
            <span>{$tStore('studio_shows.view_kanban')}</span>
          </button>
          <button
            type="button"
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold cursor-pointer transition-all {currentView === 'table' ? 'bg-card text-primary shadow-xs font-bold border border-border/40' : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => (currentView = 'table')}
            title="Compact Table View"
          >
            <TableIcon class="w-3.5 h-3.5" />
            <span>{$tStore('studio_shows.view_table')}</span>
          </button>
        </div>

        {#if canManageNumbers(userRole)}
          <Button
            size="sm"
            class="gap-1.5"
            onclick={() => (isAddModalOpen = true)}
          >
            <Plus class="w-4 h-4" />
            <span>{$tStore('studio_shows.btn_add_number')}</span>
          </Button>
        {/if}
      </div>
    </div>

    <!-- Stage Funnel Filter Pills -->
    <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-2 pt-2 border-t border-border">
      <button
        type="button"
        class="flex flex-col p-2.5 rounded-xl border text-left transition-all cursor-pointer {selectedStageFilter === 'all' ? 'border-primary bg-primary/10 shadow-xs' : 'border-border bg-card hover:bg-muted/40'}"
        onclick={() => (selectedStageFilter = 'all')}
      >
        <span class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground">{$tStore('studio_shows.filter_all_stages')}</span>
        <span class="text-lg font-black text-foreground">{stageStats.total}</span>
      </button>

      <button
        type="button"
        class="flex flex-col p-2.5 rounded-xl border text-left transition-all cursor-pointer {selectedStageFilter === 'draft' ? 'border-slate-400 dark:border-slate-600 bg-slate-500/10 shadow-xs' : 'border-border bg-card hover:bg-muted/40'}"
        onclick={() => (selectedStageFilter = 'draft')}
      >
        <span class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground">{$tStore('studio_shows.kanban_draft')}</span>
        <span class="text-lg font-black text-foreground">{stageStats.draft}</span>
      </button>

      <button
        type="button"
        class="flex flex-col p-2.5 rounded-xl border text-left transition-all cursor-pointer {selectedStageFilter === 'in_practice' ? 'border-blue-500 bg-blue-500/10 shadow-xs' : 'border-border bg-card hover:bg-muted/40'}"
        onclick={() => (selectedStageFilter = 'in_practice')}
      >
        <span class="text-[10px] font-bold uppercase tracking-wider text-blue-600 dark:text-blue-400">{$tStore('studio_shows.kanban_practice')}</span>
        <span class="text-lg font-black text-blue-600 dark:text-blue-400">{stageStats.in_practice}</span>
      </button>

      <button
        type="button"
        class="flex flex-col p-2.5 rounded-xl border text-left transition-all cursor-pointer {selectedStageFilter === 'ready_for_qc' ? 'border-primary bg-primary/10 shadow-xs' : 'border-border bg-card hover:bg-muted/40'}"
        onclick={() => (selectedStageFilter = 'ready_for_qc')}
      >
        <span class="text-[10px] font-bold uppercase tracking-wider text-primary">{$tStore('studio_shows.kanban_ready_qc')}</span>
        <span class="text-lg font-black text-primary">{stageStats.ready_for_qc}</span>
      </button>

      <button
        type="button"
        class="flex flex-col p-2.5 rounded-xl border text-left transition-all cursor-pointer {selectedStageFilter === 'qc_approved' ? 'border-indigo-500 bg-indigo-500/10 shadow-xs' : 'border-border bg-card hover:bg-muted/40'}"
        onclick={() => (selectedStageFilter = 'qc_approved')}
      >
        <span class="text-[10px] font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400">{$tStore('studio_shows.kanban_qc_approved')}</span>
        <span class="text-lg font-black text-indigo-600 dark:text-indigo-400">{stageStats.qc_approved}</span>
      </button>

      <button
        type="button"
        class="flex flex-col p-2.5 rounded-xl border text-left transition-all cursor-pointer {selectedStageFilter === 'stage_ready' ? 'border-emerald-500 bg-emerald-500/10 shadow-xs' : 'border-border bg-card hover:bg-muted/40'}"
        onclick={() => (selectedStageFilter = 'stage_ready')}
      >
        <span class="text-[10px] font-bold uppercase tracking-wider text-emerald-600 dark:text-emerald-400">{$tStore('studio_shows.kanban_stage_ready')}</span>
        <span class="text-lg font-black text-emerald-600 dark:text-emerald-400">{stageStats.stage_ready}</span>
      </button>
    </div>
  </Card>

  <!-- Filter & Search Toolbar -->
  <div class="flex flex-col sm:flex-row items-center justify-between gap-3">
    <div class="relative w-full sm:w-80">
      <Search class="w-4 h-4 text-muted-foreground absolute left-3 top-1/2 -translate-y-1/2 pointer-events-none" />
      <Input
        type="text"
        placeholder={$tStore('studio_shows.search_placeholder')}
        bind:value={searchQuery}
        class="pl-9 text-xs h-9 bg-card"
      />
      {#if searchQuery}
        <button
          type="button"
          class="absolute right-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
          onclick={() => (searchQuery = '')}
        >
          <X class="w-3.5 h-3.5" />
        </button>
      {/if}
    </div>

    <div class="flex items-center gap-2.5 w-full sm:w-auto">
      <div class="flex items-center gap-1.5 bg-card border border-border rounded-lg px-2.5 py-1 text-xs">
        <Filter class="w-3.5 h-3.5 text-muted-foreground" />
        <span class="text-muted-foreground font-medium">PM:</span>
        <select
          bind:value={selectedPmFilter}
          class="bg-transparent border-0 text-xs font-semibold text-foreground outline-none cursor-pointer"
        >
          <option value="all">{$tStore('studio_shows.filter_all_pms')}</option>
          {#each uniquePms as pm}
            <option value={pm}>{pm}</option>
          {/each}
        </select>
      </div>

      {#if selectedStageFilter !== 'all' || selectedPmFilter !== 'all' || searchQuery}
        <Button
          variant="ghost"
          size="sm"
          class="h-8 px-2 text-xs text-muted-foreground hover:text-foreground"
          onclick={() => {
            selectedStageFilter = 'all';
            selectedPmFilter = 'all';
            searchQuery = '';
          }}
        >
          {$tStore('studio_shows.btn_reset_filters')}
        </Button>
      {/if}
    </div>
  </div>

  {#if filteredNumbers.length === 0}
    <Card class="p-12 text-center flex flex-col items-center justify-center gap-3">
      <div class="w-12 h-12 rounded-full bg-muted flex items-center justify-center text-muted-foreground">
        <Search class="w-6 h-6" />
      </div>
      <div>
        <h3 class="text-base font-bold text-foreground m-0">{$tStore('studio_shows.empty_title')}</h3>
        <p class="text-xs text-muted-foreground m-0 mt-1 max-w-sm">
          {$tStore('studio_shows.empty_desc')}
        </p>
      </div>
      <Button
        variant="outline"
        size="sm"
        onclick={() => {
          selectedStageFilter = 'all';
          selectedPmFilter = 'all';
          searchQuery = '';
        }}
      >
        {$tStore('studio_shows.btn_reset_filters')}
      </Button>
    </Card>
  {:else if currentView === 'grid'}
    <!-- VIEW 1: SCALABLE BENTO GRID (DEFAULT) -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each filteredNumbers as song, index (song.id)}
        <NumberCard
          {song}
          {index}
          {showId}
          {userRole}
          {currentUserName}
          onOpenLineup={openLineupDrawer}
          onOpenQc={openQcDrawer}
          onAdvanceStage={advanceStatus}
        />
      {/each}
    </div>
  {:else if currentView === 'kanban'}
    <!-- VIEW 2: 5-STAGE KANBAN BOARD -->
    <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-5 gap-3.5 overflow-x-auto pb-2">
      <NumberKanbanColumn
        title={$tStore('show_mgmt.status.draft')}
        stageKey="draft"
        count={filteredNumbers.filter((n) => n.stage === 'draft').length}
        colorClass="text-slate-600 dark:text-slate-300"
        bgBadgeClass="bg-slate-200 dark:bg-slate-800 text-slate-700 dark:text-slate-300"
        numbers={filteredNumbers}
        {userRole}
        {currentUserName}
        onOpenLineup={openLineupDrawer}
        onOpenQc={openQcDrawer}
        onAdvanceStage={advanceStatus}
      />
      <NumberKanbanColumn
        title={$tStore('studio_shows.kanban_practice')}
        stageKey="in_practice"
        count={filteredNumbers.filter((n) => n.stage === 'in_practice').length}
        colorClass="text-blue-600"
        bgBadgeClass="bg-blue-100 dark:bg-blue-950/50 text-blue-700 dark:text-blue-300"
        numbers={filteredNumbers}
        {userRole}
        {currentUserName}
        onOpenLineup={openLineupDrawer}
        onOpenQc={openQcDrawer}
        onAdvanceStage={advanceStatus}
      />
      <NumberKanbanColumn
        title={$tStore('studio_shows.kanban_ready_qc')}
        stageKey="ready_for_qc"
        count={filteredNumbers.filter((n) => n.stage === 'ready_for_qc').length}
        colorClass="text-primary"
        bgBadgeClass="bg-orange-100 dark:bg-orange-950/50 text-primary"
        numbers={filteredNumbers}
        {userRole}
        {currentUserName}
        onOpenLineup={openLineupDrawer}
        onOpenQc={openQcDrawer}
        onAdvanceStage={advanceStatus}
      />
      <NumberKanbanColumn
        title={$tStore('studio_shows.kanban_qc_approved')}
        stageKey="qc_approved"
        count={filteredNumbers.filter((n) => n.stage === 'qc_approved').length}
        colorClass="text-indigo-600"
        bgBadgeClass="bg-indigo-100 dark:bg-indigo-950/50 text-indigo-700 dark:text-indigo-300"
        numbers={filteredNumbers}
        {userRole}
        {currentUserName}
        onOpenLineup={openLineupDrawer}
        onOpenQc={openQcDrawer}
        onAdvanceStage={advanceStatus}
      />
      <NumberKanbanColumn
        title={$tStore('studio_shows.kanban_stage_ready')}
        stageKey="stage_ready"
        count={filteredNumbers.filter((n) => n.stage === 'stage_ready').length}
        colorClass="text-emerald-600"
        bgBadgeClass="bg-emerald-100 dark:bg-emerald-950/50 text-emerald-700 dark:text-emerald-300"
        numbers={filteredNumbers}
        {userRole}
        {currentUserName}
        onOpenLineup={openLineupDrawer}
        onOpenQc={openQcDrawer}
        onAdvanceStage={advanceStatus}
      />
    </div>
  {:else if currentView === 'table'}
    <!-- VIEW 3: COMPACT DATA TABLE -->
    <NumberTable
      numbers={filteredNumbers}
      {showId}
      {userRole}
      {currentUserName}
      onOpenLineup={openLineupDrawer}
      onOpenQc={openQcDrawer}
      onAdvanceStage={advanceStatus}
    />
  {/if}
</div>

<!-- Add Modal -->
<NumberAddModal
  bind:open={isAddModalOpen}
  onOpenChange={(open) => (isAddModalOpen = open)}
  {availableRoster}
  onCreate={handleCreateNumber}
/>

<!-- QC Drawer -->
<NumberQcDrawer
  bind:open={isQcDrawerOpen}
  song={activeSongForQc}
  onOpenChange={(open) => (isQcDrawerOpen = open)}
  onSubmitVerdict={handleQcSubmit}
/>

<!-- Lineup Drawer -->
<NumberLineupModal
  bind:open={isLineupDrawerOpen}
  song={activeSongForLineup}
  {availableRoster}
  onOpenChange={(open) => (isLineupDrawerOpen = open)}
  onSaveLineup={handleLineupSubmit}
/>
