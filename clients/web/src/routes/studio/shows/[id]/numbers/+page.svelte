<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    Music,
    CircleCheck,
    Clock,
    UserCheck,
    Users,
    MessageSquare,
    Plus,
    X,
    LayoutGrid,
    Kanban,
    Table as TableIcon,
    Search,
    Filter,
    Activity,
    MicVocal,
    Guitar,
    Disc3,
    ShieldCheck,
    ThumbsUp,
    CircleAlert,
    RefreshCw,
    Sparkles,
  } from '@lucide/svelte';

  import { page } from '$app/state';
  import {
    canManageNumbers,
    canManageSongScoped,
    canAuditSongScoped,
  } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';
  import { api } from '$lib/api/client';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogDescription,
    DialogFooter,
  } from '$lib/components/ui/dialog';
  import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
  } from '$lib/components/ui/table';

  interface SongNumber {
    id: string;
    title: string;
    genre: string;
    pmName: string;
    stage: 'draft' | 'in_practice' | 'ready_for_qc' | 'qc_approved' | 'stage_ready';
    qcReviewer: string;
    qcNotes?: string;
    lineup?: {
      vocalLead?: string;
      guitarLead?: string;
      bass?: string;
      drums?: string;
      keys?: string;
    };
  }

  let { data } = $props();

  const showId = $derived(page.params.id || '');
  const userRole = $derived((page.data?.user?.role || 'admin') as UserRole);
  const currentUserName = $derived(page.data?.user?.fullName || 'Administrator');

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

  let isQcDrawerOpen = $state(false);
  let activeSongForQc = $state<SongNumber | null>(null);
  let qcVerdict = $state<'pass' | 'revision'>('pass');
  let qcNotesInput = $state('');

  let isLineupDrawerOpen = $state(false);
  let activeSongForLineup = $state<SongNumber | null>(null);
  let formVocalLead = $state('');
  let formGuitarLead = $state('');
  let formBass = $state('');
  let formDrums = $state('');
  let formKeys = $state('');

  let isAddModalOpen = $state(false);
  let newTitle = $state('');
  let newGenre = $state('');
  let newPm = $state('');
  let newQcReviewer = $state('');

  $effect(() => {
    if (availableRoster.length > 0) {
      if (!newPm) newPm = availableRoster[0];
      if (!newQcReviewer) newQcReviewer = availableRoster[1] || availableRoster[0];
    }
  });

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
    qcNotesInput = song.qcNotes || '';
    qcVerdict = 'pass';
    isQcDrawerOpen = true;
  }

  function openLineupDrawer(song: SongNumber) {
    activeSongForLineup = song;
    formVocalLead = song.lineup?.vocalLead || '';
    formGuitarLead = song.lineup?.guitarLead || '';
    formBass = song.lineup?.bass || '';
    formDrums = song.lineup?.drums || '';
    formKeys = song.lineup?.keys || '';
    isLineupDrawerOpen = true;
  }

  async function handleLineupSubmit(e: Event) {
    e.preventDefault();
    if (!activeSongForLineup) return;

    const lineupPayload = {
      vocalLead: formVocalLead || undefined,
      guitarLead: formGuitarLead || undefined,
      bass: formBass || undefined,
      drums: formDrums || undefined,
      keys: formKeys || undefined,
    };

    numbers = numbers.map((n) =>
      n.id === activeSongForLineup?.id
        ? {
            ...n,
            lineup: lineupPayload,
          }
        : n
    );
    isLineupDrawerOpen = false;

    try {
      await api.shows.updateLineup(showId, activeSongForLineup.id, lineupPayload);
    } catch (err) {
      console.error('Failed to sync lineup to backend:', err);
    }
  }

  async function handleQcSubmit(e: Event) {
    e.preventDefault();
    if (!activeSongForQc) return;

    const songId = activeSongForQc.id;
    const verdict = qcVerdict;
    const notes = qcNotesInput;

    numbers = numbers.map((n) =>
      n.id === songId
        ? {
            ...n,
            stage: verdict === 'pass' ? 'qc_approved' : 'in_practice',
            qcNotes: notes,
          }
        : n
    );

    isQcDrawerOpen = false;
    activeSongForQc = null;

    try {
      await api.shows.submitQc(showId, songId, { verdict, notes });
    } catch (err) {
      console.error('Failed to submit QC to backend:', err);
    }
  }

  async function advanceStatus(song: SongNumber, nextStage: SongNumber['stage']) {
    numbers = numbers.map((n) => (n.id === song.id ? { ...n, stage: nextStage } : n));
    try {
      await api.shows.updateStage(showId, song.id, nextStage);
    } catch (err) {
      console.error('Failed to update stage in backend:', err);
    }
  }

  function openAddModal() {
    newTitle = '';
    newGenre = '';
    newPm = availableRoster[0] || '';
    newQcReviewer = availableRoster[1] || availableRoster[0] || '';
    isAddModalOpen = true;
  }

  async function handleCreateNumber(e: Event) {
    e.preventDefault();
    if (!newTitle.trim()) return;

    const newSong: SongNumber = {
      id: `num-${Date.now()}`,
      title: newTitle.trim(),
      genre: newGenre.trim() || 'Live Performance',
      pmName: newPm,
      stage: 'draft',
      qcReviewer: newQcReviewer,
      lineup: {},
    };

    numbers = [newSong, ...numbers];
    isAddModalOpen = false;

    try {
      await api.shows.createNumber(showId, {
        title: newSong.title,
        genre: newSong.genre,
        pm_name: newSong.pmName,
        qc_reviewer: newSong.qcReviewer,
      });
    } catch (err) {
      console.error('Failed to create number in backend:', err);
    }
  }

  function getStageBadgeVariant(stage: SongNumber['stage']) {
    switch (stage) {
      case 'draft':
        return 'bg-slate-100 text-slate-700 border-slate-200';
      case 'in_practice':
        return 'bg-blue-500/10 text-blue-600 border-blue-500/20';
      case 'ready_for_qc':
        return 'bg-primary/10 text-primary border-primary/20';
      case 'qc_approved':
        return 'bg-indigo-500/10 text-indigo-600 border-indigo-500/20';
      case 'stage_ready':
        return 'bg-emerald-500/10 text-emerald-600 border-emerald-500/20';
    }
  }

  function getStageLabel(stage: SongNumber['stage']) {
    switch (stage) {
      case 'draft':
        return $tStore('studio_shows.kanban_draft');
      case 'in_practice':
        return $tStore('studio_shows.kanban_practice');
      case 'ready_for_qc':
        return $tStore('studio_shows.kanban_ready_qc');
      case 'qc_approved':
        return $tStore('studio_shows.kanban_qc_approved');
      case 'stage_ready':
        return $tStore('studio_shows.kanban_stage_ready');
    }
  }
</script>

<div class="flex flex-col gap-4">
  <!-- Pipeline Funnel Summary Banner (1-Click Stage Filter) -->
  <Card class="p-4 flex flex-col gap-3.5 shadow-sm">
    <div class="flex items-center justify-between flex-wrap gap-3">
      <div class="flex items-center gap-2 flex-wrap">
        <Activity class="w-4 h-4 text-primary" />
        <span class="text-sm font-bold text-foreground">{$tStore('studio_shows.pipeline_heading')}</span>
        <span class="text-xs text-muted-foreground ml-1">{$tStore('studio_shows.pipeline_sub').replace('{count}', numbers.length.toString())}</span>
      </div>

      {#if canManageNumbers(userRole)}
        <Button
          size="sm"
          onclick={openAddModal}
          id="btn-add-music-number"
          class="gap-1.5"
        >
          <Plus class="w-4 h-4" />
          <span>{$tStore('studio_shows.btn_add_number')}</span>
        </Button>
      {/if}
    </div>

    <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-2.5">
      <button
        type="button"
        class="bg-card border border-border/80 rounded-xl p-2.5 flex flex-col items-start gap-1 cursor-pointer transition-all hover:bg-muted/40 hover:border-primary/40 hover:-translate-y-0.5 hover:shadow-xs text-left {selectedStageFilter === 'all' ? 'bg-primary/10 border-primary shadow-xs' : ''}"
        onclick={() => (selectedStageFilter = 'all')}
      >
        <span class="text-lg font-extrabold leading-none text-foreground">{stageStats.total}</span>
        <span class="text-[11px] font-semibold text-muted-foreground">{$tStore('studio_shows.pipeline_all')}</span>
      </button>

      <button
        type="button"
        class="bg-card border border-border/80 rounded-xl p-2.5 flex flex-col items-start gap-1 cursor-pointer transition-all hover:bg-muted/40 hover:border-primary/40 hover:-translate-y-0.5 hover:shadow-xs text-left {selectedStageFilter === 'draft' ? 'bg-primary/10 border-primary shadow-xs' : ''}"
        onclick={() => (selectedStageFilter = 'draft')}
      >
        <span class="text-lg font-extrabold leading-none text-muted-foreground">{stageStats.draft}</span>
        <span class="text-[11px] font-semibold text-muted-foreground">{$tStore('studio_shows.kanban_draft')}</span>
      </button>

      <button
        type="button"
        class="bg-card border border-border/80 rounded-xl p-2.5 flex flex-col items-start gap-1 cursor-pointer transition-all hover:bg-muted/40 hover:border-primary/40 hover:-translate-y-0.5 hover:shadow-xs text-left {selectedStageFilter === 'in_practice' ? 'bg-primary/10 border-primary shadow-xs' : ''}"
        onclick={() => (selectedStageFilter = 'in_practice')}
      >
        <span class="text-lg font-extrabold leading-none text-primary">{stageStats.in_practice}</span>
        <span class="text-[11px] font-semibold text-muted-foreground">{$tStore('studio_shows.kanban_practice')}</span>
      </button>

      <button
        type="button"
        class="bg-card border border-border/80 rounded-xl p-2.5 flex flex-col items-start gap-1 cursor-pointer transition-all hover:bg-muted/40 hover:border-primary/40 hover:-translate-y-0.5 hover:shadow-xs text-left {selectedStageFilter === 'ready_for_qc' ? 'bg-primary/10 border-primary shadow-xs' : ''}"
        onclick={() => (selectedStageFilter = 'ready_for_qc')}
      >
        <span class="text-lg font-extrabold leading-none text-primary">{stageStats.ready_for_qc}</span>
        <span class="text-[11px] font-semibold text-muted-foreground">{$tStore('studio_shows.kanban_ready_qc')}</span>
      </button>

      <button
        type="button"
        class="bg-card border border-border/80 rounded-xl p-2.5 flex flex-col items-start gap-1 cursor-pointer transition-all hover:bg-muted/40 hover:border-primary/40 hover:-translate-y-0.5 hover:shadow-xs text-left {selectedStageFilter === 'qc_approved' ? 'bg-primary/10 border-primary shadow-xs' : ''}"
        onclick={() => (selectedStageFilter = 'qc_approved')}
      >
        <span class="text-lg font-extrabold leading-none text-primary">{stageStats.qc_approved}</span>
        <span class="text-[11px] font-semibold text-muted-foreground">{$tStore('studio_shows.kanban_qc_approved')}</span>
      </button>

      <button
        type="button"
        class="bg-card border border-border/80 rounded-xl p-2.5 flex flex-col items-start gap-1 cursor-pointer transition-all hover:bg-muted/40 hover:border-primary/40 hover:-translate-y-0.5 hover:shadow-xs text-left {selectedStageFilter === 'stage_ready' ? 'bg-primary/10 border-primary shadow-xs' : ''}"
        onclick={() => (selectedStageFilter = 'stage_ready')}
      >
        <span class="text-lg font-extrabold leading-none text-emerald-600">{stageStats.stage_ready}</span>
        <span class="text-[11px] font-semibold text-muted-foreground">{$tStore('studio_shows.kanban_stage_ready')}</span>
      </button>
    </div>
  </Card>

  <!-- Search, Filter & View Controls Toolbar -->
  <Card class="p-3 flex items-center justify-between flex-wrap gap-3 shadow-sm">
    <div class="relative flex-1 min-w-[240px] max-w-sm">
      <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground pointer-events-none" />
      <Input
        type="text"
        placeholder={$tStore('studio_shows.search_placeholder')}
        bind:value={searchQuery}
        class="pl-9 pr-8 h-9 text-xs"
        id="input-search-numbers"
      />
      {#if searchQuery}
        <button
          type="button"
          class="absolute right-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground cursor-pointer"
          onclick={() => (searchQuery = '')}
          aria-label="Clear Search"
        >
          <X class="w-3.5 h-3.5" />
        </button>
      {/if}
    </div>

    <div class="flex items-center flex-wrap gap-2.5">
      <div class="flex items-center gap-1.5 bg-slate-50 border border-slate-200 rounded-lg px-2.5 py-1">
        <Filter class="w-3.5 h-3.5 text-muted-foreground shrink-0" />
        <select bind:value={selectedStageFilter} class="bg-transparent border-0 text-xs font-semibold text-slate-700 outline-none cursor-pointer" id="select-stage-filter">
          <option value="all">{$tStore('studio_shows.filter_all_stages')} ({numbers.length})</option>
          <option value="draft">{$tStore('studio_shows.kanban_draft')} ({stageStats.draft})</option>
          <option value="in_practice">{$tStore('studio_shows.kanban_practice')} ({stageStats.in_practice})</option>
          <option value="ready_for_qc">{$tStore('studio_shows.kanban_ready_qc')} ({stageStats.ready_for_qc})</option>
          <option value="qc_approved">{$tStore('studio_shows.kanban_qc_approved')} ({stageStats.qc_approved})</option>
          <option value="stage_ready">{$tStore('studio_shows.kanban_stage_ready')} ({stageStats.stage_ready})</option>
        </select>
      </div>

      <div class="flex items-center gap-1.5 bg-slate-50 border border-slate-200 rounded-lg px-2.5 py-1">
        <Users class="w-3.5 h-3.5 text-muted-foreground shrink-0" />
        <select bind:value={selectedPmFilter} class="bg-transparent border-0 text-xs font-semibold text-slate-700 outline-none cursor-pointer" id="select-pm-filter">
          <option value="all">{$tStore('studio_shows.filter_all_pms')}</option>
          {#each uniquePms as pm}
            <option value={pm}>{pm}</option>
          {/each}
        </select>
      </div>

      <!-- View Switcher -->
      <div class="flex items-center bg-slate-100 rounded-lg p-0.5 gap-0.5" role="group" aria-label="View Switcher">
        <button
          type="button"
          class="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-all {currentView === 'grid' ? 'bg-white text-primary shadow-sm font-bold' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (currentView = 'grid')}
          id="btn-view-grid"
          title={$tStore('studio_shows.view_grid')}
        >
          <LayoutGrid class="w-3.5 h-3.5" />
          <span>{$tStore('studio_shows.view_grid')}</span>
        </button>

        <button
          type="button"
          class="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-all {currentView === 'kanban' ? 'bg-white text-primary shadow-sm font-bold' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (currentView = 'kanban')}
          id="btn-view-kanban"
          title={$tStore('studio_shows.view_kanban')}
        >
          <Kanban class="w-3.5 h-3.5" />
          <span>{$tStore('studio_shows.view_kanban')}</span>
        </button>

        <button
          type="button"
          class="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-all {currentView === 'table' ? 'bg-white text-primary shadow-sm font-bold' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (currentView = 'table')}
          id="btn-view-table"
          title={$tStore('studio_shows.view_table')}
        >
          <TableIcon class="w-3.5 h-3.5" />
          <span>{$tStore('studio_shows.view_table')}</span>
        </button>
      </div>
    </div>
  </Card>

  <!-- Empty State when filters produce zero matches -->
  {#if filteredNumbers.length === 0}
    <Card class="flex flex-col items-center justify-center p-12 text-center gap-3">
      <Music class="w-10 h-10 text-muted-foreground" />
      <h3 class="text-base font-bold text-foreground m-0">{$tStore('studio_shows.empty_title')}</h3>
      <p class="text-xs text-muted-foreground max-w-sm m-0">{$tStore('studio_shows.empty_desc')}</p>
      <Button
        variant="outline"
        size="sm"
        onclick={() => {
          searchQuery = '';
          selectedStageFilter = 'all';
          selectedPmFilter = 'all';
        }}
      >
        {$tStore('studio_shows.btn_reset_filters')}
      </Button>
    </Card>
  {:else if currentView === 'grid'}
    <!-- VIEW 1: SCALABLE BENTO GRID (DEFAULT) -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each filteredNumbers as song, index (song.id)}
        <Card class="p-4 flex flex-col justify-between gap-3 transition-all duration-200 hover:-translate-y-0.5 hover:border-primary/40 hover:shadow-md {song.stage === 'stage_ready' ? 'border-l-4 border-l-emerald-500' : ''}">
          <div>
            <div class="flex items-center justify-between mb-2">
              <span class="font-mono text-[11px] font-bold text-muted-foreground bg-muted px-2 py-0.5 rounded">#{index + 1}</span>
              <Badge variant="outline" class="font-bold text-[11px] {getStageBadgeVariant(song.stage)}">
                {getStageLabel(song.stage)}
              </Badge>
            </div>

            <h3 class="text-base font-bold text-foreground leading-snug m-0">{song.title}</h3>
            <div class="text-xs text-muted-foreground font-medium mt-0.5">{song.genre}</div>
            <div class="flex flex-wrap gap-2 mt-2 text-xs text-muted-foreground">
              <span class="bg-muted/50 px-1.5 py-0.5 rounded border border-border"><strong>PM:</strong> {song.pmName}</span>
              <span class="bg-muted/50 px-1.5 py-0.5 rounded border border-border"><strong>QC:</strong> {song.qcReviewer}</span>
            </div>

            <!-- Lineup Section -->
            <div class="mt-3 flex flex-col gap-1.5">
              <div class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">Band Allocation:</div>
              <div class="flex flex-wrap gap-1.5">
                {#if song.lineup?.vocalLead}
                  <span class="inline-flex items-center gap-1 text-[11px] font-semibold px-2 py-0.5 rounded bg-primary/10 text-primary">
                    <MicVocal class="w-2.5 h-2.5" /> {song.lineup.vocalLead}
                  </span>
                {/if}
                {#if song.lineup?.guitarLead}
                  <span class="inline-flex items-center gap-1 text-[11px] font-semibold px-2 py-0.5 rounded bg-blue-500/10 text-blue-600">
                    <Guitar class="w-2.5 h-2.5" /> {song.lineup.guitarLead}
                  </span>
                {/if}
                {#if song.lineup?.bass}
                  <span class="inline-flex items-center gap-1 text-[11px] font-semibold px-2 py-0.5 rounded bg-purple-500/10 text-purple-600">
                    <Disc3 class="w-2.5 h-2.5" /> {song.lineup.bass}
                  </span>
                {/if}
                {#if song.lineup?.drums}
                  <span class="inline-flex items-center gap-1 text-[11px] font-semibold px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-600">
                    🥁 {song.lineup.drums}
                  </span>
                {/if}
                {#if song.lineup?.keys}
                  <span class="inline-flex items-center gap-1 text-[11px] font-semibold px-2 py-0.5 rounded bg-teal-500/10 text-teal-600">
                    🎹 {song.lineup.keys}
                  </span>
                {/if}
                {#if !song.lineup?.vocalLead && !song.lineup?.guitarLead && !song.lineup?.bass && !song.lineup?.drums && !song.lineup?.keys}
                  <span class="text-[11px] text-muted-foreground italic bg-muted/40 px-2 py-0.5 rounded">Lineup Unassigned</span>
                {/if}
              </div>
            </div>

            <!-- QC Notes Box -->
            {#if song.qcNotes}
              <div class="mt-2.5 flex items-start gap-1.5 text-xs p-2 rounded-lg border {song.stage === 'ready_for_qc' ? 'bg-primary/10 text-primary border-primary/30' : 'bg-muted/40 text-foreground border-border'}">
                <MessageSquare class="w-3.5 h-3.5 shrink-0 mt-0.5" />
                <span>{song.qcNotes}</span>
              </div>
            {/if}
          </div>

          <!-- Responsive Action Buttons Row (Key-Scoped Roles) -->
          <div class="flex flex-col gap-2 pt-2 border-t border-border">
            <div class="flex flex-wrap gap-1.5 w-full">
              <!-- Cross-Screen Link to Sprint Timetable -->
              <a
                href="/studio/shows/{showId}/sprints?song={encodeURIComponent(song.title)}"
                class="flex-1 inline-flex items-center justify-center gap-1.5 text-xs font-semibold px-2.5 py-1.5 rounded-lg bg-slate-100 text-slate-700 hover:bg-slate-200 transition-colors"
                title="View all scheduled rehearsal sessions in Sprint Calendar"
              >
                <Clock class="w-3 h-3" />
                <span>Sprint</span>
              </a>

              {#if canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
                <Button
                  variant="outline"
                  size="sm"
                  class="flex-1 gap-1 text-xs h-8"
                  onclick={() => openLineupDrawer(song)}
                >
                  <Users class="w-3 h-3" />
                  <span>Lineup</span>
                </Button>

                {#if song.stage === 'draft'}
                  <Button
                    variant="secondary"
                    size="sm"
                    class="flex-1 text-xs h-8 border border-border/70 font-semibold"
                    onclick={() => advanceStatus(song, 'in_practice')}
                  >
                    <span>Start Practice</span>
                  </Button>
                {:else if song.stage === 'in_practice'}
                  <Button
                    size="sm"
                    class="flex-1 text-xs h-8 bg-primary hover:bg-primary/90 text-primary-foreground font-semibold"
                    onclick={() => advanceStatus(song, 'ready_for_qc')}
                  >
                    <span>Submit QC</span>
                  </Button>
                {/if}
              {/if}

              {#if song.stage === 'ready_for_qc' && canAuditSongScoped(userRole, false, song.qcReviewer === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
                <Button
                  size="sm"
                  class="flex-1 text-xs h-8 bg-primary hover:bg-primary/90 text-primary-foreground gap-1 font-semibold"
                  onclick={() => openQcDrawer(song)}
                >
                  <UserCheck class="w-3 h-3" />
                  <span>Audit QC</span>
                </Button>
              {/if}

              {#if song.stage === 'qc_approved' && canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
                <Button
                  size="sm"
                  class="flex-1 text-xs h-8 bg-emerald-600 hover:bg-emerald-700 text-white gap-1"
                  onclick={() => advanceStatus(song, 'stage_ready')}
                >
                  <CircleCheck class="w-3 h-3" />
                  <span>Promote</span>
                </Button>
              {/if}
            </div>

            {#if song.stage === 'stage_ready'}
              <div class="inline-flex items-center justify-center gap-1.5 w-full py-1.5 text-xs font-bold text-emerald-600 bg-emerald-500/10 rounded-lg">
                <CircleCheck class="w-3.5 h-3.5 text-emerald-600" />
                <span>100% Stage Ready</span>
              </div>
            {/if}
          </div>
        </Card>
      {/each}
    </div>

  {:else if currentView === 'kanban'}
    <!-- VIEW 2: REFINED KANBAN BOARD -->
    <div class="overflow-x-auto pb-2">
      <div class="grid grid-cols-5 gap-3 min-w-[1100px]">
        <!-- Column 1: Draft -->
        <Card class="p-3 bg-slate-50 flex flex-col gap-3 min-h-[480px]">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-slate-600">{$tStore('show_mgmt.status.draft')}</span>
            <span class="text-xs font-bold bg-slate-200 text-slate-700 px-2 py-0.5 rounded-full">{filteredNumbers.filter((n) => n.stage === 'draft').length}</span>
          </div>
          <div class="flex flex-col gap-2.5">
            {#each filteredNumbers.filter((n) => n.stage === 'draft') as song (song.id)}
              <Card class="p-3 bg-white flex flex-col gap-1.5 shadow-sm">
                <h4 class="text-sm font-bold text-foreground m-0">{song.title}</h4>
                <div class="text-xs text-muted-foreground">Leader (PM): {song.pmName}</div>
                <div class="text-[11px] text-muted-foreground">{song.genre}</div>
                {#if canManageNumbers(userRole)}
                  <div class="flex gap-1.5 mt-2">
                    <Button variant="outline" size="sm" class="flex-1 text-[11px] h-7 px-2" onclick={() => openLineupDrawer(song)}>
                      <Users class="w-3 h-3 mr-1" /> Lineup
                    </Button>
                    <Button size="sm" class="flex-1 text-[11px] h-7 px-2 bg-blue-600 hover:bg-blue-700 text-white" onclick={() => advanceStatus(song, 'in_practice')}>
                      Start
                    </Button>
                  </div>
                {/if}
              </Card>
            {/each}
          </div>
        </Card>

        <!-- Column 2: In Practice -->
        <Card class="p-3 bg-slate-50 flex flex-col gap-3 min-h-[480px]">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-blue-600">{$tStore('studio_shows.kanban_practice')}</span>
            <span class="text-xs font-bold bg-blue-100 text-blue-700 px-2 py-0.5 rounded-full">{filteredNumbers.filter((n) => n.stage === 'in_practice').length}</span>
          </div>
          <div class="flex flex-col gap-2.5">
            {#each filteredNumbers.filter((n) => n.stage === 'in_practice') as song (song.id)}
              <Card class="p-3 bg-white flex flex-col gap-1.5 shadow-sm">
                <h4 class="text-sm font-bold text-foreground m-0">{song.title}</h4>
                <div class="text-xs text-muted-foreground">Leader (PM): {song.pmName}</div>
                <div class="text-[11px] text-muted-foreground">QC: {song.qcReviewer}</div>

                {#if song.qcNotes}
                  <div class="text-[11px] p-1.5 bg-orange-50 text-orange-800 rounded border border-orange-200 flex items-start gap-1">
                    <MessageSquare class="w-3 h-3 shrink-0 mt-0.5" /> {song.qcNotes}
                  </div>
                {/if}

                {#if canManageNumbers(userRole)}
                  <div class="flex gap-1.5 mt-2">
                    <Button variant="outline" size="sm" class="flex-1 text-[11px] h-7 px-2" onclick={() => openLineupDrawer(song)}>
                      Lineup
                    </Button>
                    <Button size="sm" class="flex-1 text-[11px] h-7 px-2 bg-primary text-primary-foreground" onclick={() => advanceStatus(song, 'ready_for_qc')}>
                      Submit QC
                    </Button>
                  </div>
                {/if}
              </Card>
            {/each}
          </div>
        </Card>

        <!-- Column 3: Ready for QC -->
        <Card class="p-3 bg-slate-50 flex flex-col gap-3 min-h-[480px]">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-primary">{$tStore('studio_shows.kanban_ready_qc')}</span>
            <span class="text-xs font-bold bg-orange-100 text-primary px-2 py-0.5 rounded-full">{filteredNumbers.filter((n) => n.stage === 'ready_for_qc').length}</span>
          </div>
          <div class="flex flex-col gap-2.5">
            {#each filteredNumbers.filter((n) => n.stage === 'ready_for_qc') as song (song.id)}
              <Card class="p-3 bg-white flex flex-col gap-1.5 shadow-sm">
                <h4 class="text-sm font-bold text-foreground m-0">{song.title}</h4>
                <div class="text-xs text-muted-foreground">Leader (PM): {song.pmName}</div>
                <div class="text-[11px] text-muted-foreground">QC: {song.qcReviewer}</div>
                <div class="mt-2">
                  <Button size="sm" class="w-full text-[11px] h-7 bg-orange-600 hover:bg-orange-700 text-white gap-1" onclick={() => openQcDrawer(song)}>
                    <UserCheck class="w-3 h-3" /> Audit & Submit QC
                  </Button>
                </div>
              </Card>
            {/each}
          </div>
        </Card>

        <!-- Column 4: QC Approved -->
        <Card class="p-3 bg-slate-50 flex flex-col gap-3 min-h-[480px]">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-indigo-600">{$tStore('studio_shows.kanban_qc_approved')}</span>
            <span class="text-xs font-bold bg-indigo-100 text-indigo-700 px-2 py-0.5 rounded-full">{filteredNumbers.filter((n) => n.stage === 'qc_approved').length}</span>
          </div>
          <div class="flex flex-col gap-2.5">
            {#each filteredNumbers.filter((n) => n.stage === 'qc_approved') as song (song.id)}
              <Card class="p-3 bg-white flex flex-col gap-1.5 shadow-sm">
                <h4 class="text-sm font-bold text-foreground m-0">{song.title}</h4>
                <div class="text-xs text-muted-foreground">Leader (PM): {song.pmName}</div>
                {#if song.qcNotes}
                  <div class="text-[11px] p-1.5 bg-slate-50 text-slate-700 rounded border border-slate-200">{song.qcNotes}</div>
                {/if}
                <div class="mt-2">
                  <Button size="sm" class="w-full text-[11px] h-7 bg-emerald-600 hover:bg-emerald-700 text-white" onclick={() => advanceStatus(song, 'stage_ready')}>
                    Promote to Ready
                  </Button>
                </div>
              </Card>
            {/each}
          </div>
        </Card>

        <!-- Column 5: Stage Ready -->
        <Card class="p-3 bg-slate-50 flex flex-col gap-3 min-h-[480px]">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-emerald-600">{$tStore('studio_shows.kanban_stage_ready')}</span>
            <span class="text-xs font-bold bg-emerald-100 text-emerald-700 px-2 py-0.5 rounded-full">{filteredNumbers.filter((n) => n.stage === 'stage_ready').length}</span>
          </div>
          <div class="flex flex-col gap-2.5">
            {#each filteredNumbers.filter((n) => n.stage === 'stage_ready') as song (song.id)}
              <Card class="p-3 bg-white flex flex-col gap-1.5 shadow-sm border-l-4 border-l-emerald-500">
                <h4 class="text-sm font-bold text-foreground m-0">{song.title}</h4>
                <div class="text-xs text-muted-foreground">Leader (PM): {song.pmName}</div>
                <div class="flex items-center justify-between mt-2 pt-2 border-t border-slate-100">
                  <Button variant="outline" size="sm" class="text-[11px] h-7 px-2" onclick={() => openLineupDrawer(song)}>
                    Lineup
                  </Button>
                  <div class="inline-flex items-center gap-1 text-xs font-bold text-emerald-600">
                    <CircleCheck class="w-3.5 h-3.5" /> Ready
                  </div>
                </div>
              </Card>
            {/each}
          </div>
        </Card>
      </div>
    </div>

  {:else if currentView === 'table'}
    <!-- VIEW 3: COMPACT PRODUCTION TABLE -->
    <Card class="p-0 overflow-hidden shadow-xs border border-border/80 rounded-2xl">
      <div class="overflow-x-auto">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead class="w-12 text-center text-xs">#</TableHead>
              <TableHead class="text-xs">Song Title</TableHead>
              <TableHead class="text-xs">Genre</TableHead>
              <TableHead class="text-xs">PM / Leader</TableHead>
              <TableHead class="text-xs">QC Reviewer</TableHead>
              <TableHead class="text-xs">Lineup Status</TableHead>
              <TableHead class="text-xs">Current Stage</TableHead>
              <TableHead class="text-right text-xs">Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each filteredNumbers as song, index (song.id)}
              <TableRow>
                <TableCell class="text-center font-bold text-xs text-muted-foreground">
                  #{index + 1}
                </TableCell>
                <TableCell>
                  <div class="font-bold text-sm text-foreground">{song.title}</div>
                  <div class="text-xs text-muted-foreground">{song.genre}</div>
                </TableCell>
                <TableCell class="text-xs text-muted-foreground">{song.genre}</TableCell>
                <TableCell class="text-xs font-semibold text-foreground">{song.pmName}</TableCell>
                <TableCell class="text-xs text-muted-foreground">{song.qcReviewer || '—'}</TableCell>
                <TableCell>
                  <div class="flex flex-wrap gap-1">
                    {#if song.lineup?.vocalLead}<span class="text-[10px] px-1.5 py-0.5 rounded bg-amber-500/10 text-amber-600">Vo: {song.lineup.vocalLead}</span>{/if}
                    {#if song.lineup?.guitarLead}<span class="text-[10px] px-1.5 py-0.5 rounded bg-primary/10 text-primary">Gu: {song.lineup.guitarLead}</span>{/if}
                    {#if song.lineup?.bass}<span class="text-[10px] px-1.5 py-0.5 rounded bg-emerald-500/10 text-emerald-600">Ba: {song.lineup.bass}</span>{/if}
                    {#if song.lineup?.drums}<span class="text-[10px] px-1.5 py-0.5 rounded bg-purple-500/10 text-purple-600">Dr: {song.lineup.drums}</span>{/if}
                    {#if song.lineup?.keys}<span class="text-[10px] px-1.5 py-0.5 rounded bg-cyan-500/10 text-cyan-600">Kb: {song.lineup.keys}</span>{/if}
                  </div>
                </TableCell>
                <TableCell>
                  <Badge variant="outline" class="{getStageBadgeVariant(song.stage)} text-[11px] font-semibold">
                    {getStageLabel(song.stage)}
                  </Badge>
                </TableCell>
                <TableCell class="text-right">
                  <div class="flex items-center justify-end gap-1.5">
                    <a
                      href="/studio/shows/{showId}/sprints?song={encodeURIComponent(song.title)}"
                      class="h-7 px-2.5 inline-flex items-center justify-center text-xs font-semibold rounded-lg bg-secondary text-secondary-foreground hover:bg-muted transition-colors"
                      title="View Rehearsals in Sprint Calendar"
                    >
                      <Clock class="w-3.5 h-3.5 mr-1" />
                      <span>Sprint</span>
                    </a>

                    <Button
                      variant="outline"
                      size="sm"
                      class="h-7 px-2"
                      onclick={() => openLineupDrawer(song)}
                      title="Assign Band Lineup"
                    >
                      <Users class="w-3.5 h-3.5" />
                    </Button>

                    {#if song.stage === 'draft'}
                      <Button
                        variant="secondary"
                        size="sm"
                        class="h-7 px-2.5 text-xs border border-border/70 font-semibold"
                        onclick={() => advanceStatus(song, 'in_practice')}
                      >
                        Start
                      </Button>
                    {:else if song.stage === 'in_practice'}
                      <Button
                        size="sm"
                        class="h-7 px-2.5 text-xs bg-primary hover:bg-primary/90 text-primary-foreground font-semibold"
                        onclick={() => advanceStatus(song, 'ready_for_qc')}
                      >
                        Submit QC
                      </Button>
                    {:else if song.stage === 'ready_for_qc'}
                      <Button
                        size="sm"
                        class="h-7 px-2.5 text-xs bg-primary hover:bg-primary/90 text-primary-foreground font-semibold"
                        onclick={() => openQcDrawer(song)}
                      >
                        Audit
                      </Button>
                    {:else if song.stage === 'qc_approved'}
                      <Button
                        size="sm"
                        class="h-7 px-2.5 text-xs bg-emerald-600 hover:bg-emerald-700 text-white"
                        onclick={() => advanceStatus(song, 'stage_ready')}
                      >
                        Promote
                      </Button>
                    {:else}
                      <CircleCheck class="w-4 h-4 text-emerald-600 inline-block ml-2" />
                    {/if}
                  </div>
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    </Card>
  {/if}
</div>

<!-- Modal: Add New Music Number -->
<Dialog bind:open={isAddModalOpen}>
  <DialogContent class="max-w-md">
    <DialogHeader>
      <DialogTitle>{$tStore('studio_shows.modal_add_title')}</DialogTitle>
      <DialogDescription>{$tStore('studio_shows.modal_add_desc')}</DialogDescription>
    </DialogHeader>

    <form onsubmit={handleCreateNumber} class="flex flex-col gap-4 mt-2">
      <div class="flex flex-col gap-1.5">
        <Label for="new-song-title">{$tStore('studio_shows.label_song_title')}</Label>
        <Input
          id="new-song-title"
          type="text"
          bind:value={newTitle}
          placeholder="e.g. Diễm Xưa, Đi Về Nhà..."
          required
        />
      </div>

      <div class="flex flex-col gap-1.5">
        <Label for="new-song-genre">{$tStore('studio_shows.label_genre')}</Label>
        <Input
          id="new-song-genre"
          type="text"
          bind:value={newGenre}
          placeholder="e.g. Pop Rock, Acoustic Ballad, Jazz Fusion..."
        />
      </div>

      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <Label for="new-song-pm">{$tStore('studio_shows.label_pm')}</Label>
          <select id="new-song-pm" bind:value={newPm} class="h-9 px-3 text-xs bg-background border border-input rounded-md outline-none">
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="new-song-qc">{$tStore('studio_shows.label_qc_reviewer')}</Label>
          <select id="new-song-qc" bind:value={newQcReviewer} class="h-9 px-3 text-xs bg-background border border-input rounded-md outline-none">
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>
      </div>

      <DialogFooter class="mt-4">
        <Button variant="outline" type="button" onclick={() => (isAddModalOpen = false)}>
          Cancel
        </Button>
        <Button type="submit">
          {$tStore('studio_shows.btn_create_number')}
        </Button>
      </DialogFooter>
    </form>
  </DialogContent>
</Dialog>

<!-- Modal: QC Verdict Drawer -->
<Dialog bind:open={isQcDrawerOpen}>
  <DialogContent class="sm:max-w-xl w-full p-0 overflow-hidden border border-border/80 shadow-2xl rounded-2xl bg-card">
    <!-- Header with Accent Gradient Pill -->
    <div class="p-5 sm:p-6 pb-4 border-b border-border/60 bg-gradient-to-b from-muted/40 to-card">
      <div class="flex items-center gap-2.5 mb-1.5">
        <div class="flex items-center justify-center w-8 h-8 rounded-xl bg-primary/10 text-primary border border-primary/20 shadow-xs shrink-0">
          <ShieldCheck class="w-4 h-4 text-primary" />
        </div>
        <div>
          <DialogTitle class="text-base font-bold text-foreground tracking-tight">
            {$tStore('studio_shows.qc_drawer_title')}
          </DialogTitle>
          <p class="text-xs text-muted-foreground mt-0.5">
            Stage Quality Assurance & Rehearsal Assessment
          </p>
        </div>
      </div>

      {#if activeSongForQc}
        <!-- Song & Reviewer Metadata Bento Strip -->
        <div class="mt-3.5 p-2.5 sm:p-3 rounded-xl bg-background/80 border border-border/70 flex items-center justify-between gap-3 text-xs shadow-xs">
          <div class="flex items-center gap-2 min-w-0 flex-1">
            <div class="w-2 h-2 rounded-full bg-primary shrink-0 animate-pulse"></div>
            <div class="truncate">
              <span class="text-muted-foreground font-medium">Song:</span>
              <span class="font-bold text-foreground ml-1">{activeSongForQc.title}</span>
              {#if activeSongForQc.genre}
                <span class="text-[11px] text-muted-foreground ml-1.5 px-1.5 py-0.5 rounded-md bg-muted font-normal inline-block">
                  {activeSongForQc.genre}
                </span>
              {/if}
            </div>
          </div>
          <div class="shrink-0 flex items-center gap-1.5 text-muted-foreground bg-muted/60 px-2.5 py-1 rounded-lg border border-border/50 text-[11px] sm:text-xs">
            <UserCheck class="w-3.5 h-3.5 text-primary" />
            <span class="font-semibold text-foreground">{activeSongForQc.qcReviewer}</span>
          </div>
        </div>
      {/if}
    </div>

    {#if activeSongForQc}
      <form onsubmit={handleQcSubmit} class="p-5 sm:p-6 pt-4 sm:pt-5 flex flex-col gap-4 sm:gap-5">
        <!-- Interactive Verdict Card Picker -->
        <div class="flex flex-col gap-2">
          <div class="flex items-center justify-between">
            <Label class="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {$tStore('studio_shows.qc_verdict')}
            </Label>
            <span class="text-[11px] font-medium text-muted-foreground">Select decision</span>
          </div>

          <div class="grid grid-cols-2 gap-3">
            <!-- Pass Option Card -->
            <button
              type="button"
              onclick={() => (qcVerdict = 'pass')}
              class="relative flex flex-col gap-2 p-3.5 rounded-xl border text-left transition-all duration-200 cursor-pointer {qcVerdict === 'pass'
                ? 'border-emerald-500/80 bg-emerald-500/10 shadow-sm ring-2 ring-emerald-500/20'
                : 'border-border/80 bg-card hover:bg-muted/40 hover:border-border'}"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center justify-center w-7 h-7 rounded-lg {qcVerdict === 'pass' ? 'bg-emerald-500 text-white' : 'bg-muted text-muted-foreground'} transition-colors">
                  <ThumbsUp class="w-3.5 h-3.5" />
                </div>
                {#if qcVerdict === 'pass'}
                  <span class="flex h-2 w-2 rounded-full bg-emerald-500"></span>
                {/if}
              </div>
              <div>
                <div class="text-xs font-bold {qcVerdict === 'pass' ? 'text-emerald-700 dark:text-emerald-400' : 'text-foreground'}">
                  {$tStore('studio_shows.qc_pass')}
                </div>
                <div class="text-[11px] text-muted-foreground mt-0.5 line-clamp-2">
                  Ready to advance to Stage Ready pipeline.
                </div>
              </div>
            </button>

            <!-- Revision Option Card -->
            <button
              type="button"
              onclick={() => (qcVerdict = 'revision')}
              class="relative flex flex-col gap-2 p-3.5 rounded-xl border text-left transition-all duration-200 cursor-pointer {qcVerdict === 'revision'
                ? 'border-amber-500/80 bg-amber-500/10 shadow-sm ring-2 ring-amber-500/20'
                : 'border-border/80 bg-card hover:bg-muted/40 hover:border-border'}"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center justify-center w-7 h-7 rounded-lg {qcVerdict === 'revision' ? 'bg-amber-500 text-white' : 'bg-muted text-muted-foreground'} transition-colors">
                  <RefreshCw class="w-3.5 h-3.5" />
                </div>
                {#if qcVerdict === 'revision'}
                  <span class="flex h-2 w-2 rounded-full bg-amber-500"></span>
                {/if}
              </div>
              <div>
                <div class="text-xs font-bold {qcVerdict === 'revision' ? 'text-amber-700 dark:text-amber-400' : 'text-foreground'}">
                  {$tStore('studio_shows.qc_revision')}
                </div>
                <div class="text-[11px] text-muted-foreground mt-0.5 line-clamp-2">
                  Return to practice band with audit feedback.
                </div>
              </div>
            </button>
          </div>
        </div>

        <!-- Notes / Feedback Field -->
        <div class="flex flex-col gap-1.5">
          <div class="flex items-center justify-between">
            <Label for="qc-notes" class="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {$tStore('studio_shows.qc_notes')}
            </Label>
            <span class="text-[11px] text-muted-foreground">Required</span>
          </div>
          <textarea
            id="qc-notes"
            bind:value={qcNotesInput}
            rows="3"
            placeholder="Provide specific notes on vocal intonation, rhythm tightness, instrument balance..."
            required
            class="w-full p-3 text-xs leading-relaxed bg-background border border-input rounded-xl outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary transition-all shadow-xs resize-none placeholder:text-muted-foreground/60"
          ></textarea>
        </div>

        <!-- Dialog Footer Actions -->
        <DialogFooter class="flex items-center justify-end gap-2.5 pt-2 border-t border-border/50 mt-1">
          <Button
            variant="ghost"
            type="button"
            class="h-9 px-4 text-xs font-semibold rounded-xl text-muted-foreground hover:text-foreground"
            onclick={() => (isQcDrawerOpen = false)}
          >
            Cancel
          </Button>
          <Button
            type="submit"
            class="h-9 px-5 text-xs font-bold rounded-xl gap-1.5 shadow-sm transition-all {qcVerdict === 'pass' ? 'bg-emerald-600 hover:bg-emerald-700 text-white' : 'bg-primary hover:bg-primary/90 text-primary-foreground'}"
          >
            {#if qcVerdict === 'pass'}
              <CircleCheck class="w-3.5 h-3.5" />
            {:else}
              <RefreshCw class="w-3.5 h-3.5" />
            {/if}
            {$tStore('studio_shows.btn_submit_qc')}
          </Button>
        </DialogFooter>
      </form>
    {/if}
  </DialogContent>
</Dialog>

<!-- Modal: Lineup Role Assignment Drawer -->
<Dialog bind:open={isLineupDrawerOpen}>
  <DialogContent class="max-w-md">
    <DialogHeader>
      <DialogTitle>Assign Band Lineup</DialogTitle>
      {#if activeSongForLineup}
        <DialogDescription>
          Song: <strong class="text-foreground">{activeSongForLineup.title}</strong> ({activeSongForLineup.genre})
        </DialogDescription>
      {/if}
    </DialogHeader>

    {#if activeSongForLineup}
      <form onsubmit={handleLineupSubmit} class="flex flex-col gap-3.5 mt-2">
        <div class="flex flex-col gap-1.5">
          <Label for="role-vocal">Vocal Lead</Label>
          <select id="role-vocal" bind:value={formVocalLead} class="h-9 px-3 text-xs bg-background border border-input rounded-md outline-none">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="role-guitar">Guitar Lead / Solo</Label>
          <select id="role-guitar" bind:value={formGuitarLead} class="h-9 px-3 text-xs bg-background border border-input rounded-md outline-none">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="role-bass">Bass Guitar</Label>
          <select id="role-bass" bind:value={formBass} class="h-9 px-3 text-xs bg-background border border-input rounded-md outline-none">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="role-drums">Drum Kit</Label>
          <select id="role-drums" bind:value={formDrums} class="h-9 px-3 text-xs bg-background border border-input rounded-md outline-none">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="role-keys">Keyboard / Synthesizer</Label>
          <select id="role-keys" bind:value={formKeys} class="h-9 px-3 text-xs bg-background border border-input rounded-md outline-none">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <DialogFooter class="mt-4">
          <Button variant="outline" type="button" onclick={() => (isLineupDrawerOpen = false)}>Cancel</Button>
          <Button type="submit">
            Save Lineup Allocation
          </Button>
        </DialogFooter>
      </form>
    {/if}
  </DialogContent>
</Dialog>
