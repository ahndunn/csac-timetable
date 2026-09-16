<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    Users,
    UserPlus,
    Search,
    Filter,
    LayoutGrid,
    Table as TableIcon,
    Music,
    MicVocal,
    Guitar,
    Disc3,
    Sparkles,
    CircleCheck,
    CircleAlert,
    TriangleAlert,
    Zap,
  } from '@lucide/svelte';

  import { page } from '$app/state';
  import { canManageShowRoster } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';
  import { api } from '$lib/api/client';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';
  import { Input } from '$lib/components/ui/input';
  import { auth } from '$lib/stores/auth.svelte';

  // Sub-components
  import RosterCard, { type ShowRosterMember, type WorkloadInfo } from './components/RosterCard.svelte';
  import RosterTable from './components/RosterTable.svelte';
  import RosterMemberModal, { type BandRoleDef } from './components/RosterMemberModal.svelte';
  import RosterRemoveModal from './components/RosterRemoveModal.svelte';

  let { data } = $props();

  const showId = $derived(page.params.id || '');
  const activeUser = $derived(auth.user || page.data?.user || null);
  const userRole = $derived((activeUser?.role || 'member') as UserRole);

  const AVAILABLE_BAND_ROLES: BandRoleDef[] = [
    { id: 'vocals', key: 'show_mgmt.roles.vocal_lead', icon: MicVocal },
    { id: 'lead_guitar', key: 'show_mgmt.roles.guitar_lead', icon: Guitar },
    { id: 'rhythm_guitar', key: 'show_mgmt.roles.guitar_rhythm', icon: Guitar },
    { id: 'bass', key: 'show_mgmt.roles.bass', icon: Disc3 },
    { id: 'drums', key: 'show_mgmt.roles.drums', icon: Disc3 },
    { id: 'keys', key: 'show_mgmt.roles.keys', icon: Music },
    { id: 'percussion', key: 'show_mgmt.roles.percussion', icon: Music },
    { id: 'backing_vocals', key: 'show_mgmt.roles.vocal_harmony', icon: MicVocal },
  ];

  let roster = $state<ShowRosterMember[]>([]);

  $effect(() => {
    roster = data?.roster || [];
  });

  let currentView = $state<'grid' | 'table'>('grid');
  let searchQuery = $state('');
  let instrumentFilter = $state('all');
  let workloadFilter = $state('all');

  // Modal States
  let isAddEditModalOpen = $state(false);
  let editingMember = $state<ShowRosterMember | null>(null);

  let isRemoveModalOpen = $state(false);
  let memberToRemove = $state<ShowRosterMember | null>(null);

  function getWorkloadInfo(member: ShowRosterMember): WorkloadInfo {
    const songCount = member?.assignedSongs?.length || 0;
    const hours = member?.practiceHours || 0;

    if (songCount >= 5 || hours >= 20 || member?.workloadFlag === 'fatigued') {
      return {
        variant: 'bg-red-500/10 text-red-600 border-red-200 dark:border-red-800',
        labelKey: 'show_mgmt.roster_page.workload_fatigued',
        icon: TriangleAlert,
        descKey: 'show_mgmt.roster_page.workload_fatigued_desc',
      };
    } else if (songCount >= 3 || hours >= 12 || member?.workloadFlag === 'moderate') {
      return {
        variant: 'bg-amber-500/10 text-amber-600 border-amber-200 dark:border-amber-800',
        labelKey: 'show_mgmt.roster_page.workload_moderate',
        icon: CircleAlert,
        descKey: 'show_mgmt.roster_page.workload_moderate_desc',
      };
    } else {
      return {
        variant: 'bg-emerald-500/10 text-emerald-600 border-emerald-200 dark:border-emerald-800',
        labelKey: 'show_mgmt.roster_page.workload_optimal',
        icon: CircleCheck,
        descKey: 'show_mgmt.roster_page.workload_optimal_desc',
      };
    }
  }

  const workloadSummary = $derived({
    total: roster.length,
    optimal: roster.filter((m) => getWorkloadInfo(m).labelKey === 'show_mgmt.roster_page.workload_optimal').length,
    moderate: roster.filter((m) => getWorkloadInfo(m).labelKey === 'show_mgmt.roster_page.workload_moderate').length,
    fatigued: roster.filter((m) => getWorkloadInfo(m).labelKey === 'show_mgmt.roster_page.workload_fatigued').length,
  });

  const filteredRoster = $derived(
    roster.filter((m) => {
      if (!m) return false;
      const secondary = m.secondaryInstruments || [];
      const matchInst =
        instrumentFilter === 'all' ||
        m.primaryInstrument === instrumentFilter ||
        secondary.includes(instrumentFilter);

      const workload = getWorkloadInfo(m);
      const matchWorkload =
        workloadFilter === 'all' ||
        (workloadFilter === 'optimal' && workload.labelKey === 'show_mgmt.roster_page.workload_optimal') ||
        (workloadFilter === 'moderate' && workload.labelKey === 'show_mgmt.roster_page.workload_moderate') ||
        (workloadFilter === 'fatigued' && workload.labelKey === 'show_mgmt.roster_page.workload_fatigued');

      if (!matchInst || !matchWorkload) return false;
      if (!searchQuery.trim()) return true;

      const q = searchQuery.toLowerCase().trim();
      const matchName = (m.fullName || '').toLowerCase().includes(q);
      const matchEmail = (m.email || '').toLowerCase().includes(q);
      const matchRole = (m.showRole || '').toLowerCase().includes(q);
      const assigned = m.assignedSongs || [];
      const matchSongs = assigned.some((s) => (s || '').toLowerCase().includes(q));

      return matchName || matchEmail || matchRole || matchSongs;
    })
  );

  function openAddModal() {
    editingMember = null;
    isAddEditModalOpen = true;
  }

  function openEditModal(member: ShowRosterMember) {
    editingMember = member;
    isAddEditModalOpen = true;
  }

  function promptRemoveMember(member: ShowRosterMember) {
    memberToRemove = member;
    isRemoveModalOpen = true;
  }

  async function handleSaveMember(payload: {
    fullName: string;
    email: string;
    phone?: string;
    showRole: ShowRosterMember['showRole'];
    primaryInstrument: string;
    secondaryInstruments: string[];
    practiceHours: number;
  }) {
    if (editingMember) {
      try {
        const updated = await api.shows.saveRosterMember(showId, {
          id: editingMember.id,
          ...payload,
        });
        roster = roster.map((m) => (m.id === updated.id ? updated : m));
      } catch {
        roster = roster.map((m) =>
          m.id === editingMember?.id
            ? { ...m, ...payload }
            : m
        );
      }
    } else {
      try {
        const created = await api.shows.saveRosterMember(showId, payload);
        roster = [created, ...roster];
      } catch {
        const newMember: ShowRosterMember = {
          id: `mem-${Date.now()}`,
          ...payload,
          assignedSongs: [],
          attendanceRate: 100,
        };
        roster = [newMember, ...roster];
      }
    }
  }

  async function confirmRemoveMember(memberId: string) {
    try {
      await api.shows.deleteRosterMember(showId, memberId);
      roster = roster.filter((m) => m.id !== memberId);
    } catch {
      roster = roster.filter((m) => m.id !== memberId);
    }
  }
</script>

<div class="flex flex-col gap-5">
  <!-- Header Bar -->
  <Card class="p-5 flex flex-col md:flex-row items-start md:items-center justify-between gap-4 shadow-sm">
    <div class="flex flex-col gap-1.5">
      <div class="flex items-center gap-2">
        <Badge variant="outline" class="bg-primary/10 text-primary border-primary/20 gap-1 font-bold">
          <Users class="w-3 h-3 text-primary" />
          <span>{$tStore('show_mgmt.roster_page.title')}</span>
        </Badge>
        <span class="text-xs text-muted-foreground font-semibold">
          {$tStore('show_mgmt.roster_page.total_members', { count: roster.length })}
        </span>
      </div>
      <h2 class="text-xl font-extrabold text-foreground tracking-tight m-0">
        {$tStore('show_mgmt.roster_page.title')}
      </h2>
      <p class="text-xs text-muted-foreground m-0">
        {$tStore('show_mgmt.roster_page.subtitle')}
      </p>
    </div>

    <div class="flex items-center gap-2 flex-wrap">
      <!-- View Switcher -->
      <div class="flex items-center bg-muted/60 border border-border/50 rounded-lg p-0.5 gap-0.5">
        <button
          type="button"
          class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold cursor-pointer transition-all {currentView === 'grid' ? 'bg-card text-primary shadow-xs font-bold border border-border/40' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (currentView = 'grid')}
        >
          <LayoutGrid class="w-3.5 h-3.5" />
          <span>{$tStore('studio_shows.view_grid')}</span>
        </button>
        <button
          type="button"
          class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold cursor-pointer transition-all {currentView === 'table' ? 'bg-card text-primary shadow-xs font-bold border border-border/40' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (currentView = 'table')}
        >
          <TableIcon class="w-3.5 h-3.5" />
          <span>{$tStore('studio_shows.view_table')}</span>
        </button>
      </div>

      {#if canManageShowRoster(userRole)}
        <Button size="sm" class="gap-1.5" onclick={openAddModal}>
          <UserPlus class="w-4 h-4" />
          <span>{$tStore('show_mgmt.roster_page.btn_add_member')}</span>
        </Button>
      {/if}
    </div>
  </Card>

  <!-- Workload Health Bar (Bento Metrics) -->
  <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
    <button
      type="button"
      class="p-4 rounded-xl border flex items-center justify-between text-left transition-all cursor-pointer {workloadFilter === 'optimal' ? 'border-emerald-500 bg-emerald-500/10 shadow-xs' : 'border-border bg-card hover:bg-muted/40'}"
      onclick={() => (workloadFilter = workloadFilter === 'optimal' ? 'all' : 'optimal')}
    >
      <div class="flex items-center gap-3">
        <div class="flex h-9 w-9 items-center justify-center rounded-xl bg-emerald-500/15 text-emerald-600">
          <CircleCheck class="w-5 h-5 text-emerald-600" />
        </div>
        <div>
          <div class="text-xs font-bold text-foreground">{$tStore('show_mgmt.roster_page.workload_optimal')}</div>
          <div class="text-[11px] text-muted-foreground">1–2 songs, balanced load</div>
        </div>
      </div>
      <span class="text-xl font-black text-emerald-600">{workloadSummary.optimal}</span>
    </button>

    <button
      type="button"
      class="p-4 rounded-xl border flex items-center justify-between text-left transition-all cursor-pointer {workloadFilter === 'moderate' ? 'border-amber-500 bg-amber-500/10 shadow-xs' : 'border-border bg-card hover:bg-muted/40'}"
      onclick={() => (workloadFilter = workloadFilter === 'moderate' ? 'all' : 'moderate')}
    >
      <div class="flex items-center gap-3">
        <div class="flex h-9 w-9 items-center justify-center rounded-xl bg-amber-500/15 text-amber-600">
          <CircleAlert class="w-5 h-5 text-amber-600" />
        </div>
        <div>
          <div class="text-xs font-bold text-foreground">{$tStore('show_mgmt.roster_page.workload_moderate')}</div>
          <div class="text-[11px] text-muted-foreground">3–4 songs, monitor sprints</div>
        </div>
      </div>
      <span class="text-xl font-black text-amber-600">{workloadSummary.moderate}</span>
    </button>

    <button
      type="button"
      class="p-4 rounded-xl border flex items-center justify-between text-left transition-all cursor-pointer {workloadFilter === 'fatigued' ? 'border-red-500 bg-red-500/10 shadow-xs' : 'border-border bg-card hover:bg-muted/40'}"
      onclick={() => (workloadFilter = workloadFilter === 'fatigued' ? 'all' : 'fatigued')}
    >
      <div class="flex items-center gap-3">
        <div class="flex h-9 w-9 items-center justify-center rounded-xl bg-red-500/15 text-red-600">
          <TriangleAlert class="w-5 h-5 text-red-600" />
        </div>
        <div>
          <div class="text-xs font-bold text-foreground">{$tStore('show_mgmt.roster_page.workload_fatigued')}</div>
          <div class="text-[11px] text-muted-foreground">&ge; 5 songs or &ge; 20h practice</div>
        </div>
      </div>
      <span class="text-xl font-black text-red-600">{workloadSummary.fatigued}</span>
    </button>
  </div>

  <!-- Search & Filter Controls -->
  <div class="flex flex-col sm:flex-row items-center justify-between gap-3">
    <div class="relative w-full sm:w-80">
      <Search class="w-4 h-4 text-muted-foreground absolute left-3 top-1/2 -translate-y-1/2 pointer-events-none" />
      <Input
        type="text"
        placeholder={$tStore('show_mgmt.roster_page.search_placeholder')}
        bind:value={searchQuery}
        class="pl-9 text-xs h-9 bg-card"
      />
    </div>

    <div class="flex items-center gap-2.5 w-full sm:w-auto">
      <div class="flex items-center gap-1.5 bg-card border border-border rounded-lg px-2.5 py-1 text-xs">
        <Filter class="w-3.5 h-3.5 text-muted-foreground" />
        <span class="text-muted-foreground font-medium">Instrument:</span>
        <select
          bind:value={instrumentFilter}
          class="bg-transparent border-0 text-xs font-semibold text-foreground outline-none cursor-pointer"
        >
          <option value="all">{$tStore('show_mgmt.roster_page.filter_instrument_all')}</option>
          {#each AVAILABLE_BAND_ROLES as role}
            <option value={role.id}>{$tStore(role.key)}</option>
          {/each}
        </select>
      </div>

      {#if instrumentFilter !== 'all' || workloadFilter !== 'all' || searchQuery}
        <Button
          variant="ghost"
          size="sm"
          class="h-8 px-2 text-xs text-muted-foreground hover:text-foreground"
          onclick={() => {
            instrumentFilter = 'all';
            workloadFilter = 'all';
            searchQuery = '';
          }}
        >
          {$tStore('show_mgmt.roster_page.btn_reset_filters')}
        </Button>
      {/if}
    </div>
  </div>

  {#if filteredRoster.length === 0}
    <Card class="p-12 text-center flex flex-col items-center justify-center gap-3">
      <div class="w-12 h-12 rounded-full bg-muted flex items-center justify-center text-muted-foreground">
        <Users class="w-6 h-6" />
      </div>
      <div>
        <h3 class="text-base font-bold text-foreground m-0">{$tStore('show_mgmt.roster_page.empty_title')}</h3>
        <p class="text-xs text-muted-foreground m-0 mt-1 max-w-sm">
          {$tStore('show_mgmt.roster_page.empty_desc')}
        </p>
      </div>
    </Card>
  {:else if currentView === 'grid'}
    <!-- Bento Grid View -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each filteredRoster as member (member.id)}
        <RosterCard
          {member}
          {showId}
          {userRole}
          workload={getWorkloadInfo(member)}
          onEdit={openEditModal}
          onRemove={promptRemoveMember}
        />
      {/each}
    </div>
  {:else}
    <!-- Tabular Roster View -->
    <RosterTable
      roster={filteredRoster}
      {userRole}
      {getWorkloadInfo}
      onEdit={openEditModal}
      onRemove={promptRemoveMember}
    />
  {/if}
</div>

<!-- Add / Edit Member Modal -->
<RosterMemberModal
  bind:open={isAddEditModalOpen}
  member={editingMember}
  {userRole}
  availableBandRoles={AVAILABLE_BAND_ROLES}
  onOpenChange={(open) => (isAddEditModalOpen = open)}
  onSave={handleSaveMember}
/>

<!-- Remove Member Confirmation Modal -->
<RosterRemoveModal
  bind:open={isRemoveModalOpen}
  member={memberToRemove}
  onOpenChange={(open) => (isRemoveModalOpen = open)}
  onConfirm={confirmRemoveMember}
/>
