<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    Users,
    Shield,
    Music,
    Mail,
    Phone,
    TriangleAlert,
    Plus,
    Search,
    SlidersHorizontal,
    LayoutGrid,
    Table as TableIcon,
    CircleCheck,
    Activity,
    Clock,
    Zap,
    MicVocal,
    Guitar,
    Disc,
    SlidersVertical,
    Trash2,
    PenLine,
    X,
  } from '@lucide/svelte';
  import { page } from '$app/state';
  import type { BandRole, ShowRole, ShowRosterMember, UserRole } from '$lib/types/timetable';
  import { canManageShowRoster, canEditPerformerProfile, canManageShowScoped } from '$lib/auth';
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

  let { data } = $props();

  const showId = $derived(page.params.id || 'show-2026-annual');
  const userRole = $derived((page.data?.user?.role || 'admin') as UserRole);

  let roster = $state<ShowRosterMember[]>([]);

  $effect(() => {
    roster = data?.roster || [];
  });

  let viewMode = $state<'cards' | 'matrix'>('cards');
  let searchQuery = $state('');
  let selectedFilter = $state<string>('all');

  let isAddEditModalOpen = $state(false);
  let isRemoveModalOpen = $state(false);
  let editingMember = $state<ShowRosterMember | null>(null);
  let memberToRemove = $state<ShowRosterMember | null>(null);

  let formFullName = $state('');
  let formEmail = $state('');
  let formPhone = $state('');
  let formShowRole = $state<ShowRole>('Performer');
  let formPrimaryInst = $state<BandRole>('vocal_lead');
  let formSecondaryInst = $state<BandRole[]>([]);
  let formPracticeHours = $state(4);

  const AVAILABLE_BAND_ROLES: { id: BandRole; key: string; icon: any; category: 'vocals' | 'strings' | 'rhythm' | 'keys_tech' }[] = [
    { id: 'vocal_lead', key: 'show_mgmt.roles.vocal_lead', icon: MicVocal, category: 'vocals' },
    { id: 'vocal_harmony', key: 'show_mgmt.roles.vocal_harmony', icon: MicVocal, category: 'vocals' },
    { id: 'guitar_lead', key: 'show_mgmt.roles.guitar_lead', icon: Guitar, category: 'strings' },
    { id: 'guitar_rhythm', key: 'show_mgmt.roles.guitar_rhythm', icon: Guitar, category: 'strings' },
    { id: 'bass', key: 'show_mgmt.roles.bass', icon: Guitar, category: 'strings' },
    { id: 'keys', key: 'show_mgmt.roles.keys', icon: SlidersVertical, category: 'keys_tech' },
    { id: 'drums', key: 'show_mgmt.roles.drums', icon: Disc, category: 'rhythm' },
    { id: 'percussion', key: 'show_mgmt.roles.percussion', icon: Disc, category: 'rhythm' },
    { id: 'sound_tech', key: 'show_mgmt.roles.sound_tech', icon: SlidersHorizontal, category: 'keys_tech' },
  ];

  let totalMembers = $derived(roster.length);
  let dmCount = $derived(roster.filter((m) => m.showRole === 'DM').length);
  let pmCount = $derived(roster.filter((m) => m.showRole === 'PM').length);
  let qcCount = $derived(roster.filter((m) => m.showRole === 'QC').length);
  let performerCount = $derived(roster.filter((m) => m.showRole === 'Performer').length);
  let totalPracticeHours = $derived(roster.reduce((sum, m) => sum + m.totalPracticeHours, 0));

  let fatigueCount = $derived(roster.filter((m) => m.workloadStatus === 'fatigued').length);
  let moderateCount = $derived(roster.filter((m) => m.workloadStatus === 'moderate').length);
  let optimalCount = $derived(roster.filter((m) => m.workloadStatus === 'optimal').length);

  let vocalsCount = $derived(
    roster.filter((m) => m.primaryInstrument === 'vocal_lead' || m.primaryInstrument === 'vocal_harmony').length
  );
  let stringsCount = $derived(
    roster.filter((m) => ['guitar_lead', 'guitar_rhythm', 'bass'].includes(m.primaryInstrument)).length
  );
  let rhythmCount = $derived(
    roster.filter((m) => ['drums', 'percussion'].includes(m.primaryInstrument)).length
  );
  let keysTechCount = $derived(
    roster.filter((m) => ['keys', 'sound_tech'].includes(m.primaryInstrument)).length
  );

  let filteredRoster = $derived(
    roster.filter((m) => {
      const q = searchQuery.toLowerCase().trim();
      const matchQuery =
        !q ||
        m.fullName.toLowerCase().includes(q) ||
        m.email.toLowerCase().includes(q) ||
        m.showRole.toLowerCase().includes(q) ||
        m.primaryInstrument.toLowerCase().includes(q);

      if (!matchQuery) return false;

      if (selectedFilter === 'all') return true;
      if (selectedFilter === 'leadership') return ['DM', 'PM', 'QC'].includes(m.showRole);
      if (selectedFilter === 'performers') return m.showRole === 'Performer';
      if (selectedFilter === 'vocals') return ['vocal_lead', 'vocal_harmony'].includes(m.primaryInstrument);
      if (selectedFilter === 'strings') return ['guitar_lead', 'guitar_rhythm', 'bass'].includes(m.primaryInstrument);
      if (selectedFilter === 'rhythm') return ['drums', 'percussion'].includes(m.primaryInstrument);
      if (selectedFilter === 'keys_tech') return ['keys', 'sound_tech'].includes(m.primaryInstrument);
      return true;
    })
  );

  function calculateWorkload(songCount: number): 'optimal' | 'moderate' | 'fatigued' {
    if (songCount >= 5) return 'fatigued';
    if (songCount >= 3) return 'moderate';
    return 'optimal';
  }

  function openAddModal() {
    editingMember = null;
    formFullName = '';
    formEmail = '';
    formPhone = '';
    formShowRole = 'Performer';
    formPrimaryInst = 'vocal_lead';
    formSecondaryInst = [];
    formPracticeHours = 4;
    isAddEditModalOpen = true;
  }

  function openEditModal(member: ShowRosterMember) {
    editingMember = member;
    formFullName = member.fullName;
    formEmail = member.email;
    formPhone = member.phone || '';
    formShowRole = member.showRole;
    formPrimaryInst = member.primaryInstrument;
    formSecondaryInst = [...member.secondaryInstruments];
    formPracticeHours = member.totalPracticeHours;
    isAddEditModalOpen = true;
  }

  async function handleSaveMember(e: Event) {
    e.preventDefault();
    if (!formFullName.trim()) return;

    if (editingMember) {
      const updatedMember: ShowRosterMember = {
        ...editingMember,
        fullName: formFullName.trim(),
        email: formEmail.trim(),
        phone: formPhone.trim(),
        showRole: formShowRole,
        isDM: formShowRole === 'DM',
        primaryInstrument: formPrimaryInst,
        secondaryInstruments: formSecondaryInst,
        totalPracticeHours: Number(formPracticeHours) || editingMember.totalPracticeHours,
      };

      roster = roster.map((m) => (m.id === editingMember?.id ? updatedMember : m));
      isAddEditModalOpen = false;

      try {
        await api.shows.saveRosterMember(showId, {
          id: editingMember.id,
          fullName: updatedMember.fullName,
          email: updatedMember.email,
          phone: updatedMember.phone,
          showRole: updatedMember.showRole,
          primaryInstrument: updatedMember.primaryInstrument,
          secondaryInstruments: updatedMember.secondaryInstruments,
          practiceHours: updatedMember.totalPracticeHours,
        });
      } catch (err) {
        console.error('Failed to update roster member in backend:', err);
      }
    } else {
      const newMember: ShowRosterMember = {
        id: `mem-${Date.now()}`,
        userId: `u-${Date.now()}`,
        fullName: formFullName.trim(),
        email: formEmail.trim() || `${formFullName.toLowerCase().replace(/\s+/g, '')}@csac.local`,
        phone: formPhone.trim() || '+84 900 000 000',
        showRole: formShowRole,
        isDM: formShowRole === 'DM',
        primaryInstrument: formPrimaryInst,
        secondaryInstruments: formSecondaryInst,
        assignedSongCount: 1,
        assignedSongTitles: ['Intro / Warmup Rehearsal'],
        totalPracticeHours: Number(formPracticeHours) || 4,
        workloadStatus: calculateWorkload(1),
        attendanceRate: 100,
        joinedAt: new Date().toISOString().split('T')[0],
      };
      roster = [newMember, ...roster];
      isAddEditModalOpen = false;

      try {
        await api.shows.saveRosterMember(showId, {
          fullName: newMember.fullName,
          email: newMember.email,
          phone: newMember.phone,
          showRole: newMember.showRole,
          primaryInstrument: newMember.primaryInstrument,
          secondaryInstruments: newMember.secondaryInstruments,
          practiceHours: newMember.totalPracticeHours,
        });
      } catch (err) {
        console.error('Failed to create roster member in backend:', err);
      }
    }
  }

  function promptRemoveMember(member: ShowRosterMember) {
    memberToRemove = member;
    isRemoveModalOpen = true;
  }

  async function confirmRemoveMember() {
    if (memberToRemove) {
      const toRemoveId = memberToRemove.id;
      roster = roster.filter((m) => m.id !== toRemoveId);
      memberToRemove = null;
      isRemoveModalOpen = false;

      try {
        await api.shows.deleteRosterMember(showId, toRemoveId);
      } catch (err) {
        console.error('Failed to delete member from backend:', err);
      }
    }
  }

  function toggleSecondaryInstrument(inst: BandRole) {
    if (formSecondaryInst.includes(inst)) {
      formSecondaryInst = formSecondaryInst.filter((i) => i !== inst);
    } else {
      formSecondaryInst = [...formSecondaryInst, inst];
    }
  }

  function getWorkloadBadge(status: 'optimal' | 'moderate' | 'fatigued') {
    switch (status) {
      case 'fatigued':
        return { labelKey: 'show_mgmt.workload_fatigued', variant: 'bg-red-50 text-red-600 border-red-200', icon: TriangleAlert };
      case 'moderate':
        return { labelKey: 'show_mgmt.workload_moderate', variant: 'bg-amber-50 text-amber-600 border-amber-200', icon: Activity };
      default:
        return { labelKey: 'show_mgmt.workload_optimal', variant: 'bg-emerald-50 text-emerald-600 border-emerald-200', icon: CircleCheck };
    }
  }
</script>

<div class="flex flex-col gap-4 w-full">
  <!-- Top Header & Role Simulation Bar -->
  <Card class="p-5 flex flex-col md:flex-row items-start md:items-center justify-between gap-4 shadow-sm">
    <div class="flex flex-col gap-1">
      <div class="flex items-center gap-3 flex-wrap">
        <h2 class="text-xl font-extrabold text-foreground tracking-tight m-0">{$tStore('show_mgmt.roster_page.title')}</h2>
        <Badge variant="secondary" class="font-bold text-xs">{$tStore('show_mgmt.roster_page.stat_total_members')}: {totalMembers}</Badge>
      </div>
      <p class="text-xs text-muted-foreground m-0">{$tStore('show_mgmt.roster_page.subtitle')}</p>
    </div>

    <div class="flex items-center gap-3 flex-wrap">
      <div class="flex items-center gap-2 bg-slate-50 px-3 py-1.5 rounded-lg border border-slate-200/80 text-xs">
        <span class="text-muted-foreground font-semibold">Governance Scope:</span>
        <span class="inline-flex items-center gap-1 font-bold text-primary">
          {#if userRole === 'admin' || userRole === 'moderator'}
            <Shield class="w-3.5 h-3.5 text-primary" />
            <span>Global {userRole.toUpperCase()}</span>
          {:else}
            <Users class="w-3.5 h-3.5 text-blue-600" />
            <span>Show DM Roster Lead</span>
          {/if}
        </span>
      </div>

      {#if canManageShowScoped(userRole, true)}
        <Button size="sm" class="gap-1.5" onclick={openAddModal}>
          <Plus class="w-4 h-4" />
          <span>{$tStore('show_mgmt.roster_modal.btn_add_member')}</span>
        </Button>
      {/if}
    </div>
  </Card>

  <!-- Bento 4-Box Monitor Matrix & Performance Metrics -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
    <!-- Box 1: Roster Headcount & Leadership Breakdown -->
    <Card class="p-4 flex flex-col justify-between gap-3 border border-border/90 hover:border-primary/50 transition-all">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-muted text-muted-foreground flex items-center justify-center shrink-0">
          <Users class="w-5 h-5" />
        </div>
        <div>
          <div class="text-3xl font-black text-foreground leading-none">{totalMembers}</div>
          <div class="text-xs text-muted-foreground mt-0.5 font-medium">{$tStore('show_mgmt.roster_page.stat_total_members')}</div>
        </div>
      </div>
      <div class="flex flex-wrap gap-1.5 pt-2 border-t border-border">
        <span class="text-[11px] font-bold px-2 py-0.5 rounded bg-primary/10 text-primary">DM: {dmCount}</span>
        <span class="text-[11px] font-bold px-2 py-0.5 rounded bg-primary/15 text-primary">PM: {pmCount}</span>
        <span class="text-[11px] font-bold px-2 py-0.5 rounded bg-primary/10 text-primary">QC: {qcCount}</span>
        <span class="text-[11px] font-bold px-2 py-0.5 rounded bg-muted text-foreground">Cast: {performerCount}</span>
      </div>
    </Card>

    <!-- Box 2: Band Sections & Performance Role Coverage -->
    <Card class="p-4 flex flex-col justify-between gap-3 border border-border/90 hover:border-primary/50 transition-all">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-muted text-muted-foreground flex items-center justify-center shrink-0">
          <Music class="w-5 h-5" />
        </div>
        <div>
          <div class="text-3xl font-black text-foreground leading-none">4 / 4</div>
          <div class="text-xs text-muted-foreground mt-0.5 font-medium">{$tStore('show_mgmt.roster_page.stat_coverage')}</div>
        </div>
      </div>
      <div class="grid grid-cols-2 gap-1.5 pt-2 border-t border-border text-[11px]">
        <div class="flex items-center gap-1"><MicVocal class="w-3 h-3 text-muted-foreground" /><span>Vocals: <strong>{vocalsCount}</strong></span></div>
        <div class="flex items-center gap-1"><Guitar class="w-3 h-3 text-muted-foreground" /><span>Strings: <strong>{stringsCount}</strong></span></div>
        <div class="flex items-center gap-1"><Disc class="w-3 h-3 text-emerald-600" /><span>Rhythm: <strong>{rhythmCount}</strong></span></div>
        <div class="flex items-center gap-1"><SlidersVertical class="w-3 h-3 text-muted-foreground" /><span>Keys: <strong>{keysTechCount}</strong></span></div>
      </div>
    </Card>

    <!-- Box 3: Workload & Fatigue Risk Distribution -->
    <Card class="p-4 flex flex-col justify-between gap-3 border border-border/90 hover:border-primary/50 transition-all">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-destructive/10 flex items-center justify-center shrink-0">
          <Activity class="w-5 h-5 text-destructive" />
        </div>
        <div>
          <div class="text-3xl font-black leading-none {fatigueCount > 0 ? 'text-destructive' : 'text-emerald-600'}">
            {fatigueCount}
          </div>
          <div class="text-xs text-muted-foreground mt-0.5 font-medium">Fatigue Risks (5+ Songs)</div>
        </div>
      </div>
      <div class="flex items-center gap-1.5 pt-2 border-t border-border">
        <div class="flex-1 text-center py-0.5 bg-emerald-500/10 text-emerald-700 rounded text-[10px] font-bold">
          <div>{optimalCount}</div>
          <div class="text-[9px] font-normal text-muted-foreground">Optimal (1-2)</div>
        </div>
        <div class="flex-1 text-center py-0.5 bg-amber-500/10 text-amber-700 rounded text-[10px] font-bold">
          <div>{moderateCount}</div>
          <div class="text-[9px] font-normal text-muted-foreground">Busy (3-4)</div>
        </div>
        <div class="flex-1 text-center py-0.5 {fatigueCount > 0 ? 'bg-destructive/10 text-destructive' : 'bg-muted text-muted-foreground'} rounded text-[10px] font-bold">
          <div>{fatigueCount}</div>
          <div class="text-[9px] font-normal text-muted-foreground">Fatigued</div>
        </div>
      </div>
    </Card>

    <!-- Box 4: Total Show Practice Effort & Rehearsal Hours -->
    <Card class="p-4 flex flex-col justify-between gap-3 border border-border/90 hover:border-primary/50 transition-all">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-muted text-muted-foreground flex items-center justify-center shrink-0">
          <Clock class="w-5 h-5" />
        </div>
        <div>
          <div class="text-3xl font-black text-foreground leading-none">{totalPracticeHours}<span class="text-xl font-bold text-muted-foreground">h</span></div>
          <div class="text-xs text-muted-foreground mt-0.5 font-medium">{$tStore('show_mgmt.roster_page.stat_total_practice')}</div>
        </div>
      </div>
      <div class="flex items-center gap-1.5 pt-2 border-t border-border text-xs text-muted-foreground">
        <Zap class="w-3.5 h-3.5 text-muted-foreground" />
        <span>Avg {Math.round(totalPracticeHours / (totalMembers || 1))}h effort per performer</span>
      </div>
    </Card>
  </div>

  <!-- Search, Filter Toolbar & View Mode Switcher -->
  <Card class="p-3 flex items-center justify-between flex-wrap gap-3 shadow-sm">
    <div class="relative flex-1 min-w-[220px] max-w-sm">
      <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground pointer-events-none" />
      <Input
        type="text"
        bind:value={searchQuery}
        placeholder={$tStore('show_mgmt.roster_page.search_placeholder')}
        class="pl-9 pr-8 h-9 text-xs"
      />
      {#if searchQuery}
        <button type="button" class="absolute right-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground cursor-pointer" onclick={() => (searchQuery = '')}>
          <X class="w-3.5 h-3.5" />
        </button>
      {/if}
    </div>

    <div class="flex items-center flex-wrap gap-1.5">
      <button
        type="button"
        class="px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-colors {selectedFilter === 'all' ? 'bg-primary text-primary-foreground' : 'bg-slate-100 text-slate-700 hover:bg-slate-200'}"
        onclick={() => (selectedFilter = 'all')}
      >
        {$tStore('show_mgmt.roster_page.filter_all_roles')}
      </button>
      <button
        type="button"
        class="inline-flex items-center gap-1 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-colors {selectedFilter === 'leadership' ? 'bg-primary text-primary-foreground' : 'bg-slate-100 text-slate-700 hover:bg-slate-200'}"
        onclick={() => (selectedFilter = 'leadership')}
      >
        <Shield class="w-3 h-3" />
        <span>{$tStore('show_mgmt.roster_page.filter_leadership')}</span>
      </button>
      <button
        type="button"
        class="inline-flex items-center gap-1 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-colors {selectedFilter === 'vocals' ? 'bg-primary text-primary-foreground' : 'bg-slate-100 text-slate-700 hover:bg-slate-200'}"
        onclick={() => (selectedFilter = 'vocals')}
      >
        <MicVocal class="w-3 h-3" />
        <span>{$tStore('show_mgmt.roster_page.filter_vocals')}</span>
      </button>
      <button
        type="button"
        class="inline-flex items-center gap-1 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-colors {selectedFilter === 'strings' ? 'bg-primary text-primary-foreground' : 'bg-slate-100 text-slate-700 hover:bg-slate-200'}"
        onclick={() => (selectedFilter = 'strings')}
      >
        <Guitar class="w-3 h-3" />
        <span>{$tStore('show_mgmt.roster_page.filter_strings')}</span>
      </button>
      <button
        type="button"
        class="inline-flex items-center gap-1 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-colors {selectedFilter === 'rhythm' ? 'bg-primary text-primary-foreground' : 'bg-slate-100 text-slate-700 hover:bg-slate-200'}"
        onclick={() => (selectedFilter = 'rhythm')}
      >
        <Disc class="w-3 h-3" />
        <span>{$tStore('show_mgmt.roster_page.filter_rhythm')}</span>
      </button>
      <button
        type="button"
        class="inline-flex items-center gap-1 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-colors {selectedFilter === 'keys_tech' ? 'bg-primary text-primary-foreground' : 'bg-slate-100 text-slate-700 hover:bg-slate-200'}"
        onclick={() => (selectedFilter = 'keys_tech')}
      >
        <SlidersVertical class="w-3 h-3" />
        <span>{$tStore('show_mgmt.roster_page.filter_keys_tech')}</span>
      </button>
    </div>

    <div class="flex items-center bg-slate-100 rounded-lg p-0.5 gap-0.5">
      <button
        type="button"
        class="p-1.5 rounded-md cursor-pointer transition-all {viewMode === 'cards' ? 'bg-white text-primary shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
        onclick={() => (viewMode = 'cards')}
        title={$tStore('show_mgmt.roster_page.view_cards')}
      >
        <LayoutGrid class="w-4 h-4" />
      </button>
      <button
        type="button"
        class="p-1.5 rounded-md cursor-pointer transition-all {viewMode === 'matrix' ? 'bg-white text-primary shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
        onclick={() => (viewMode = 'matrix')}
        title={$tStore('show_mgmt.roster_page.view_matrix')}
      >
        <TableIcon class="w-4 h-4" />
      </button>
    </div>
  </Card>

  <!-- Content View 1: Bento Card Grid View -->
  {#if viewMode === 'cards'}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#if filteredRoster.length === 0}
        <Card class="col-span-full p-12 flex flex-col items-center justify-center text-center gap-2">
          <Users class="w-10 h-10 text-muted-foreground" />
          <p class="text-xs text-muted-foreground m-0">{$tStore('show_mgmt.roster_page.no_members_found')}</p>
        </Card>
      {:else}
        {#each filteredRoster as member (member.id)}
          {@const workload = getWorkloadBadge(member.workloadStatus)}
          {@const WorkloadIcon = workload.icon}
          <Card class="p-4 flex flex-col justify-between gap-3 transition-all hover:shadow-md hover:border-primary/40">
            <div>
              <div class="flex items-start gap-3">
                <div class="w-10 h-10 rounded-full bg-primary/10 text-primary font-bold flex items-center justify-center shrink-0">
                  {member.fullName.charAt(0)}
                </div>
                <div class="flex-1 min-w-0">
                  <div class="flex items-center justify-between gap-1 flex-wrap">
                    <span class="font-bold text-foreground text-sm truncate">{member.fullName}</span>
                    <div class="flex gap-1">
                      {#if member.isDM}
                        <Badge class="bg-primary text-primary-foreground text-[10px] px-1.5 py-0">Show DM</Badge>
                      {/if}
                      {#if member.pmSongTitles && member.pmSongTitles.length > 0}
                        <Badge class="bg-blue-600 text-white text-[10px] px-1.5 py-0">PM ({member.pmSongTitles.length})</Badge>
                      {/if}
                      {#if member.qcSongTitles && member.qcSongTitles.length > 0}
                        <Badge class="bg-purple-600 text-white text-[10px] px-1.5 py-0">QC ({member.qcSongTitles.length})</Badge>
                      {/if}
                      {#if !member.isDM && (!member.pmSongTitles || member.pmSongTitles.length === 0) && (!member.qcSongTitles || member.qcSongTitles.length === 0)}
                        <Badge variant="secondary" class="text-[10px] px-1.5 py-0">Performer</Badge>
                      {/if}
                    </div>
                  </div>
                  <div class="flex flex-col gap-0.5 mt-1 text-[11px] text-muted-foreground">
                    <span class="flex items-center gap-1 truncate"><Mail class="w-3 h-3 shrink-0" /> {member.email}</span>
                    {#if member.phone}
                      <span class="flex items-center gap-1"><Phone class="w-3 h-3 shrink-0" /> {member.phone}</span>
                    {/if}
                  </div>
                </div>
              </div>

              <!-- Instrument & Performance Role Capabilities -->
              <div class="flex flex-wrap gap-1.5 mt-3">
                <Badge variant="outline" class="bg-primary/10 text-primary border-primary/20 gap-1 text-[11px] font-bold">
                  <Music class="w-3 h-3" />
                  <span>{$tStore(`show_mgmt.roles.${member.primaryInstrument}`)}</span>
                </Badge>
                {#each member.secondaryInstruments as secInst}
                  <Badge variant="secondary" class="text-[11px]">
                    + {$tStore(`show_mgmt.roles.${secInst}`)}
                  </Badge>
                {/each}
              </div>

              <!-- Assigned Numbers & Participation List -->
              <div class="mt-3 p-2.5 bg-muted/40 rounded-lg flex flex-col gap-1.5 border border-border/60">
                <div class="flex items-center justify-between text-[11px]">
                  <span class="text-muted-foreground">Participating in <strong>{member.assignedSongCount}</strong> songs:</span>
                  <span class="font-bold text-emerald-600">{member.attendanceRate}% Attended</span>
                </div>
                <div class="flex flex-wrap gap-1">
                  {#each member.assignedSongTitles as songTitle}
                    <a
                      href="/studio/shows/{showId}/numbers?q={encodeURIComponent(songTitle)}"
                      class="text-[10px] font-medium bg-card px-1.5 py-0.5 rounded border border-border text-foreground hover:text-primary hover:border-primary transition-colors"
                      title="View in Music Numbers"
                    >
                      {songTitle}
                    </a>
                  {/each}
                </div>
              </div>
            </div>

            <div class="flex flex-col gap-2 pt-2 border-t border-border">
              <div class="flex items-center justify-between text-xs">
                <div class="flex items-center gap-1 text-muted-foreground">
                  <Clock class="w-3.5 h-3.5 text-primary" />
                  <span><strong>{member.totalPracticeHours} hrs</strong> practice</span>
                </div>
                <Badge variant="outline" class="font-bold text-[10px] gap-1 {workload.variant}">
                  <WorkloadIcon class="w-3 h-3" />
                  <span>{$tStore(workload.labelKey)}</span>
                </Badge>
              </div>

              <div class="flex gap-1.5 mt-1">
                {#if canEditPerformerProfile(userRole)}
                  <Button
                    variant="outline"
                    size="sm"
                    class="flex-1 text-xs h-7 gap-1"
                    onclick={() => openEditModal(member)}
                  >
                    <PenLine class="w-3 h-3" />
                    <span>Edit Profile</span>
                  </Button>
                {/if}

                {#if canManageShowScoped(userRole, true)}
                  <Button
                    variant="ghost"
                    size="sm"
                    class="text-xs h-7 gap-1 text-red-600 hover:text-red-700 hover:bg-red-50"
                    onclick={() => promptRemoveMember(member)}
                  >
                    <Trash2 class="w-3 h-3" />
                    <span>Remove</span>
                  </Button>
                {/if}
              </div>
            </div>
          </Card>
        {/each}
      {/if}
    </div>
  {:else}
    <!-- Content View 2: High-Density Role & Effort Monitor Matrix -->
    <Card class="p-0 overflow-hidden shadow-sm">
      <Table>
        <TableHeader>
          <TableRow class="bg-slate-50">
            <TableHead>{$tStore('show_mgmt.roster_page.col_member')}</TableHead>
            <TableHead>{$tStore('show_mgmt.roster_page.col_primary_inst')}</TableHead>
            <TableHead>{$tStore('show_mgmt.roster_page.col_secondary_inst')}</TableHead>
            <TableHead>{$tStore('show_mgmt.roster_page.col_assigned_songs')}</TableHead>
            <TableHead>{$tStore('show_mgmt.roster_page.col_practice_hours')}</TableHead>
            <TableHead>{$tStore('show_mgmt.roster_page.col_attendance')}</TableHead>
            <TableHead>{$tStore('show_mgmt.roster_page.col_workload')}</TableHead>
            {#if canEditPerformerProfile(userRole)}
              <TableHead class="text-right">{$tStore('show_mgmt.roster_page.col_actions')}</TableHead>
            {/if}
          </TableRow>
        </TableHeader>
        <TableBody>
          {#each filteredRoster as member (member.id)}
            {@const workload = getWorkloadBadge(member.workloadStatus)}
            <TableRow>
              <TableCell>
                <div class="flex items-center gap-2.5">
                  <div class="w-8 h-8 rounded-full bg-primary/10 text-primary font-bold text-xs flex items-center justify-center shrink-0">
                    {member.fullName.charAt(0)}
                  </div>
                  <div>
                    <div class="font-bold text-xs text-foreground flex items-center gap-1.5">
                      <span>{member.fullName}</span>
                      {#if member.isDM}<Badge class="bg-primary text-primary-foreground text-[9px] px-1 py-0">DM</Badge>{/if}
                      {#if member.pmSongTitles && member.pmSongTitles.length > 0}<Badge class="bg-blue-600 text-white text-[9px] px-1 py-0">PM</Badge>{/if}
                      {#if member.qcSongTitles && member.qcSongTitles.length > 0}<Badge class="bg-purple-600 text-white text-[9px] px-1 py-0">QC</Badge>{/if}
                    </div>
                    <div class="text-[11px] text-muted-foreground">{member.email}</div>
                  </div>
                </div>
              </TableCell>
              <TableCell>
                <Badge variant="outline" class="bg-primary/10 text-primary border-primary/20 text-xs">
                  {$tStore(`show_mgmt.roles.${member.primaryInstrument}`)}
                </Badge>
              </TableCell>
              <TableCell>
                <div class="flex flex-wrap gap-1 max-w-[160px]">
                  {#each member.secondaryInstruments as sInst}
                    <span class="text-[10px] bg-slate-100 text-slate-700 px-1 py-0.5 rounded">{$tStore(`show_mgmt.roles.${sInst}`)}</span>
                  {/each}
                  {#if member.secondaryInstruments.length === 0}
                    <span class="text-muted-foreground text-xs">—</span>
                  {/if}
                </div>
              </TableCell>
              <TableCell>
                <a href="/studio/shows/{showId}/numbers?q={encodeURIComponent(member.fullName)}" class="text-xs font-semibold text-primary hover:underline">
                  <strong>{member.assignedSongCount}</strong> songs
                </a>
              </TableCell>
              <TableCell class="text-xs font-semibold">{member.totalPracticeHours} hrs</TableCell>
              <TableCell class="text-xs font-bold text-emerald-600">{member.attendanceRate}%</TableCell>
              <TableCell>
                <Badge variant="outline" class="text-[10px] font-bold {workload.variant}">
                  {$tStore(workload.labelKey)}
                </Badge>
              </TableCell>
              {#if canEditPerformerProfile(userRole)}
                <TableCell class="text-right">
                  <div class="inline-flex items-center gap-1">
                    <Button variant="ghost" size="sm" class="h-7 w-7 p-0" onclick={() => openEditModal(member)}>
                      <PenLine class="w-3.5 h-3.5" />
                    </Button>
                    {#if canManageShowScoped(userRole, true)}
                      <Button variant="ghost" size="sm" class="h-7 w-7 p-0 text-red-600 hover:text-red-700" onclick={() => promptRemoveMember(member)}>
                        <Trash2 class="w-3.5 h-3.5" />
                      </Button>
                    {/if}
                  </div>
                </TableCell>
              {/if}
            </TableRow>
          {/each}
        </TableBody>
      </Table>
    </Card>
  {/if}
</div>

<!-- Modal: Add / Edit Roster Member -->
<Dialog bind:open={isAddEditModalOpen}>
  <DialogContent class="max-w-md">
    <DialogHeader>
      <DialogTitle>
        {editingMember
          ? $tStore('show_mgmt.roster_modal.title_edit')
          : $tStore('show_mgmt.roster_modal.title_add')}
      </DialogTitle>
      <DialogDescription>
        Manage performer credentials, assigned leadership role, and band capability.
      </DialogDescription>
    </DialogHeader>

    <form onsubmit={handleSaveMember} class="flex flex-col gap-3.5 mt-2">
      <div class="flex flex-col gap-1.5">
        <Label for="member-name">Member Full Name *</Label>
        <Input
          id="member-name"
          type="text"
          bind:value={formFullName}
          placeholder="e.g. Minh Pháp"
          required
        />
      </div>

      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <Label for="member-email">Email Address</Label>
          <Input
            id="member-email"
            type="email"
            bind:value={formEmail}
            placeholder="e.g. minhphap@csac.local"
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="member-phone">Phone / Zalo</Label>
          <Input
            id="member-phone"
            type="tel"
            bind:value={formPhone}
            placeholder="+84 901 234 567"
          />
        </div>
      </div>

      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <Label for="member-show-role">{$tStore('show_mgmt.roster_modal.select_role')}</Label>
          <select
            id="member-show-role"
            bind:value={formShowRole}
            class="h-9 px-3 text-xs bg-background border border-input rounded-md outline-none"
            disabled={!canManageShowRoster(userRole)}
          >
            <option value="DM">Delivery Manager (DM)</option>
            <option value="PM">Performance Manager (PM)</option>
            <option value="QC">Quality Reviewer (QC)</option>
            <option value="Performer">Cast / Performer</option>
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="member-practice-hours">Practice Hours</Label>
          <Input
            id="member-practice-hours"
            type="number"
            min="0"
            bind:value={formPracticeHours}
          />
        </div>
      </div>

      <div class="flex flex-col gap-1.5">
        <Label for="member-primary-inst">{$tStore('show_mgmt.roster_modal.select_instrument')} *</Label>
        <select id="member-primary-inst" bind:value={formPrimaryInst} class="h-9 px-3 text-xs bg-background border border-input rounded-md outline-none">
          {#each AVAILABLE_BAND_ROLES as role}
            <option value={role.id}>{$tStore(role.key)}</option>
          {/each}
        </select>
      </div>

      <div class="flex flex-col gap-1.5">
        <Label>Secondary / Backup Instruments</Label>
        <div class="flex flex-wrap gap-1.5 max-h-28 overflow-y-auto p-1 border border-input rounded-md">
          {#each AVAILABLE_BAND_ROLES as role}
            {#if role.id !== formPrimaryInst}
              {@const RoleIcon = role.icon}
              <button
                type="button"
                class="inline-flex items-center gap-1 text-[11px] px-2 py-1 rounded border transition-colors cursor-pointer {formSecondaryInst.includes(role.id) ? 'bg-primary text-primary-foreground border-primary' : 'bg-slate-50 text-slate-700 border-slate-200 hover:bg-slate-100'}"
                onclick={() => toggleSecondaryInstrument(role.id)}
              >
                <RoleIcon class="w-3 h-3" />
                <span>{$tStore(role.key)}</span>
              </button>
            {/if}
          {/each}
        </div>
      </div>

      <DialogFooter class="mt-4">
        <Button
          type="button"
          variant="outline"
          onclick={() => (isAddEditModalOpen = false)}
        >
          {$tStore('show_mgmt.roster_page.btn_cancel')}
        </Button>
        <Button type="submit">
          {$tStore('show_mgmt.roster_modal.btn_submit')}
        </Button>
      </DialogFooter>
    </form>
  </DialogContent>
</Dialog>

<!-- Modal: Confirm Remove Member -->
<Dialog bind:open={isRemoveModalOpen}>
  <DialogContent class="max-w-sm">
    <DialogHeader>
      <DialogTitle class="text-red-600 flex items-center gap-2">
        <TriangleAlert class="w-5 h-5 text-red-600" />
        <span>{$tStore('show_mgmt.roster_page.confirm_remove_title')}</span>
      </DialogTitle>
      {#if memberToRemove}
        <DialogDescription>
          {$tStore('show_mgmt.roster_page.confirm_remove_msg', { name: memberToRemove.fullName })}
        </DialogDescription>
      {/if}
    </DialogHeader>

    {#if memberToRemove}
      <div class="p-3 bg-red-50 text-red-800 rounded-lg text-xs flex flex-col gap-1">
        <span>Active Assignments: <strong>{memberToRemove.assignedSongCount} songs</strong></span>
        <span>Practice Effort: <strong>{memberToRemove.totalPracticeHours} hrs</strong></span>
      </div>
    {/if}

    <DialogFooter class="mt-4">
      <Button
        type="button"
        variant="outline"
        onclick={() => (isRemoveModalOpen = false)}
      >
        {$tStore('show_mgmt.roster_page.btn_cancel')}
      </Button>
      <Button
        type="button"
        variant="destructive"
        onclick={confirmRemoveMember}
      >
        {$tStore('show_mgmt.roster_page.btn_confirm_remove')}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
