<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    Users,
    UserCheck,
    Shield,
    Music,
    Mail,
    Phone,
    AlertTriangle,
    Plus,
    Search,
    SlidersHorizontal,
    LayoutGrid,
    Table,
    Sparkles,
    CheckCircle2,
    Activity,
    Clock,
    Zap,
    Mic2,
    Guitar,
    Disc,
    Sliders,
    Trash2,
    Edit3,
    UserCog,
    HelpCircle,
    ChevronDown,
    X,
  } from '@lucide/svelte';
  import type { BandRole, ShowRole, ShowRosterMember, UserRole } from '$lib/types/timetable';
  import { canManageShowRoster, canEditPerformerProfile, hasRole } from '$lib/auth';

  // Active Session Simulation Role for testing & live governance
  let activeUserRole = $state<UserRole>('dm');

  // Roster Seed Data with full effort, roles, and assigned songs
  let roster = $state<ShowRosterMember[]>([
    {
      id: 'mem-1',
      userId: 'u-101',
      fullName: 'Minh Pháp',
      email: 'minhphap@csac.local',
      phone: '+84 901 234 567',
      showRole: 'DM',
      primaryInstrument: 'vocal_lead',
      secondaryInstruments: ['guitar_rhythm'],
      assignedSongCount: 5,
      assignedSongTitles: ['Hào Khí Việt Nam', 'Đi Giữa Trời Rực Rỡ', 'Nối Vòng Tay Lớn', 'Bài Ca Hy Vọng', 'Dấu Chân Phía Trước'],
      totalPracticeHours: 24,
      workloadStatus: 'fatigued',
      attendanceRate: 98,
      joinedAt: '2026-08-15',
    },
    {
      id: 'mem-2',
      userId: 'u-102',
      fullName: 'Hoàng Nam',
      email: 'hoangnam@csac.local',
      phone: '+84 912 345 678',
      showRole: 'PM',
      primaryInstrument: 'guitar_lead',
      secondaryInstruments: ['guitar_rhythm'],
      assignedSongCount: 3,
      assignedSongTitles: ['Hào Khí Việt Nam', 'Đi Giữa Trời Rực Rỡ', 'Khát Vọng Tuổi Trẻ'],
      totalPracticeHours: 16,
      workloadStatus: 'moderate',
      attendanceRate: 94,
      joinedAt: '2026-08-18',
    },
    {
      id: 'mem-3',
      userId: 'u-103',
      fullName: 'Bảo Anh',
      email: 'baoanh@csac.local',
      phone: '+84 934 567 890',
      showRole: 'PM',
      primaryInstrument: 'bass',
      secondaryInstruments: ['guitar_lead'],
      assignedSongCount: 4,
      assignedSongTitles: ['Hào Khí Việt Nam', 'Đi Giữa Trời Rực Rỡ', 'Nối Vòng Tay Lớn', 'Rock Vầng Trăng'],
      totalPracticeHours: 18,
      workloadStatus: 'moderate',
      attendanceRate: 92,
      joinedAt: '2026-08-20',
    },
    {
      id: 'mem-4',
      userId: 'u-104',
      fullName: 'Thu Hà',
      email: 'thuha@csac.local',
      phone: '+84 945 678 901',
      showRole: 'QC',
      primaryInstrument: 'drums',
      secondaryInstruments: ['percussion'],
      assignedSongCount: 2,
      assignedSongTitles: ['Hào Khí Việt Nam', 'Nối Vòng Tay Lớn'],
      totalPracticeHours: 10,
      workloadStatus: 'optimal',
      attendanceRate: 100,
      joinedAt: '2026-08-22',
    },
    {
      id: 'mem-5',
      userId: 'u-105',
      fullName: 'Khánh Linh',
      email: 'khanhlinh@csac.local',
      phone: '+84 956 789 012',
      showRole: 'Performer',
      primaryInstrument: 'vocal_harmony',
      secondaryInstruments: ['keys'],
      assignedSongCount: 2,
      assignedSongTitles: ['Hào Khí Việt Nam', 'Bài Ca Hy Vọng'],
      totalPracticeHours: 8,
      workloadStatus: 'optimal',
      attendanceRate: 95,
      joinedAt: '2026-08-25',
    },
    {
      id: 'mem-6',
      userId: 'u-106',
      fullName: 'Quốc Bảo',
      email: 'quocbao@csac.local',
      phone: '+84 967 890 123',
      showRole: 'Performer',
      primaryInstrument: 'keys',
      secondaryInstruments: ['sound_tech'],
      assignedSongCount: 3,
      assignedSongTitles: ['Hào Khí Việt Nam', 'Đi Giữa Trời Rực Rỡ', 'Bài Ca Hy Vọng'],
      totalPracticeHours: 14,
      workloadStatus: 'moderate',
      attendanceRate: 90,
      joinedAt: '2026-08-27',
    },
    {
      id: 'mem-7',
      userId: 'u-107',
      fullName: 'Trọng Hiếu',
      email: 'tronghieu@csac.local',
      phone: '+84 978 901 234',
      showRole: 'Performer',
      primaryInstrument: 'sound_tech',
      secondaryInstruments: [],
      assignedSongCount: 1,
      assignedSongTitles: ['Hào Khí Việt Nam (Live Audio)'],
      totalPracticeHours: 6,
      workloadStatus: 'optimal',
      attendanceRate: 100,
      joinedAt: '2026-08-29',
    },
  ]);

  // View Mode & Filtering State
  let viewMode = $state<'cards' | 'matrix'>('cards');
  let searchQuery = $state('');
  let selectedFilter = $state<string>('all');

  // Modal State
  let isAddEditModalOpen = $state(false);
  let isRemoveModalOpen = $state(false);
  let editingMember = $state<ShowRosterMember | null>(null);
  let memberToRemove = $state<ShowRosterMember | null>(null);

  // Modal Form Inputs
  let formFullName = $state('');
  let formEmail = $state('');
  let formPhone = $state('');
  let formShowRole = $state<ShowRole>('Performer');
  let formPrimaryInst = $state<BandRole>('vocal_lead');
  let formSecondaryInst = $state<BandRole[]>([]);
  let formPracticeHours = $state(4);

  // Band Roles list with labels and icons
  const AVAILABLE_BAND_ROLES: { id: BandRole; key: string; icon: any; category: 'vocals' | 'strings' | 'rhythm' | 'keys_tech' }[] = [
    { id: 'vocal_lead', key: 'show_mgmt.roles.vocal_lead', icon: Mic2, category: 'vocals' },
    { id: 'vocal_harmony', key: 'show_mgmt.roles.vocal_harmony', icon: Mic2, category: 'vocals' },
    { id: 'guitar_lead', key: 'show_mgmt.roles.guitar_lead', icon: Guitar, category: 'strings' },
    { id: 'guitar_rhythm', key: 'show_mgmt.roles.guitar_rhythm', icon: Guitar, category: 'strings' },
    { id: 'bass', key: 'show_mgmt.roles.bass', icon: Guitar, category: 'strings' },
    { id: 'keys', key: 'show_mgmt.roles.keys', icon: Sliders, category: 'keys_tech' },
    { id: 'drums', key: 'show_mgmt.roles.drums', icon: Disc, category: 'rhythm' },
    { id: 'percussion', key: 'show_mgmt.roles.percussion', icon: Disc, category: 'rhythm' },
    { id: 'sound_tech', key: 'show_mgmt.roles.sound_tech', icon: SlidersHorizontal, category: 'keys_tech' },
  ];

  // Derived Summary Metrics
  let totalMembers = $derived(roster.length);
  let dmCount = $derived(roster.filter((m) => m.showRole === 'DM').length);
  let pmCount = $derived(roster.filter((m) => m.showRole === 'PM').length);
  let qcCount = $derived(roster.filter((m) => m.showRole === 'QC').length);
  let performerCount = $derived(roster.filter((m) => m.showRole === 'Performer').length);
  let totalPracticeHours = $derived(roster.reduce((sum, m) => sum + m.totalPracticeHours, 0));

  let fatigueCount = $derived(roster.filter((m) => m.workloadStatus === 'fatigued').length);
  let moderateCount = $derived(roster.filter((m) => m.workloadStatus === 'moderate').length);
  let optimalCount = $derived(roster.filter((m) => m.workloadStatus === 'optimal').length);

  // Performance Role Section Counts
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

  // Filtered Roster
  let filteredRoster = $derived(
    roster.filter((m) => {
      // Search filter
      const q = searchQuery.toLowerCase().trim();
      const matchQuery =
        !q ||
        m.fullName.toLowerCase().includes(q) ||
        m.email.toLowerCase().includes(q) ||
        m.showRole.toLowerCase().includes(q) ||
        m.primaryInstrument.toLowerCase().includes(q);

      if (!matchQuery) return false;

      // Category filter
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

  // Workload Status Evaluation Helper
  function calculateWorkload(songCount: number): 'optimal' | 'moderate' | 'fatigued' {
    if (songCount >= 5) return 'fatigued';
    if (songCount >= 3) return 'moderate';
    return 'optimal';
  }

  // Action Handlers
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

  function handleSaveMember(e: Event) {
    e.preventDefault();
    if (!formFullName.trim()) return;

    if (editingMember) {
      roster = roster.map((m) =>
        m.id === editingMember?.id
          ? {
              ...m,
              fullName: formFullName.trim(),
              email: formEmail.trim(),
              phone: formPhone.trim(),
              showRole: formShowRole,
              primaryInstrument: formPrimaryInst,
              secondaryInstruments: formSecondaryInst,
              totalPracticeHours: Number(formPracticeHours) || m.totalPracticeHours,
            }
          : m
      );
    } else {
      const newMember: ShowRosterMember = {
        id: `mem-${Date.now()}`,
        userId: `u-${Date.now()}`,
        fullName: formFullName.trim(),
        email: formEmail.trim() || `${formFullName.toLowerCase().replace(/\s+/g, '')}@csac.local`,
        phone: formPhone.trim() || '+84 900 000 000',
        showRole: formShowRole,
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
    }
    isAddEditModalOpen = false;
  }

  function promptRemoveMember(member: ShowRosterMember) {
    memberToRemove = member;
    isRemoveModalOpen = true;
  }

  function confirmRemoveMember() {
    if (memberToRemove) {
      roster = roster.filter((m) => m.id !== memberToRemove?.id);
      memberToRemove = null;
      isRemoveModalOpen = false;
    }
  }

  function toggleSecondaryInstrument(inst: BandRole) {
    if (formSecondaryInst.includes(inst)) {
      formSecondaryInst = formSecondaryInst.filter((i) => i !== inst);
    } else {
      formSecondaryInst = [...formSecondaryInst, inst];
    }
  }

  function getShowRoleBadgeClass(role: ShowRole): string {
    switch (role) {
      case 'DM':
        return 'role-badge-dm';
      case 'PM':
        return 'role-badge-pm';
      case 'QC':
        return 'role-badge-qc';
      default:
        return 'role-badge-performer';
    }
  }

  function getWorkloadBadge(status: 'optimal' | 'moderate' | 'fatigued') {
    switch (status) {
      case 'fatigued':
        return { labelKey: 'show_mgmt.workload_fatigued', cssClass: 'workload-red', icon: AlertTriangle };
      case 'moderate':
        return { labelKey: 'show_mgmt.workload_moderate', cssClass: 'workload-yellow', icon: Activity };
      default:
        return { labelKey: 'show_mgmt.workload_optimal', cssClass: 'workload-green', icon: CheckCircle2 };
    }
  }
</script>

<div class="roster-hub">
  <!-- Top Header & Role Simulation Bar -->
  <div class="header-card bento-card">
    <div class="header-left">
      <div class="header-title-row">
        <h2>{$tStore('show_mgmt.roster_page.title')}</h2>
        <span class="member-count-badge">{$tStore('show_mgmt.roster_page.stat_total_members')}: {totalMembers}</span>
      </div>
      <p class="header-desc">{$tStore('show_mgmt.roster_page.subtitle')}</p>
    </div>

    <div class="header-controls">
      <!-- RBAC Simulation Switcher -->
      <div class="role-switch-box">
        <span class="role-switch-label">{$tStore('show_mgmt.roster_page.active_role_badge')}</span>
        <select bind:value={activeUserRole} class="role-select">
          <option value="dm">Delivery Manager (DM)</option>
          <option value="pm">Performance Manager (PM)</option>
          <option value="qc">QC Reviewer</option>
          <option value="member">Performer (Member)</option>
          <option value="admin">Admin</option>
        </select>
      </div>

      {#if canManageShowRoster(activeUserRole)}
        <button type="button" class="bento-btn bento-btn-primary" onclick={openAddModal}>
          <Plus size={16} />
          <span>{$tStore('show_mgmt.roster_modal.btn_add_member')}</span>
        </button>
      {/if}
    </div>
  </div>

  <!-- Bento 4-Box Monitor Matrix & Performance Metrics -->
  <div class="monitor-metrics-grid">
    <!-- Box 1: Roster Headcount & Leadership Breakdown -->
    <div class="metric-bento-card bento-card">
      <div class="metric-top">
        <div class="metric-icon-wrap bg-orange-soft">
          <Users size={20} class="text-orange" />
        </div>
        <div class="metric-stat-group">
          <span class="metric-num">{totalMembers}</span>
          <span class="metric-title">{$tStore('show_mgmt.roster_page.stat_total_members')}</span>
        </div>
      </div>
      <div class="leadership-pills">
        <span class="lead-pill dm-pill">DM: {dmCount}</span>
        <span class="lead-pill pm-pill">PM: {pmCount}</span>
        <span class="lead-pill qc-pill">QC: {qcCount}</span>
        <span class="lead-pill perf-pill">Cast: {performerCount}</span>
      </div>
    </div>

    <!-- Box 2: Band Sections & Performance Role Coverage -->
    <div class="metric-bento-card bento-card">
      <div class="metric-top">
        <div class="metric-icon-wrap bg-blue-soft">
          <Music size={20} class="text-blue" />
        </div>
        <div class="metric-stat-group">
          <span class="metric-num">4 / 4</span>
          <span class="metric-title">{$tStore('show_mgmt.roster_page.stat_coverage')}</span>
        </div>
      </div>
      <div class="coverage-bar-group">
        <div class="coverage-item" title="Vocals">
          <Mic2 size={13} class="text-orange" />
          <span>Vocals: <strong>{vocalsCount}</strong></span>
        </div>
        <div class="coverage-item" title="Strings">
          <Guitar size={13} class="text-blue" />
          <span>Strings: <strong>{stringsCount}</strong></span>
        </div>
        <div class="coverage-item" title="Rhythm">
          <Disc size={13} class="text-green" />
          <span>Rhythm: <strong>{rhythmCount}</strong></span>
        </div>
        <div class="coverage-item" title="Keys/Tech">
          <Sliders size={13} class="text-purple" />
          <span>Keys: <strong>{keysTechCount}</strong></span>
        </div>
      </div>
    </div>

    <!-- Box 3: Workload & Fatigue Risk Distribution -->
    <div class="metric-bento-card bento-card">
      <div class="metric-top">
        <div class="metric-icon-wrap bg-red-soft">
          <Activity size={20} class="text-red" />
        </div>
        <div class="metric-stat-group">
          <span class="metric-num {fatigueCount > 0 ? 'text-red' : 'text-green'}">
            {fatigueCount}
          </span>
          <span class="metric-title">Fatigue Risks (5+ Songs)</span>
        </div>
      </div>
      <div class="workload-ratio-strip">
        <div class="ratio-bubble green-bubble">
          <span class="ratio-val">{optimalCount}</span>
          <span class="ratio-lbl">Optimal (1-2)</span>
        </div>
        <div class="ratio-bubble yellow-bubble">
          <span class="ratio-val">{moderateCount}</span>
          <span class="ratio-lbl">Moderate (3-4)</span>
        </div>
        <div class="ratio-bubble red-bubble">
          <span class="ratio-val">{fatigueCount}</span>
          <span class="ratio-lbl">Fatigued (5+)</span>
        </div>
      </div>
    </div>

    <!-- Box 4: Total Show Practice Effort & Rehearsal Hours -->
    <div class="metric-bento-card bento-card">
      <div class="metric-top">
        <div class="metric-icon-wrap bg-purple-soft">
          <Clock size={20} class="text-purple" />
        </div>
        <div class="metric-stat-group">
          <span class="metric-num">{totalPracticeHours}h</span>
          <span class="metric-title">{$tStore('show_mgmt.roster_page.stat_total_practice')}</span>
        </div>
      </div>
      <div class="effort-subtext">
        <Zap size={14} class="text-orange" />
        <span>Average {Math.round(totalPracticeHours / (totalMembers || 1))}h effort per performer</span>
      </div>
    </div>
  </div>

  <!-- Search, Filter Toolbar & View Mode Switcher -->
  <div class="toolbar-card bento-card">
    <div class="search-box">
      <Search size={16} class="search-icon" />
      <input
        type="text"
        bind:value={searchQuery}
        placeholder={$tStore('show_mgmt.roster_page.search_placeholder')}
        class="search-input"
      />
      {#if searchQuery}
        <button type="button" class="clear-search-btn" onclick={() => (searchQuery = '')}>
          <X size={14} />
        </button>
      {/if}
    </div>

    <div class="filter-chips">
      <button
        type="button"
        class="filter-chip {selectedFilter === 'all' ? 'active' : ''}"
        onclick={() => (selectedFilter = 'all')}
      >
        {$tStore('show_mgmt.roster_page.filter_all_roles')}
      </button>
      <button
        type="button"
        class="filter-chip {selectedFilter === 'leadership' ? 'active' : ''}"
        onclick={() => (selectedFilter = 'leadership')}
      >
        <Shield size={13} />
        {$tStore('show_mgmt.roster_page.filter_leadership')}
      </button>
      <button
        type="button"
        class="filter-chip {selectedFilter === 'vocals' ? 'active' : ''}"
        onclick={() => (selectedFilter = 'vocals')}
      >
        <Mic2 size={13} />
        {$tStore('show_mgmt.roster_page.filter_vocals')}
      </button>
      <button
        type="button"
        class="filter-chip {selectedFilter === 'strings' ? 'active' : ''}"
        onclick={() => (selectedFilter = 'strings')}
      >
        <Guitar size={13} />
        {$tStore('show_mgmt.roster_page.filter_strings')}
      </button>
      <button
        type="button"
        class="filter-chip {selectedFilter === 'rhythm' ? 'active' : ''}"
        onclick={() => (selectedFilter = 'rhythm')}
      >
        <Disc size={13} />
        {$tStore('show_mgmt.roster_page.filter_rhythm')}
      </button>
      <button
        type="button"
        class="filter-chip {selectedFilter === 'keys_tech' ? 'active' : ''}"
        onclick={() => (selectedFilter = 'keys_tech')}
      >
        <Sliders size={13} />
        {$tStore('show_mgmt.roster_page.filter_keys_tech')}
      </button>
    </div>

    <div class="view-mode-toggle">
      <button
        type="button"
        class="view-btn {viewMode === 'cards' ? 'active' : ''}"
        onclick={() => (viewMode = 'cards')}
        title={$tStore('show_mgmt.roster_page.view_cards')}
      >
        <LayoutGrid size={16} />
      </button>
      <button
        type="button"
        class="view-btn {viewMode === 'matrix' ? 'active' : ''}"
        onclick={() => (viewMode = 'matrix')}
        title={$tStore('show_mgmt.roster_page.view_matrix')}
      >
        <Table size={16} />
      </button>
    </div>
  </div>

  <!-- Content View 1: Bento Card Grid View -->
  {#if viewMode === 'cards'}
    <div class="roster-bento-grid">
      {#if filteredRoster.length === 0}
        <div class="empty-state bento-card">
          <Users size={36} class="text-slate-400" />
          <p>{$tStore('show_mgmt.roster_page.no_members_found')}</p>
        </div>
      {:else}
        {#each filteredRoster as member (member.id)}
          {@const workload = getWorkloadBadge(member.workloadStatus)}
          <div class="member-card bento-card">
            <!-- Header: Avatar, Name, Email, Show Role Badge -->
            <div class="member-card-header">
              <div class="avatar-circle">
                {member.fullName.charAt(0)}
              </div>
              <div class="member-info-col">
                <div class="name-role-row">
                  <span class="member-name">{member.fullName}</span>
                  <span class="role-badge {getShowRoleBadgeClass(member.showRole)}">
                    {member.showRole}
                  </span>
                </div>
                <div class="contact-subline">
                  <span class="contact-item"><Mail size={12} /> {member.email}</span>
                  {#if member.phone}
                    <span class="contact-item"><Phone size={12} /> {member.phone}</span>
                  {/if}
                </div>
              </div>
            </div>

            <!-- Instrument & Performance Role Capabilities -->
            <div class="instrument-tag-row">
              <div class="primary-inst-chip">
                <Music size={13} class="text-orange" />
                <span>{$tStore(`show_mgmt.roles.${member.primaryInstrument}`)}</span>
              </div>
              {#each member.secondaryInstruments as secInst}
                <div class="sec-inst-chip">
                  <span>+ {$tStore(`show_mgmt.roles.${secInst}`)}</span>
                </div>
              {/each}
            </div>

            <!-- Assigned Numbers & Participation List -->
            <div class="songs-assigned-box">
              <div class="songs-assigned-header">
                <span class="songs-assigned-title">
                  Participating in <strong>{member.assignedSongCount}</strong> Music Numbers:
                </span>
                <span class="attendance-tag">{member.attendanceRate}% Attendance</span>
              </div>
              <div class="song-chips-wrap">
                {#each member.assignedSongTitles as songTitle}
                  <span class="song-chip">{songTitle}</span>
                {/each}
              </div>
            </div>

            <!-- Workload & Practice Effort Summary Bar -->
            <div class="effort-workload-bar">
              <div class="effort-stat">
                <Clock size={13} class="text-purple" />
                <span><strong>{member.totalPracticeHours} hrs</strong> total practice</span>
              </div>
              <div class="workload-pill {workload.cssClass}">
                <svelte:component this={workload.icon} size={12} />
                <span>{$tStore(workload.labelKey)}</span>
              </div>
            </div>

            <!-- Action Buttons: Role Gated -->
            <div class="card-actions-bar">
              {#if canEditPerformerProfile(activeUserRole)}
                <button
                  type="button"
                  class="action-btn edit-btn"
                  onclick={() => openEditModal(member)}
                >
                  <Edit3 size={13} />
                  <span>Edit Profile</span>
                </button>
              {/if}

              {#if canManageShowRoster(activeUserRole)}
                <button
                  type="button"
                  class="action-btn remove-btn"
                  onclick={() => promptRemoveMember(member)}
                >
                  <Trash2 size={13} />
                  <span>Remove</span>
                </button>
              {/if}
            </div>
          </div>
        {/each}
      {/if}
    </div>
  {:else}
    <!-- Content View 2: High-Density Role & Effort Monitor Matrix -->
    <div class="matrix-table-card bento-card">
      <div class="matrix-card-header">
        <div>
          <h3>{$tStore('show_mgmt.roster_page.matrix_section_title')}</h3>
          <p>{$tStore('show_mgmt.roster_page.matrix_section_desc')}</p>
        </div>
      </div>

      <div class="table-responsive">
        <table class="matrix-table">
          <thead>
            <tr>
              <th>{$tStore('show_mgmt.roster_page.col_member')}</th>
              <th>{$tStore('show_mgmt.roster_page.col_primary_inst')}</th>
              <th>{$tStore('show_mgmt.roster_page.col_secondary_inst')}</th>
              <th>{$tStore('show_mgmt.roster_page.col_assigned_songs')}</th>
              <th>{$tStore('show_mgmt.roster_page.col_practice_hours')}</th>
              <th>{$tStore('show_mgmt.roster_page.col_attendance')}</th>
              <th>{$tStore('show_mgmt.roster_page.col_workload')}</th>
              {#if canEditPerformerProfile(activeUserRole)}
                <th>{$tStore('show_mgmt.roster_page.col_actions')}</th>
              {/if}
            </tr>
          </thead>
          <tbody>
            {#each filteredRoster as member (member.id)}
              {@const workload = getWorkloadBadge(member.workloadStatus)}
              <tr>
                <!-- Member Name & Role -->
                <td>
                  <div class="table-member-cell">
                    <div class="table-avatar">{member.fullName.charAt(0)}</div>
                    <div>
                      <div class="table-member-name">{member.fullName}</div>
                      <div class="table-member-email">{member.email}</div>
                    </div>
                    <span class="role-badge-sm {getShowRoleBadgeClass(member.showRole)}">
                      {member.showRole}
                    </span>
                  </div>
                </td>

                <!-- Primary Instrument -->
                <td>
                  <div class="table-primary-chip">
                    <Music size={12} class="text-orange" />
                    <span>{$tStore(`show_mgmt.roles.${member.primaryInstrument}`)}</span>
                  </div>
                </td>

                <!-- Secondary Instruments -->
                <td>
                  {#if member.secondaryInstruments.length > 0}
                    <div class="table-sec-chips">
                      {#each member.secondaryInstruments as sInst}
                        <span class="sec-mini-tag">{$tStore(`show_mgmt.roles.${sInst}`)}</span>
                      {/each}
                    </div>
                  {:else}
                    <span class="text-muted">—</span>
                  {/if}
                </td>

                <!-- Assigned Songs Count & List preview -->
                <td>
                  <div class="songs-count-cell">
                    <strong>{member.assignedSongCount}</strong> songs
                    <span class="songs-preview-text" title={member.assignedSongTitles.join(', ')}>
                      ({member.assignedSongTitles.slice(0, 2).join(', ')}{member.assignedSongTitles.length > 2 ? '...' : ''})
                    </span>
                  </div>
                </td>

                <!-- Practice Hours -->
                <td>
                  <div class="hours-badge">
                    <Clock size={12} class="text-purple" />
                    <strong>{member.totalPracticeHours} hrs</strong>
                  </div>
                </td>

                <!-- Attendance Rate -->
                <td>
                  <span class="attendance-num">{member.attendanceRate}%</span>
                </td>

                <!-- Workload Status -->
                <td>
                  <div class="table-workload-tag {workload.cssClass}">
                    <svelte:component this={workload.icon} size={11} />
                    <span>{$tStore(workload.labelKey)}</span>
                  </div>
                </td>

                <!-- Actions -->
                {#if canEditPerformerProfile(activeUserRole)}
                  <td>
                    <div class="table-actions">
                      <button
                        type="button"
                        class="icon-btn edit-icon"
                        onclick={() => openEditModal(member)}
                        title="Edit Member"
                      >
                        <Edit3 size={14} />
                      </button>
                      {#if canManageShowRoster(activeUserRole)}
                        <button
                          type="button"
                          class="icon-btn remove-icon"
                          onclick={() => promptRemoveMember(member)}
                          title="Remove Member"
                        >
                          <Trash2 size={14} />
                        </button>
                      {/if}
                    </div>
                  </td>
                {/if}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>

<!-- Modal: Add / Edit Roster Member -->
{#if isAddEditModalOpen}
  <div class="modal-backdrop" onclick={() => (isAddEditModalOpen = false)} role="presentation">
    <div class="modal-card bento-card" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="modal-header">
        <h3>
          {editingMember
            ? $tStore('show_mgmt.roster_modal.title_edit')
            : $tStore('show_mgmt.roster_modal.title_add')}
        </h3>
        <button type="button" class="close-modal-btn" onclick={() => (isAddEditModalOpen = false)}>
          <X size={18} />
        </button>
      </div>

      <form onsubmit={handleSaveMember} class="modal-form">
        <!-- Full Name -->
        <div class="form-group">
          <label for="member-name">Member Full Name *</label>
          <input
            id="member-name"
            type="text"
            bind:value={formFullName}
            placeholder="e.g. Minh Pháp"
            required
            class="form-input"
          />
        </div>

        <!-- Email & Phone -->
        <div class="form-row">
          <div class="form-group">
            <label for="member-email">Email Address</label>
            <input
              id="member-email"
              type="email"
              bind:value={formEmail}
              placeholder="e.g. minhphap@csac.local"
              class="form-input"
            />
          </div>

          <div class="form-group">
            <label for="member-phone">Phone / Zalo</label>
            <input
              id="member-phone"
              type="tel"
              bind:value={formPhone}
              placeholder="+84 901 234 567"
              class="form-input"
            />
          </div>
        </div>

        <!-- Show Leadership Role Delegation & Practice Hours -->
        <div class="form-row">
          <div class="form-group">
            <label for="member-show-role">
              {$tStore('show_mgmt.roster_modal.select_role')}
            </label>
            <select
              id="member-show-role"
              bind:value={formShowRole}
              class="form-input"
              disabled={!canManageShowRoster(activeUserRole)}
            >
              <option value="DM">Delivery Manager (DM)</option>
              <option value="PM">Performance Manager (PM)</option>
              <option value="QC">Quality Reviewer (QC)</option>
              <option value="Performer">Cast / Performer</option>
            </select>
          </div>

          <div class="form-group">
            <label for="member-practice-hours">Practice Hours Committed</label>
            <input
              id="member-practice-hours"
              type="number"
              min="0"
              bind:value={formPracticeHours}
              class="form-input"
            />
          </div>
        </div>

        <!-- Primary Musical Instrument -->
        <div class="form-group">
          <label for="member-primary-inst">
            {$tStore('show_mgmt.roster_modal.select_instrument')} *
          </label>
          <select id="member-primary-inst" bind:value={formPrimaryInst} class="form-input">
            {#each AVAILABLE_BAND_ROLES as role}
              <option value={role.id}>{$tStore(role.key)}</option>
            {/each}
          </select>
        </div>

        <!-- Secondary Musical Skills / Doubling Instruments -->
        <div class="form-group">
          <label class="section-label">Secondary / Backup Instruments</label>
          <div class="secondary-inst-selector">
            {#each AVAILABLE_BAND_ROLES as role}
              {#if role.id !== formPrimaryInst}
                <button
                  type="button"
                  class="sec-toggle-chip {formSecondaryInst.includes(role.id) ? 'selected' : ''}"
                  onclick={() => toggleSecondaryInstrument(role.id)}
                >
                  <svelte:component this={role.icon} size={12} />
                  <span>{$tStore(role.key)}</span>
                </button>
              {/if}
            {/each}
          </div>
        </div>

        <!-- Modal Actions -->
        <div class="modal-actions">
          <button
            type="button"
            class="bento-btn bento-btn-secondary"
            onclick={() => (isAddEditModalOpen = false)}
          >
            {$tStore('show_mgmt.roster_page.btn_cancel')}
          </button>
          <button type="submit" class="bento-btn bento-btn-primary">
            {$tStore('show_mgmt.roster_modal.btn_submit')}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Modal: Confirm Remove Member -->
{#if isRemoveModalOpen && memberToRemove}
  <div class="modal-backdrop" onclick={() => (isRemoveModalOpen = false)} role="presentation">
    <div class="modal-card bento-card" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="modal-header">
        <h3 class="text-red">{$tStore('show_mgmt.roster_page.confirm_remove_title')}</h3>
        <button type="button" class="close-modal-btn" onclick={() => (isRemoveModalOpen = false)}>
          <X size={18} />
        </button>
      </div>

      <div class="confirm-modal-body">
        <AlertTriangle size={32} class="text-red confirm-icon" />
        <p>
          {$tStore('show_mgmt.roster_page.confirm_remove_msg', { name: memberToRemove.fullName })}
        </p>
        <div class="impact-box">
          <span>Active Assignments: <strong>{memberToRemove.assignedSongCount} songs</strong></span>
          <span>Practice Effort: <strong>{memberToRemove.totalPracticeHours} hrs</strong></span>
        </div>
      </div>

      <div class="modal-actions">
        <button
          type="button"
          class="bento-btn bento-btn-secondary"
          onclick={() => (isRemoveModalOpen = false)}
        >
          {$tStore('show_mgmt.roster_page.btn_cancel')}
        </button>
        <button
          type="button"
          class="bento-btn bento-btn-danger"
          onclick={confirmRemoveMember}
        >
          {$tStore('show_mgmt.roster_page.btn_confirm_remove')}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .roster-hub {
    display: flex;
    flex-direction: column;
    gap: 16px;
    width: 100%;
  }

  .bento-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 16px;
    padding: 20px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
  }

  /* Header Bar */
  .header-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 16px;
  }

  .header-title-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .header-title-row h2 {
    font-size: 20px;
    font-weight: 800;
    color: #0f172a;
    margin: 0;
  }

  .member-count-badge {
    padding: 4px 10px;
    background: #f1f5f9;
    border-radius: 20px;
    font-size: 12px;
    font-weight: 700;
    color: #475569;
  }

  .header-desc {
    font-size: 13px;
    color: #64748b;
    margin: 6px 0 0 0;
  }

  .header-controls {
    display: flex;
    align-items: center;
    gap: 14px;
    flex-wrap: wrap;
  }

  .role-switch-box {
    display: flex;
    align-items: center;
    gap: 8px;
    background: #f8fafc;
    border: 1px solid #cbd5e1;
    border-radius: 10px;
    padding: 6px 12px;
  }

  .role-switch-label {
    font-size: 12px;
    font-weight: 700;
    color: #475569;
  }

  .role-select {
    border: none;
    background: transparent;
    font-size: 12px;
    font-weight: 700;
    color: #0f172a;
    outline: none;
    cursor: pointer;
  }

  /* Bento 4-Box Monitor Grid */
  .monitor-metrics-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 16px;
  }

  @media (max-width: 1024px) {
    .monitor-metrics-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 640px) {
    .monitor-metrics-grid {
      grid-template-columns: 1fr;
    }
  }

  .metric-bento-card {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 14px;
    padding: 16px;
  }

  .metric-top {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .metric-icon-wrap {
    width: 44px;
    height: 44px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .bg-orange-soft { background: rgba(255, 107, 0, 0.1); }
  .bg-blue-soft { background: rgba(59, 130, 246, 0.1); }
  .bg-red-soft { background: rgba(239, 68, 68, 0.1); }
  .bg-purple-soft { background: rgba(147, 51, 234, 0.1); }

  .text-orange { color: #ff6b00; }
  .text-blue { color: #2563eb; }
  .text-red { color: #ef4444; }
  .text-green { color: #16a34a; }
  .text-purple { color: #9333ea; }
  .text-muted { color: #94a3b8; }

  .metric-stat-group {
    display: flex;
    flex-direction: column;
  }

  .metric-num {
    font-size: 22px;
    font-weight: 800;
    color: #0f172a;
    line-height: 1.1;
  }

  .metric-title {
    font-size: 11px;
    font-weight: 600;
    color: #64748b;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .leadership-pills {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
  }

  .lead-pill {
    font-size: 11px;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 6px;
  }

  .dm-pill { background: rgba(255, 107, 0, 0.12); color: #ff6b00; }
  .pm-pill { background: rgba(59, 130, 246, 0.12); color: #2563eb; }
  .qc-pill { background: rgba(147, 51, 234, 0.12); color: #9333ea; }
  .perf-pill { background: #f1f5f9; color: #475569; }

  .coverage-bar-group {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 6px;
    font-size: 11px;
    color: #475569;
  }

  .coverage-item {
    display: flex;
    align-items: center;
    gap: 4px;
    background: #f8fafc;
    padding: 4px 8px;
    border-radius: 6px;
  }

  .workload-ratio-strip {
    display: flex;
    gap: 6px;
  }

  .ratio-bubble {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 4px 6px;
    border-radius: 8px;
  }

  .green-bubble { background: rgba(22, 163, 74, 0.1); color: #16a34a; }
  .yellow-bubble { background: rgba(217, 119, 6, 0.1); color: #d97706; }
  .red-bubble { background: rgba(239, 68, 68, 0.1); color: #ef4444; }

  .ratio-val { font-size: 14px; font-weight: 800; }
  .ratio-lbl { font-size: 9px; font-weight: 700; }

  .effort-subtext {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: #475569;
    background: #f8fafc;
    padding: 6px 10px;
    border-radius: 8px;
  }

  /* Toolbar */
  .toolbar-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 12px 18px;
    flex-wrap: wrap;
  }

  .search-box {
    position: relative;
    display: flex;
    align-items: center;
    min-width: 260px;
    flex: 1;
  }

  .search-icon {
    position: absolute;
    left: 12px;
    color: #94a3b8;
  }

  .search-input {
    width: 100%;
    padding: 8px 34px 8px 36px;
    border: 1px solid #cbd5e1;
    border-radius: 8px;
    font-size: 13px;
    outline: none;
    transition: border-color 0.15s ease;
  }

  .search-input:focus {
    border-color: #ff6b00;
  }

  .clear-search-btn {
    position: absolute;
    right: 10px;
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    display: flex;
    align-items: center;
  }

  .filter-chips {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
  }

  .filter-chip {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 6px 12px;
    border-radius: 8px;
    border: 1px solid #e2e8f0;
    background: #ffffff;
    font-size: 12px;
    font-weight: 600;
    color: #475569;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .filter-chip:hover {
    background: #f8fafc;
    border-color: #cbd5e1;
  }

  .filter-chip.active {
    background: #0f172a;
    color: #ffffff;
    border-color: #0f172a;
  }

  .view-mode-toggle {
    display: flex;
    background: #f1f5f9;
    padding: 3px;
    border-radius: 8px;
  }

  .view-btn {
    padding: 6px 10px;
    border: none;
    background: transparent;
    border-radius: 6px;
    color: #64748b;
    cursor: pointer;
    display: flex;
    align-items: center;
    transition: all 0.15s ease;
  }

  .view-btn.active {
    background: #ffffff;
    color: #ff6b00;
    box-shadow: 0 1px 2px rgba(0,0,0,0.05);
  }

  /* Roster Bento Grid */
  .roster-bento-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 16px;
  }

  .member-card {
    display: flex;
    flex-direction: column;
    gap: 12px;
    transition: transform 0.15s ease, box-shadow 0.15s ease;
  }

  .member-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0,0,0,0.06);
  }

  .member-card-header {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .avatar-circle {
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
    border: 2px solid #ff6b00;
    color: #0f172a;
    font-size: 18px;
    font-weight: 800;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .member-info-col {
    flex: 1;
    min-width: 0;
  }

  .name-role-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .member-name {
    font-size: 15px;
    font-weight: 700;
    color: #0f172a;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .role-badge {
    padding: 3px 8px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 800;
  }

  .role-badge-dm { background: rgba(255, 107, 0, 0.15); color: #ff6b00; }
  .role-badge-pm { background: rgba(59, 130, 246, 0.15); color: #2563eb; }
  .role-badge-qc { background: rgba(147, 51, 234, 0.15); color: #9333ea; }
  .role-badge-performer { background: #f1f5f9; color: #64748b; }

  .contact-subline {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 11px;
    color: #64748b;
    margin-top: 3px;
    flex-wrap: wrap;
  }

  .contact-item {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .instrument-tag-row {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
  }

  .primary-inst-chip {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    background: #fff7ed;
    border: 1px solid #ffedd5;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 700;
    color: #c2410c;
  }

  .sec-inst-chip {
    padding: 4px 8px;
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    border-radius: 6px;
    font-size: 11px;
    color: #475569;
  }

  .songs-assigned-box {
    background: #f8fafc;
    border-radius: 8px;
    padding: 10px;
    font-size: 12px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .songs-assigned-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 11px;
    color: #475569;
  }

  .attendance-tag {
    font-weight: 700;
    color: #16a34a;
  }

  .song-chips-wrap {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
  }

  .song-chip {
    padding: 2px 6px;
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 4px;
    font-size: 11px;
    color: #334155;
  }

  .effort-workload-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-top: 4px;
    border-top: 1px solid #f1f5f9;
  }

  .effort-stat {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: #475569;
  }

  .workload-pill {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 3px 8px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 700;
  }

  .workload-green { background: rgba(22, 163, 74, 0.1); color: #16a34a; }
  .workload-yellow { background: rgba(217, 119, 6, 0.1); color: #d97706; }
  .workload-red { background: rgba(239, 68, 68, 0.1); color: #ef4444; }

  .card-actions-bar {
    display: flex;
    gap: 8px;
    margin-top: 4px;
  }

  .action-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 7px 12px;
    border-radius: 8px;
    font-size: 12px;
    font-weight: 600;
    border: 1px solid #e2e8f0;
    background: #ffffff;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .edit-btn:hover {
    background: #f8fafc;
    border-color: #cbd5e1;
    color: #0f172a;
  }

  .remove-btn {
    color: #ef4444;
    border-color: #fecaca;
    background: #fef2f2;
  }

  .remove-btn:hover {
    background: #fee2e2;
  }

  /* Matrix Table View */
  .matrix-table-card {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .matrix-card-header h3 {
    font-size: 16px;
    font-weight: 800;
    color: #0f172a;
    margin: 0;
  }

  .matrix-card-header p {
    font-size: 12px;
    color: #64748b;
    margin: 4px 0 0 0;
  }

  .table-responsive {
    overflow-x: auto;
  }

  .matrix-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
  }

  .matrix-table th {
    text-align: left;
    padding: 10px 14px;
    background: #f8fafc;
    color: #475569;
    font-weight: 700;
    font-size: 12px;
    border-bottom: 2px solid #e2e8f0;
  }

  .matrix-table td {
    padding: 12px 14px;
    border-bottom: 1px solid #f1f5f9;
    vertical-align: middle;
  }

  .table-member-cell {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .table-avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: #f1f5f9;
    font-weight: 800;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #0f172a;
  }

  .table-member-name {
    font-weight: 700;
    color: #0f172a;
  }

  .table-member-email {
    font-size: 11px;
    color: #64748b;
  }

  .role-badge-sm {
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 800;
  }

  .table-primary-chip {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 8px;
    background: #fff7ed;
    border-radius: 6px;
    font-weight: 600;
    color: #c2410c;
  }

  .table-sec-chips {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
  }

  .sec-mini-tag {
    padding: 2px 6px;
    background: #f1f5f9;
    border-radius: 4px;
    font-size: 11px;
    color: #475569;
  }

  .songs-preview-text {
    font-size: 11px;
    color: #64748b;
  }

  .hours-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    color: #0f172a;
  }

  .attendance-num {
    font-weight: 700;
    color: #16a34a;
  }

  .table-workload-tag {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 3px 8px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 700;
  }

  .table-actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .icon-btn {
    padding: 6px;
    border-radius: 6px;
    border: 1px solid #e2e8f0;
    background: #ffffff;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
  }

  .edit-icon:hover {
    background: #f8fafc;
    color: #0f172a;
  }

  .remove-icon {
    color: #ef4444;
    border-color: #fecaca;
  }

  .remove-icon:hover {
    background: #fee2e2;
  }

  /* Empty State */
  .empty-state {
    grid-column: 1 / -1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 48px 20px;
    text-align: center;
    gap: 12px;
    color: #64748b;
  }

  /* Modal Styles */
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 23, 42, 0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 16px;
  }

  .modal-card {
    width: 100%;
    max-width: 520px;
    max-height: 90vh;
    overflow-y: auto;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    border-bottom: 1px solid #e2e8f0;
    padding-bottom: 12px;
  }

  .modal-header h3 {
    font-size: 18px;
    font-weight: 800;
    margin: 0;
    color: #0f172a;
  }

  .close-modal-btn {
    border: none;
    background: transparent;
    color: #94a3b8;
    cursor: pointer;
  }

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-group label {
    font-size: 12px;
    font-weight: 700;
    color: #334155;
  }

  .form-input {
    padding: 8px 12px;
    border: 1px solid #cbd5e1;
    border-radius: 8px;
    font-size: 13px;
    outline: none;
  }

  .form-input:focus {
    border-color: #ff6b00;
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }

  .secondary-inst-selector {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .sec-toggle-chip {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 6px 10px;
    border-radius: 6px;
    border: 1px solid #e2e8f0;
    background: #ffffff;
    font-size: 11px;
    color: #475569;
    cursor: pointer;
  }

  .sec-toggle-chip.selected {
    background: #fff7ed;
    border-color: #ffedd5;
    color: #c2410c;
    font-weight: 700;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 14px;
    border-top: 1px solid #e2e8f0;
    padding-top: 14px;
  }

  .bento-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 700;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all 0.15s ease;
  }

  .bento-btn-primary {
    background: #ff6b00;
    color: #ffffff;
  }

  .bento-btn-primary:hover {
    background: #ea580c;
  }

  .bento-btn-secondary {
    background: #f1f5f9;
    color: #475569;
    border-color: #e2e8f0;
  }

  .bento-btn-danger {
    background: #ef4444;
    color: #ffffff;
  }

  .confirm-modal-body {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 12px;
    padding: 12px 0;
  }

  .confirm-icon {
    margin-bottom: 4px;
  }

  .impact-box {
    display: flex;
    gap: 16px;
    background: #f8fafc;
    padding: 8px 14px;
    border-radius: 8px;
    font-size: 12px;
    color: #475569;
  }
</style>
