<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    Music,
    CheckCircle2,
    Clock,
    AlertCircle,
    UserCheck,
    Users,
    MessageSquare,
    Plus,
    X,
    LayoutGrid,
    Kanban,
    Table,
    Search,
    Filter,
    Sparkles,
    ChevronRight,
    SlidersHorizontal,
    Activity,
    Layers,
    Mic2,
    Guitar,
    Disc3,
  } from '@lucide/svelte';

  import { page } from '$app/stores';
  import {
    canManageNumbers,
    canReviewQC,
    canManageShowScoped,
    canManageSongScoped,
    canAuditSongScoped,
    getEffectiveRole,
    hasRole,
  } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';

  export interface SongNumber {
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

  const showId = $derived($page.params.id || 'show-2026-annual');
  const userRole = $derived(($page.data?.user?.role || 'admin') as UserRole);
  const currentUserName = $derived($page.data?.user?.fullName || 'Administrator');

  // Realistic CSAC Annual Concert production dataset (14 numbers scaling across all 5 stages)
  let numbers = $state<SongNumber[]>([
    {
      id: 'num-1',
      title: 'Hào Khí Việt Nam',
      genre: 'Epic Symphony Rock',
      pmName: 'Minh Pháp',
      stage: 'stage_ready',
      qcReviewer: 'Hoàng Nam',
      qcNotes: 'Flawless vocal harmonies and drum fills. Stage ready.',
      lineup: { vocalLead: 'Minh Pháp', guitarLead: 'Hoàng Nam', bass: 'Bảo Anh', drums: 'Thu Hà' },
    },
    {
      id: 'num-2',
      title: 'Đi Giữa Trời Rực Rỡ',
      genre: 'Pop Rock',
      pmName: 'Hoàng Nam',
      stage: 'qc_approved',
      qcReviewer: 'Thu Hà',
      qcNotes: 'Lead guitar solo approved. Dynamic balance is balanced.',
      lineup: { vocalLead: 'Gia Huy', guitarLead: 'Hoàng Nam', bass: 'Bảo Anh', drums: 'Thu Hà' },
    },
    {
      id: 'num-3',
      title: 'Giọt Sương Trên Mí Mắt',
      genre: 'Acoustic Quartet',
      pmName: 'Bảo Anh',
      stage: 'ready_for_qc',
      qcReviewer: 'Minh Pháp',
      lineup: { vocalLead: 'Minh Pháp', guitarLead: 'Tùng Dương', bass: 'Bảo Anh' },
    },
    {
      id: 'num-4',
      title: 'Nối Vòng Tay Lớn',
      genre: 'Choral Folk Rock',
      pmName: 'Thu Hà',
      stage: 'in_practice',
      qcReviewer: 'Bảo Anh',
      qcNotes: 'Need tighter drum transitions in Chorus 2.',
      lineup: { vocalLead: 'Anh Pha', guitarLead: 'Hoàng Nam', bass: 'Bảo Anh', drums: 'Thu Hà' },
    },
    {
      id: 'num-5',
      title: 'Túy Âm',
      genre: 'Future Bass Rock Fusion',
      pmName: 'Gia Huy',
      stage: 'stage_ready',
      qcReviewer: 'Minh Pháp',
      qcNotes: 'Synthesizer pads and bass groove calibrated perfectly.',
      lineup: { vocalLead: 'Gia Huy', bass: 'Bảo Anh', keys: 'Phương Nhi', drums: 'Thu Hà' },
    },
    {
      id: 'num-6',
      title: 'Để Mị Nói Cho Mà Nghe',
      genre: 'Ethnic Pop Punk',
      pmName: 'Phương Nhi',
      stage: 'qc_approved',
      qcReviewer: 'Thu Hà',
      qcNotes: 'Flute & keyboard blend sounds crisp.',
      lineup: { vocalLead: 'Phương Nhi', guitarLead: 'Hoàng Nam', bass: 'Bảo Anh' },
    },
    {
      id: 'num-7',
      title: 'Bài Ca Hy Vọng',
      genre: 'Chamber Vocal Ensemble',
      pmName: 'Minh Pháp',
      stage: 'ready_for_qc',
      qcReviewer: 'Hoàng Nam',
      lineup: { vocalLead: 'Minh Pháp', keys: 'Phương Nhi' },
    },
    {
      id: 'num-8',
      title: 'Ngẫu Hứng Sông Hồng',
      genre: 'Progressive Folk Rock',
      pmName: 'Hoàng Nam',
      stage: 'in_practice',
      qcReviewer: 'Minh Pháp',
      lineup: { vocalLead: 'Anh Pha', guitarLead: 'Hoàng Nam', drums: 'Thu Hà' },
    },
    {
      id: 'num-9',
      title: 'Góc Ban Công',
      genre: 'Indie Pop Ballad',
      pmName: 'Bảo Anh',
      stage: 'in_practice',
      qcReviewer: 'Thu Hà',
      lineup: { vocalLead: 'Bảo Anh', guitarLead: 'Tùng Dương' },
    },
    {
      id: 'num-10',
      title: 'Mặt Trời Bé Con',
      genre: 'Acoustic Duo',
      pmName: 'Tùng Dương',
      stage: 'stage_ready',
      qcReviewer: 'Bảo Anh',
      qcNotes: 'Acoustic fingerstyle guitar approved for stage soundcheck.',
      lineup: { vocalLead: 'Thu Hà', guitarLead: 'Tùng Dương' },
    },
    {
      id: 'num-11',
      title: 'Tháng Mười Hai',
      genre: 'Alternative Rock',
      pmName: 'Gia Huy',
      stage: 'draft',
      qcReviewer: 'Hoàng Nam',
      lineup: { vocalLead: 'Gia Huy', guitarLead: 'Hoàng Nam' },
    },
    {
      id: 'num-12',
      title: 'Đất Nước Trọn Niềm Vui',
      genre: 'Orchestral Overture',
      pmName: 'Minh Pháp',
      stage: 'ready_for_qc',
      qcReviewer: 'Thu Hà',
      lineup: { vocalLead: 'Minh Pháp', keys: 'Phương Nhi', drums: 'Thu Hà' },
    },
    {
      id: 'num-13',
      title: 'Khát Vọng Tuổi Trẻ',
      genre: 'Youth Anthem Pop',
      pmName: 'Anh Pha',
      stage: 'draft',
      qcReviewer: 'Minh Pháp',
      lineup: { vocalLead: 'Anh Pha' },
    },
    {
      id: 'num-14',
      title: 'Khoảnh Khắc',
      genre: 'Acoustic Soul',
      pmName: 'Thu Hà',
      stage: 'in_practice',
      qcReviewer: 'Gia Huy',
      lineup: { vocalLead: 'Thu Hà', guitarLead: 'Tùng Dương', bass: 'Bảo Anh' },
    },
  ]);

  let availableRoster = [
    'Minh Pháp',
    'Hoàng Nam',
    'Bảo Anh',
    'Thu Hà',
    'Gia Huy',
    'Anh Pha',
    'Phương Nhi',
    'Tùng Dương',
  ];
  // View Mode: 'grid' | 'kanban' | 'table'
  let currentView = $state<'grid' | 'kanban' | 'table'>(
    ($page.url.searchParams.get('view') as 'grid' | 'kanban' | 'table') || 'grid'
  );

  // Search & Filter State
  let searchQuery = $state($page.url.searchParams.get('q') || '');
  let selectedStageFilter = $state<string>($page.url.searchParams.get('stage') || 'all');
  let selectedPmFilter = $state<string>('all');

  // Modal States
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
  let newPm = $state('Minh Pháp');
  let newQcReviewer = $state('Hoàng Nam');

  // Derived Pipeline Funnel Metrics
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

  // Filtered Numbers based on query and dropdowns
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

  function handleLineupSubmit(e: Event) {
    e.preventDefault();
    if (!activeSongForLineup) return;

    numbers = numbers.map((n) =>
      n.id === activeSongForLineup?.id
        ? {
            ...n,
            lineup: {
              vocalLead: formVocalLead || undefined,
              guitarLead: formGuitarLead || undefined,
              bass: formBass || undefined,
              drums: formDrums || undefined,
              keys: formKeys || undefined,
            },
          }
        : n
    );
    isLineupDrawerOpen = false;
  }

  function handleQcSubmit(e: Event) {
    e.preventDefault();
    if (!activeSongForQc) return;

    numbers = numbers.map((n) =>
      n.id === activeSongForQc?.id
        ? {
            ...n,
            stage: qcVerdict === 'pass' ? 'qc_approved' : 'in_practice',
            qcNotes: qcNotesInput,
          }
        : n
    );

    isQcDrawerOpen = false;
    activeSongForQc = null;
  }

  function advanceStatus(song: SongNumber, nextStage: SongNumber['stage']) {
    numbers = numbers.map((n) => (n.id === song.id ? { ...n, stage: nextStage } : n));
  }

  function openAddModal() {
    newTitle = '';
    newGenre = '';
    newPm = availableRoster[0] || 'Minh Pháp';
    newQcReviewer = availableRoster[1] || 'Hoàng Nam';
    isAddModalOpen = true;
  }

  function handleCreateNumber(e: Event) {
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
  }

  function getStageBadgeClass(stage: SongNumber['stage']) {
    switch (stage) {
      case 'draft':
        return 'badge-draft';
      case 'in_practice':
        return 'badge-practice';
      case 'ready_for_qc':
        return 'badge-ready-qc';
      case 'qc_approved':
        return 'badge-qc-approved';
      case 'stage_ready':
        return 'badge-stage-ready';
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

<div class="numbers-subpage">
  <!-- Pipeline Funnel Summary Banner (1-Click Stage Filter) -->
  <div class="pipeline-summary bento-card">
    <div class="funnel-header">
      <div class="funnel-title-row">
        <Activity size={18} class="text-orange" />
        <span class="funnel-heading">{$tStore('studio_shows.pipeline_heading')}</span>
        <span class="funnel-sub">{$tStore('studio_shows.pipeline_sub').replace('{count}', numbers.length.toString())}</span>
      </div>

      {#if canManageNumbers(userRole)}
        <button
          type="button"
          class="bento-btn bento-btn-primary"
          onclick={openAddModal}
          id="btn-add-music-number"
        >
          <Plus size={16} />
          <span>{$tStore('studio_shows.btn_add_number')}</span>
        </button>
      {/if}
    </div>

    <div class="funnel-tiles">
      <button
        type="button"
        class="funnel-tile {selectedStageFilter === 'all' ? 'is-active' : ''}"
        onclick={() => (selectedStageFilter = 'all')}
      >
        <span class="tile-count">{stageStats.total}</span>
        <span class="tile-label">{$tStore('studio_shows.pipeline_all')}</span>
      </button>

      <button
        type="button"
        class="funnel-tile {selectedStageFilter === 'draft' ? 'is-active' : ''}"
        onclick={() => (selectedStageFilter = 'draft')}
      >
        <span class="tile-count text-slate">{stageStats.draft}</span>
        <span class="tile-label">{$tStore('studio_shows.kanban_draft')}</span>
      </button>

      <button
        type="button"
        class="funnel-tile {selectedStageFilter === 'in_practice' ? 'is-active' : ''}"
        onclick={() => (selectedStageFilter = 'in_practice')}
      >
        <span class="tile-count text-blue">{stageStats.in_practice}</span>
        <span class="tile-label">{$tStore('studio_shows.kanban_practice')}</span>
      </button>

      <button
        type="button"
        class="funnel-tile {selectedStageFilter === 'ready_for_qc' ? 'is-active' : ''}"
        onclick={() => (selectedStageFilter = 'ready_for_qc')}
      >
        <span class="tile-count text-orange">{stageStats.ready_for_qc}</span>
        <span class="tile-label">{$tStore('studio_shows.kanban_ready_qc')}</span>
      </button>

      <button
        type="button"
        class="funnel-tile {selectedStageFilter === 'qc_approved' ? 'is-active' : ''}"
        onclick={() => (selectedStageFilter = 'qc_approved')}
      >
        <span class="tile-count text-indigo">{stageStats.qc_approved}</span>
        <span class="tile-label">{$tStore('studio_shows.kanban_qc_approved')}</span>
      </button>

      <button
        type="button"
        class="funnel-tile {selectedStageFilter === 'stage_ready' ? 'is-active' : ''}"
        onclick={() => (selectedStageFilter = 'stage_ready')}
      >
        <span class="tile-count text-green">{stageStats.stage_ready}</span>
        <span class="tile-label">{$tStore('studio_shows.kanban_stage_ready')}</span>
      </button>
    </div>
  </div>

  <!-- Search, Filter & View Controls Toolbar -->
  <div class="toolbar-card bento-card">
    <div class="search-box">
      <Search size={16} class="search-icon" />
      <input
        type="text"
        placeholder={$tStore('studio_shows.search_placeholder')}
        bind:value={searchQuery}
        class="search-input"
        id="input-search-numbers"
      />
      {#if searchQuery}
        <button
          type="button"
          class="clear-search-btn"
          onclick={() => (searchQuery = '')}
          aria-label="Clear Search"
        >
          <X size={14} />
        </button>
      {/if}
    </div>

    <div class="filters-row">
      <div class="select-wrapper">
        <Filter size={14} class="select-icon" />
        <select bind:value={selectedStageFilter} class="filter-select" id="select-stage-filter">
          <option value="all">{$tStore('studio_shows.filter_all_stages')} ({numbers.length})</option>
          <option value="draft">{$tStore('studio_shows.kanban_draft')} ({stageStats.draft})</option>
          <option value="in_practice">{$tStore('studio_shows.kanban_practice')} ({stageStats.in_practice})</option>
          <option value="ready_for_qc">{$tStore('studio_shows.kanban_ready_qc')} ({stageStats.ready_for_qc})</option>
          <option value="qc_approved">{$tStore('studio_shows.kanban_qc_approved')} ({stageStats.qc_approved})</option>
          <option value="stage_ready">{$tStore('studio_shows.kanban_stage_ready')} ({stageStats.stage_ready})</option>
        </select>
      </div>

      <div class="select-wrapper">
        <Users size={14} class="select-icon" />
        <select bind:value={selectedPmFilter} class="filter-select" id="select-pm-filter">
          <option value="all">{$tStore('studio_shows.filter_all_pms')}</option>
          {#each uniquePms as pm}
            <option value={pm}>{pm}</option>
          {/each}
        </select>
      </div>

      <!-- View Switcher -->
      <div class="view-switcher" role="group" aria-label="View Switcher">
        <button
          type="button"
          class="view-btn {currentView === 'grid' ? 'is-active' : ''}"
          onclick={() => (currentView = 'grid')}
          id="btn-view-grid"
          title={$tStore('studio_shows.view_grid')}
        >
          <LayoutGrid size={15} />
          <span class="view-btn-text">{$tStore('studio_shows.view_grid')}</span>
        </button>

        <button
          type="button"
          class="view-btn {currentView === 'kanban' ? 'is-active' : ''}"
          onclick={() => (currentView = 'kanban')}
          id="btn-view-kanban"
          title={$tStore('studio_shows.view_kanban')}
        >
          <Kanban size={15} />
          <span class="view-btn-text">{$tStore('studio_shows.view_kanban')}</span>
        </button>

        <button
          type="button"
          class="view-btn {currentView === 'table' ? 'is-active' : ''}"
          onclick={() => (currentView = 'table')}
          id="btn-view-table"
          title={$tStore('studio_shows.view_table')}
        >
          <Table size={15} />
          <span class="view-btn-text">{$tStore('studio_shows.view_table')}</span>
        </button>
      </div>
    </div>
  </div>

  <!-- Empty State when filters produce zero matches -->
  {#if filteredNumbers.length === 0}
    <div class="empty-state bento-card">
      <Music size={42} class="empty-icon text-muted" />
      <h3 class="empty-title">{$tStore('studio_shows.empty_title')}</h3>
      <p class="empty-desc">{$tStore('studio_shows.empty_desc')}</p>
      <button
        type="button"
        class="bento-btn bento-btn-sm"
        onclick={() => {
          searchQuery = '';
          selectedStageFilter = 'all';
          selectedPmFilter = 'all';
        }}
      >
        {$tStore('studio_shows.btn_reset_filters')}
      </button>
    </div>
  {:else if currentView === 'grid'}
    <!-- VIEW 1: SCALABLE BENTO GRID (DEFAULT) -->
    <div class="numbers-bento-grid">
      {#each filteredNumbers as song, index (song.id)}
        <div class="song-bento-card bento-card {song.stage === 'stage_ready' ? 'highlight-stage-ready' : ''}">
          <div class="card-top-row">
            <div class="order-badge">#{index + 1}</div>
            <span class="stage-pill {getStageBadgeClass(song.stage)}">
              {getStageLabel(song.stage)}
            </span>
          </div>

          <div class="card-main-info">
            <h3 class="song-title">{song.title}</h3>
            <div class="genre-tag">{song.genre}</div>
            <div class="personnel-row">
              <span class="pm-badge"><strong>PM:</strong> {song.pmName}</span>
              <span class="reviewer-badge"><strong>QC:</strong> {song.qcReviewer}</span>
            </div>
          </div>

          <!-- Lineup Section -->
          <div class="lineup-section">
            <div class="lineup-label">Band Allocation:</div>
            <div class="lineup-tags">
              {#if song.lineup?.vocalLead}
                <span class="lineup-tag vocal">
                  <Mic2 size={10} /> {song.lineup.vocalLead}
                </span>
              {/if}
              {#if song.lineup?.guitarLead}
                <span class="lineup-tag guitar">
                  <Guitar size={10} /> {song.lineup.guitarLead}
                </span>
              {/if}
              {#if song.lineup?.bass}
                <span class="lineup-tag bass">
                  <Disc3 size={10} /> {song.lineup.bass}
                </span>
              {/if}
              {#if song.lineup?.drums}
                <span class="lineup-tag drums">
                  🥁 {song.lineup.drums}
                </span>
              {/if}
              {#if song.lineup?.keys}
                <span class="lineup-tag keys">
                  🎹 {song.lineup.keys}
                </span>
              {/if}
              {#if !song.lineup?.vocalLead && !song.lineup?.guitarLead && !song.lineup?.bass && !song.lineup?.drums && !song.lineup?.keys}
                <span class="lineup-tag unassigned">Lineup Unassigned</span>
              {/if}
            </div>
          </div>

          <!-- QC Notes Box -->
          {#if song.qcNotes}
            <div class="qc-notes-box {song.stage === 'ready_for_qc' ? 'warning' : ''}">
              <MessageSquare size={12} class="qc-note-icon" />
              <span class="qc-note-text">{song.qcNotes}</span>
            </div>
          {/if}

          <!-- Responsive Action Buttons Row (Key-Scoped Roles) -->
          <div class="card-btn-row">
            <!-- Cross-Screen Link to Sprint Timetable -->
            <a
              href="/studio/shows/{showId}/sprints?song={encodeURIComponent(song.title)}"
              class="bento-btn bento-btn-sm sprint-link-btn"
              title="View all scheduled rehearsal sessions in Sprint Calendar"
            >
              <Clock size={12} />
              <span>Sprint Schedule</span>
            </a>

            {#if canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
              <button
                type="button"
                class="bento-btn bento-btn-sm lineup-btn"
                onclick={() => openLineupDrawer(song)}
              >
                <Users size={12} />
                <span>Assign Lineup</span>
              </button>

              {#if song.stage === 'draft'}
                <button
                  type="button"
                  class="bento-btn bento-btn-sm action-btn-blue"
                  onclick={() => advanceStatus(song, 'in_practice')}
                >
                  <span>Start Practice</span>
                </button>
              {:else if song.stage === 'in_practice'}
                <button
                  type="button"
                  class="bento-btn bento-btn-sm action-btn-orange"
                  onclick={() => advanceStatus(song, 'ready_for_qc')}
                >
                  <span>Submit for QC</span>
                </button>
              {/if}
            {/if}

            {#if song.stage === 'ready_for_qc' && canAuditSongScoped(userRole, false, song.qcReviewer === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
              <button
                type="button"
                class="bento-btn bento-btn-sm qc-btn"
                onclick={() => openQcDrawer(song)}
              >
                <UserCheck size={13} />
                <span>Audit & Submit QC</span>
              </button>
            {/if}

            {#if song.stage === 'qc_approved' && canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
              <button
                type="button"
                class="bento-btn bento-btn-sm action-btn-green"
                onclick={() => advanceStatus(song, 'stage_ready')}
              >
                <CheckCircle2 size={12} />
                <span>Promote to Stage Ready</span>
              </button>
            {/if}
          </div>

          {#if song.stage === 'stage_ready'}
            <div class="stage-ready-indicator">
              <CheckCircle2 size={14} class="text-green" />
              <span>100% Stage Ready</span>
            </div>
          {/if}
        </div>
      {/each}
    </div>

  {:else if currentView === 'kanban'}
    <!-- VIEW 2: REFINED KANBAN BOARD -->
    <div class="kanban-scroll-wrapper">
      <div class="kanban-board">
        <!-- Column 1: Draft -->
        <div class="kanban-col bento-card">
          <div class="col-header">
            <span class="col-title" style="color: #64748b">{$tStore('show_mgmt.status.draft')}</span>
            <span class="col-count">{filteredNumbers.filter((n) => n.stage === 'draft').length}</span>
          </div>
          <div class="col-cards">
            {#each filteredNumbers.filter((n) => n.stage === 'draft') as song (song.id)}
              <div class="song-card bento-card">
                <h4 class="song-title">{song.title}</h4>
                <div class="song-pm">Leader (PM): {song.pmName}</div>
                <div class="genre-micro">{song.genre}</div>
                {#if canManageNumbers(userRole)}
                  <div class="card-btn-row">
                    <button
                      type="button"
                      class="bento-btn bento-btn-sm lineup-btn"
                      onclick={() => openLineupDrawer(song)}
                    >
                      <Users size={12} />
                      <span>Assign Lineup</span>
                    </button>
                    <button
                      type="button"
                      class="bento-btn bento-btn-sm action-btn-blue"
                      onclick={() => advanceStatus(song, 'in_practice')}
                    >
                      <span>Start Practice</span>
                    </button>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        </div>

        <!-- Column 2: In Practice -->
        <div class="kanban-col bento-card">
          <div class="col-header">
            <span class="col-title text-blue">{$tStore('studio_shows.kanban_practice')}</span>
            <span class="col-count">{filteredNumbers.filter((n) => n.stage === 'in_practice').length}</span>
          </div>
          <div class="col-cards">
            {#each filteredNumbers.filter((n) => n.stage === 'in_practice') as song (song.id)}
              <div class="song-card bento-card">
                <h4 class="song-title">{song.title}</h4>
                <div class="song-pm">Leader (PM): {song.pmName}</div>
                <div class="song-qc-meta">QC Reviewer: {song.qcReviewer}</div>

                {#if song.lineup}
                  <div class="lineup-tags">
                    {#if song.lineup.vocalLead}<span class="lineup-tag vocal">Vocal: {song.lineup.vocalLead}</span>{/if}
                    {#if song.lineup.guitarLead}<span class="lineup-tag guitar">Guitar: {song.lineup.guitarLead}</span>{/if}
                    {#if song.lineup.bass}<span class="lineup-tag bass">Bass: {song.lineup.bass}</span>{/if}
                    {#if song.lineup.drums}<span class="lineup-tag drums">Drums: {song.lineup.drums}</span>{/if}
                  </div>
                {/if}

                {#if song.qcNotes}
                  <div class="qc-notes-box warning"><MessageSquare size={12} /> {song.qcNotes}</div>
                {/if}

                {#if canManageNumbers(userRole)}
                  <div class="card-btn-row">
                    <button
                      type="button"
                      class="bento-btn bento-btn-sm lineup-btn"
                      onclick={() => openLineupDrawer(song)}
                    >
                      <Users size={12} />
                      <span>Assign Lineup</span>
                    </button>
                    <button
                      type="button"
                      class="bento-btn bento-btn-sm action-btn-orange"
                      onclick={() => advanceStatus(song, 'ready_for_qc')}
                    >
                      <span>Submit for QC</span>
                    </button>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        </div>

        <!-- Column 3: Ready for QC -->
        <div class="kanban-col bento-card">
          <div class="col-header">
            <span class="col-title text-orange">{$tStore('studio_shows.kanban_ready_qc')}</span>
            <span class="col-count">{filteredNumbers.filter((n) => n.stage === 'ready_for_qc').length}</span>
          </div>
          <div class="col-cards">
            {#each filteredNumbers.filter((n) => n.stage === 'ready_for_qc') as song (song.id)}
              <div class="song-card bento-card">
                <h4 class="song-title">{song.title}</h4>
                <div class="song-pm">Leader (PM): {song.pmName}</div>
                <div class="song-qc-meta">QC Reviewer: {song.qcReviewer}</div>
                <div class="card-btn-row">
                  <button
                    type="button"
                    class="bento-btn bento-btn-sm qc-btn"
                    onclick={() => openQcDrawer(song)}
                  >
                    <UserCheck size={13} />
                    <span>Audit & Submit QC</span>
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- Column 4: QC Approved -->
        <div class="kanban-col bento-card">
          <div class="col-header">
            <span class="col-title text-indigo">{$tStore('studio_shows.kanban_qc_approved')}</span>
            <span class="col-count">{filteredNumbers.filter((n) => n.stage === 'qc_approved').length}</span>
          </div>
          <div class="col-cards">
            {#each filteredNumbers.filter((n) => n.stage === 'qc_approved') as song (song.id)}
              <div class="song-card bento-card">
                <h4 class="song-title">{song.title}</h4>
                <div class="song-pm">Leader (PM): {song.pmName}</div>
                <div class="song-qc-meta">QC Reviewer: {song.qcReviewer}</div>
                {#if song.qcNotes}
                  <div class="qc-notes-box"><MessageSquare size={12} /> {song.qcNotes}</div>
                {/if}
                <div class="card-btn-row">
                  <button
                    type="button"
                    class="bento-btn bento-btn-sm action-btn-green"
                    onclick={() => advanceStatus(song, 'stage_ready')}
                  >
                    <span>Promote to Stage Ready</span>
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- Column 5: Stage Ready -->
        <div class="kanban-col bento-card">
          <div class="col-header">
            <span class="col-title text-green">{$tStore('studio_shows.kanban_stage_ready')}</span>
            <span class="col-count">{filteredNumbers.filter((n) => n.stage === 'stage_ready').length}</span>
          </div>
          <div class="col-cards">
            {#each filteredNumbers.filter((n) => n.stage === 'stage_ready') as song (song.id)}
              <div class="song-card bento-card highlight-green">
                <h4 class="song-title">{song.title}</h4>
                <div class="song-pm">Leader (PM): {song.pmName}</div>
                <div class="song-qc-meta">QC Reviewer: {song.qcReviewer}</div>
                {#if song.qcNotes}
                  <div class="qc-notes-box"><MessageSquare size={12} /> {song.qcNotes}</div>
                {/if}
                <div class="card-btn-row">
                  <button
                    type="button"
                    class="bento-btn bento-btn-sm lineup-btn"
                    onclick={() => openLineupDrawer(song)}
                  >
                    <Users size={12} />
                    <span>Assign Lineup</span>
                  </button>
                  <div class="stage-ready-indicator">
                    <CheckCircle2 size={13} class="text-green" />
                    <span>Stage Ready</span>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>
    </div>

  {:else if currentView === 'table'}
    <!-- VIEW 3: COMPACT PRODUCTION TABLE -->
    <div class="table-container bento-card">
      <table class="production-table">
        <thead>
          <tr>
            <th class="col-th-order">{$tStore('studio_shows.th_order')}</th>
            <th>{$tStore('studio_shows.th_song')}</th>
            <th>{$tStore('studio_shows.th_genre')}</th>
            <th>{$tStore('studio_shows.th_pm')}</th>
            <th>{$tStore('studio_shows.th_lineup')}</th>
            <th>{$tStore('studio_shows.th_stage')}</th>
            <th>{$tStore('studio_shows.th_actions')}</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredNumbers as song, idx (song.id)}
            <tr class="table-row {song.stage === 'stage_ready' ? 'row-stage-ready' : ''}">
              <td class="col-td-order">#{idx + 1}</td>
              <td class="col-td-song">
                <span class="row-song-title">{song.title}</span>
                {#if song.qcNotes}
                  <span class="row-qc-note" title={song.qcNotes}>
                    <MessageSquare size={11} /> {song.qcNotes}
                  </span>
                {/if}
              </td>
              <td class="col-td-genre"><span class="genre-pill">{song.genre}</span></td>
              <td class="col-td-pm">
                <span class="pm-name">{song.pmName}</span>
                <span class="reviewer-sub">QC: {song.qcReviewer}</span>
              </td>
              <td class="col-td-lineup">
                <div class="table-lineup-chips">
                  {#if song.lineup?.vocalLead}<span class="lineup-tag vocal">Vo: {song.lineup.vocalLead}</span>{/if}
                  {#if song.lineup?.guitarLead}<span class="lineup-tag guitar">Gu: {song.lineup.guitarLead}</span>{/if}
                  {#if song.lineup?.bass}<span class="lineup-tag bass">Ba: {song.lineup.bass}</span>{/if}
                  {#if song.lineup?.drums}<span class="lineup-tag drums">Dr: {song.lineup.drums}</span>{/if}
                  {#if song.lineup?.keys}<span class="lineup-tag keys">Ke: {song.lineup.keys}</span>{/if}
                  {#if !song.lineup?.vocalLead && !song.lineup?.guitarLead && !song.lineup?.bass && !song.lineup?.drums && !song.lineup?.keys}
                    <span class="lineup-tag unassigned">None</span>
                  {/if}
                </div>
              </td>
              <td class="col-td-stage">
                <span class="stage-pill {getStageBadgeClass(song.stage)}">
                  {getStageLabel(song.stage)}
                </span>
              </td>
              <td class="col-td-actions">
                <div class="table-action-group">
                  <button
                    type="button"
                    class="table-btn-icon"
                    onclick={() => openLineupDrawer(song)}
                    title="Assign Band Lineup"
                  >
                    <Users size={14} />
                  </button>

                  {#if song.stage === 'draft'}
                    <button
                      type="button"
                      class="bento-btn bento-btn-sm action-btn-blue table-btn"
                      onclick={() => advanceStatus(song, 'in_practice')}
                    >
                      Start
                    </button>
                  {:else if song.stage === 'in_practice'}
                    <button
                      type="button"
                      class="bento-btn bento-btn-sm action-btn-orange table-btn"
                      onclick={() => advanceStatus(song, 'ready_for_qc')}
                    >
                      Submit QC
                    </button>
                  {:else if song.stage === 'ready_for_qc'}
                    <button
                      type="button"
                      class="bento-btn bento-btn-sm qc-btn table-btn"
                      onclick={() => openQcDrawer(song)}
                    >
                      Audit
                    </button>
                  {:else if song.stage === 'qc_approved'}
                    <button
                      type="button"
                      class="bento-btn bento-btn-sm action-btn-green table-btn"
                      onclick={() => advanceStatus(song, 'stage_ready')}
                    >
                      Promote
                    </button>
                  {:else}
                    <CheckCircle2 size={16} class="text-green table-done-icon" />
                  {/if}
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<!-- Modal: Add New Music Number -->
{#if isAddModalOpen}
  <div class="modal-backdrop" onclick={() => (isAddModalOpen = false)} role="presentation">
    <div class="modal-card bento-card" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="drawer-header">
        <div>
          <h2>{$tStore('studio_shows.modal_add_title')}</h2>
          <p class="modal-subtitle">{$tStore('studio_shows.modal_add_desc')}</p>
        </div>
        <button class="icon-close" onclick={() => (isAddModalOpen = false)} aria-label="Close">
          <X size={18} />
        </button>
      </div>

      <form onsubmit={handleCreateNumber} class="modal-form">
        <div class="form-group">
          <label for="new-song-title">{$tStore('studio_shows.label_song_title')}</label>
          <input
            id="new-song-title"
            type="text"
            bind:value={newTitle}
            placeholder="e.g. Diễm Xưa, Đi Về Nhà..."
            required
            class="form-input"
          />
        </div>

        <div class="form-group">
          <label for="new-song-genre">{$tStore('studio_shows.label_genre')}</label>
          <input
            id="new-song-genre"
            type="text"
            bind:value={newGenre}
            placeholder="e.g. Pop Rock, Acoustic Ballad, Jazz Fusion..."
            class="form-input"
          />
        </div>

        <div class="form-row">
          <div class="form-group flex-1">
            <label for="new-song-pm">{$tStore('studio_shows.label_pm')}</label>
            <select id="new-song-pm" bind:value={newPm} class="form-input">
              {#each availableRoster as member}
                <option value={member}>{member}</option>
              {/each}
            </select>
          </div>

          <div class="form-group flex-1">
            <label for="new-song-qc">{$tStore('studio_shows.label_qc_reviewer')}</label>
            <select id="new-song-qc" bind:value={newQcReviewer} class="form-input">
              {#each availableRoster as member}
                <option value={member}>{member}</option>
              {/each}
            </select>
          </div>
        </div>

        <div class="modal-actions">
          <button type="button" class="bento-btn" onclick={() => (isAddModalOpen = false)}>
            Cancel
          </button>
          <button type="submit" class="bento-btn bento-btn-primary">
            {$tStore('studio_shows.btn_create_number')}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Modal: QC Verdict Drawer -->
{#if isQcDrawerOpen && activeSongForQc}
  <div class="modal-backdrop" onclick={() => (isQcDrawerOpen = false)} role="presentation">
    <div class="modal-card bento-card" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="drawer-header">
        <h2>{$tStore('studio_shows.qc_drawer_title')}</h2>
        <button class="icon-close" onclick={() => (isQcDrawerOpen = false)} aria-label="Close"><X size={18} /></button>
      </div>

      <p class="song-ref">Song: <strong>{activeSongForQc.title}</strong></p>
      <p class="reviewer-ref">Assigned QC Reviewer: <strong>{activeSongForQc.qcReviewer}</strong></p>

      <form onsubmit={handleQcSubmit} class="modal-form">
        <div class="form-group">
          <label for="verdict">{$tStore('studio_shows.qc_verdict')}</label>
          <div class="radio-group">
            <label class="radio-label">
              <input type="radio" bind:group={qcVerdict} value="pass" />
              <span>{$tStore('studio_shows.qc_pass')}</span>
            </label>
            <label class="radio-label">
              <input type="radio" bind:group={qcVerdict} value="revision" />
              <span>{$tStore('studio_shows.qc_revision')}</span>
            </label>
          </div>
        </div>

        <div class="form-group">
          <label for="qc-notes">{$tStore('studio_shows.qc_notes')}</label>
          <textarea
            id="qc-notes"
            bind:value={qcNotesInput}
            rows="3"
            placeholder="Feedback notes on vocal intonation, rhythm tightness, instrument volume balance..."
            required
            class="form-input"
          ></textarea>
        </div>

        <div class="modal-actions">
          <button type="button" class="bento-btn" onclick={() => (isQcDrawerOpen = false)}>Cancel</button>
          <button type="submit" class="bento-btn bento-btn-primary">
            {$tStore('studio_shows.btn_submit_qc')}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Modal: Lineup Role Assignment Drawer -->
{#if isLineupDrawerOpen && activeSongForLineup}
  <div class="modal-backdrop" onclick={() => (isLineupDrawerOpen = false)} role="presentation">
    <div class="modal-card bento-card" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="drawer-header">
        <h2>Assign Band Lineup</h2>
        <button class="icon-close" onclick={() => (isLineupDrawerOpen = false)} aria-label="Close"><X size={18} /></button>
      </div>

      <p class="song-ref">Song: <strong>{activeSongForLineup.title}</strong> ({activeSongForLineup.genre})</p>

      <form onsubmit={handleLineupSubmit} class="modal-form">
        <div class="form-group">
          <label for="role-vocal">Vocal Lead</label>
          <select id="role-vocal" bind:value={formVocalLead} class="form-input">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label for="role-guitar">Guitar Lead / Solo</label>
          <select id="role-guitar" bind:value={formGuitarLead} class="form-input">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label for="role-bass">Bass Guitar</label>
          <select id="role-bass" bind:value={formBass} class="form-input">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label for="role-drums">Drum Kit</label>
          <select id="role-drums" bind:value={formDrums} class="form-input">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label for="role-keys">Keyboard / Synthesizer</label>
          <select id="role-keys" bind:value={formKeys} class="form-input">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="modal-actions">
          <button type="button" class="bento-btn" onclick={() => (isLineupDrawerOpen = false)}>Cancel</button>
          <button type="submit" class="bento-btn bento-btn-primary">
            Save Lineup Allocation
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .numbers-subpage {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .bento-card {
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 18px;
    padding: 16px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04), 0 4px 12px rgba(0, 0, 0, 0.02);
  }

  /* --------------------------------------------------------------------------
     1. Pipeline Funnel Summary
     -------------------------------------------------------------------------- */
  .pipeline-summary {
    display: flex;
    flex-direction: column;
    gap: 14px;
    background: #ffffff;
  }

  .funnel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 12px;
  }

  .funnel-title-row {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }

  .funnel-heading {
    font-family: 'Outfit', sans-serif;
    font-size: 16px;
    font-weight: 700;
    color: #0f172a;
  }

  .funnel-sub {
    font-size: 13px;
    color: #64748b;
    margin-left: 4px;
  }

  .funnel-tiles {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 10px;
  }

  .funnel-tile {
    background: #f8fafc;
    border: 1px solid rgba(0, 0, 0, 0.06);
    border-radius: 12px;
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    text-align: left;
  }

  .funnel-tile:hover {
    background: #ffffff;
    border-color: rgba(255, 107, 0, 0.3);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  }

  .funnel-tile.is-active {
    background: #fff7ed;
    border-color: #ff6b00;
    box-shadow: 0 2px 8px rgba(255, 107, 0, 0.15);
  }

  .tile-count {
    font-family: 'Outfit', sans-serif;
    font-size: 20px;
    font-weight: 800;
    line-height: 1;
    color: #0f172a;
  }

  .tile-label {
    font-size: 11px;
    font-weight: 600;
    color: #64748b;
  }

  .text-slate { color: #64748b; }
  .text-blue { color: #2563eb; }
  .text-orange { color: #ff6b00; }
  .text-indigo { color: #4f46e5; }
  .text-green { color: #16a34a; }

  /* --------------------------------------------------------------------------
     2. Toolbar & View Controls
     -------------------------------------------------------------------------- */
  .toolbar-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 12px;
    padding: 12px 16px;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 8px;
    background: #f8fafc;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 10px;
    padding: 6px 12px;
    flex: 1 1 280px;
    max-width: 440px;
  }

  .search-input {
    border: none;
    background: transparent;
    font-size: 13px;
    color: #0f172a;
    width: 100%;
    outline: none;
  }

  .clear-search-btn {
    background: none;
    border: none;
    cursor: pointer;
    color: #94a3b8;
    display: flex;
    align-items: center;
  }

  .filters-row {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 10px;
  }

  .select-wrapper {
    display: flex;
    align-items: center;
    gap: 6px;
    background: #f8fafc;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 10px;
    padding: 6px 10px;
  }

  .filter-select {
    border: none;
    background: transparent;
    font-size: 12px;
    font-weight: 600;
    color: #334155;
    outline: none;
    cursor: pointer;
  }

  .view-switcher {
    display: flex;
    align-items: center;
    background: #f1f5f9;
    border-radius: 10px;
    padding: 3px;
    gap: 2px;
  }

  .view-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    border: none;
    padding: 6px 10px;
    border-radius: 8px;
    font-size: 12px;
    font-weight: 600;
    color: #64748b;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .view-btn:hover {
    color: #0f172a;
  }

  .view-btn.is-active {
    background: #ffffff;
    color: #ff6b00;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
    font-weight: 700;
  }

  /* --------------------------------------------------------------------------
     3. Scalable Bento Grid View
     -------------------------------------------------------------------------- */
  .numbers-bento-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
    gap: 16px;
  }

  .song-bento-card {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 12px;
    padding: 18px;
    transition: transform 0.25s cubic-bezier(0.34, 1.56, 0.64, 1), box-shadow 0.25s ease, border-color 0.25s ease;
  }

  .song-bento-card:hover {
    transform: translateY(-3px);
    border-color: rgba(255, 107, 0, 0.3);
    box-shadow: 0 8px 24px -4px rgba(0, 0, 0, 0.08);
  }

  .song-bento-card.highlight-stage-ready {
    border-left: 4px solid #16a34a;
  }

  .card-top-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .order-badge {
    font-family: 'JetBrains Mono', monospace;
    font-size: 11px;
    font-weight: 700;
    color: #64748b;
    background: #f1f5f9;
    padding: 2px 8px;
    border-radius: 6px;
  }

  .card-main-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .song-title {
    font-family: 'Outfit', sans-serif;
    font-size: 16px;
    font-weight: 700;
    color: #0f172a;
    margin: 0;
    line-height: 1.3;
  }

  .genre-tag {
    font-size: 12px;
    color: #64748b;
    font-weight: 500;
  }

  .personnel-row {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-top: 4px;
    font-size: 12px;
    color: #475569;
  }

  .pm-badge, .reviewer-badge {
    background: #f8fafc;
    padding: 2px 6px;
    border-radius: 4px;
    border: 1px solid rgba(0, 0, 0, 0.05);
  }

  .lineup-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .lineup-label {
    font-size: 11px;
    font-weight: 700;
    color: #64748b;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .lineup-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 5px;
  }

  .lineup-tag {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 11px;
    font-weight: 600;
    padding: 3px 8px;
    border-radius: 6px;
    background: #f1f5f9;
    color: #334155;
  }

  .lineup-tag.vocal { background: rgba(255, 107, 0, 0.1); color: #ff6b00; }
  .lineup-tag.guitar { background: rgba(59, 130, 246, 0.1); color: #2563eb; }
  .lineup-tag.bass { background: rgba(147, 51, 234, 0.1); color: #9333ea; }
  .lineup-tag.drums { background: rgba(22, 163, 74, 0.1); color: #16a34a; }
  .lineup-tag.keys { background: rgba(13, 148, 136, 0.1); color: #0d9488; }
  .lineup-tag.unassigned { color: #94a3b8; font-style: italic; background: #f8fafc; }

  .qc-notes-box {
    display: flex;
    align-items: flex-start;
    gap: 6px;
    font-size: 12px;
    background: #f8fafc;
    padding: 8px 10px;
    border-radius: 8px;
    color: #334155;
    border: 1px solid rgba(0, 0, 0, 0.05);
  }

  .qc-notes-box.warning {
    background: #fff7ed;
    color: #c2410c;
    border-color: #ffedd5;
  }

  .qc-note-icon {
    flex-shrink: 0;
    margin-top: 2px;
  }

  .card-btn-row {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 6px;
    width: 100%;
  }

  .lineup-btn, .action-btn-blue, .action-btn-orange, .action-btn-green, .qc-btn {
    flex: 1 1 auto;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    white-space: nowrap;
    font-size: 11px;
    font-weight: 700;
    padding: 6px 10px;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .lineup-btn {
    background: #f1f5f9;
    color: #334155;
    border: 1px solid rgba(0, 0, 0, 0.06);
  }

  .lineup-btn:hover {
    background: #e2e8f0;
  }

  .action-btn-blue {
    background: rgba(37, 99, 235, 0.1);
    color: #2563eb;
    border: 1px solid rgba(37, 99, 235, 0.2);
  }

  .action-btn-blue:hover {
    background: #2563eb;
    color: #ffffff;
  }

  .action-btn-orange {
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    border: 1px solid rgba(255, 107, 0, 0.2);
  }

  .action-btn-orange:hover {
    background: #ff6b00;
    color: #ffffff;
  }

  .qc-btn {
    background: #fff7ed;
    color: #ea580c;
    border: 1px solid #fdba74;
  }

  .qc-btn:hover {
    background: #ea580c;
    color: #ffffff;
  }

  .action-btn-green {
    background: rgba(22, 163, 74, 0.1);
    color: #16a34a;
    border: 1px solid rgba(22, 163, 74, 0.2);
  }

  .action-btn-green:hover {
    background: #16a34a;
    color: #ffffff;
  }

  .stage-ready-indicator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    width: 100%;
    padding: 6px;
    font-size: 12px;
    font-weight: 700;
    color: #16a34a;
    background: rgba(22, 163, 74, 0.08);
    border-radius: 8px;
  }

  /* Stage Pills */
  .stage-pill {
    display: inline-flex;
    align-items: center;
    padding: 3px 8px;
    border-radius: 20px;
    font-size: 11px;
    font-weight: 700;
  }

  .badge-draft { background: #f1f5f9; color: #64748b; }
  .badge-practice { background: rgba(37, 99, 235, 0.1); color: #2563eb; }
  .badge-ready-qc { background: rgba(255, 107, 0, 0.1); color: #ff6b00; }
  .badge-qc-approved { background: rgba(79, 70, 229, 0.1); color: #4f46e5; }
  .badge-stage-ready { background: rgba(22, 163, 74, 0.12); color: #16a34a; }

  /* --------------------------------------------------------------------------
     4. Refined Kanban View
     -------------------------------------------------------------------------- */
  .kanban-scroll-wrapper {
    overflow-x: auto;
    padding-bottom: 8px;
  }

  .kanban-board {
    display: grid;
    grid-template-columns: repeat(5, minmax(225px, 1fr));
    gap: 12px;
    min-width: 1140px;
    width: 100%;
  }

  .kanban-col {
    display: flex;
    flex-direction: column;
    gap: 12px;
    background: #f8fafc;
    min-height: 480px;
  }

  .col-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .col-title {
    font-size: 12px;
    font-weight: 700;
    color: #475569;
  }

  .col-count {
    background: #e2e8f0;
    color: #475569;
    padding: 2px 8px;
    border-radius: 10px;
    font-size: 12px;
    font-weight: 700;
  }

  .col-cards {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .song-card {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 12px;
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.07);
    border-radius: 12px;
  }

  .song-card.highlight-green {
    border-left: 3px solid #16a34a;
  }

  .genre-micro {
    font-size: 11px;
    color: #64748b;
  }

  .song-pm, .song-qc-meta {
    font-size: 12px;
    color: #64748b;
  }

  /* --------------------------------------------------------------------------
     5. Compact Production Table View
     -------------------------------------------------------------------------- */
  .table-container {
    overflow-x: auto;
    padding: 0;
  }

  .production-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
    text-align: left;
  }

  .production-table th {
    background: #f8fafc;
    padding: 12px 14px;
    font-size: 11px;
    font-weight: 700;
    color: #64748b;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-bottom: 1px solid rgba(0, 0, 0, 0.08);
  }

  .production-table td {
    padding: 12px 14px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
    vertical-align: middle;
  }

  .table-row:hover {
    background: #fdfdfd;
  }

  .col-th-order, .col-td-order {
    width: 45px;
    text-align: center;
    font-family: 'JetBrains Mono', monospace;
    font-weight: 700;
    color: #94a3b8;
  }

  .col-td-song {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .row-song-title {
    font-weight: 700;
    color: #0f172a;
  }

  .row-qc-note {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 11px;
    color: #c2410c;
  }

  .genre-pill {
    background: #f1f5f9;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 11px;
    color: #475569;
  }

  .col-td-pm {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .pm-name {
    font-weight: 600;
    color: #1e293b;
  }

  .reviewer-sub {
    font-size: 11px;
    color: #64748b;
  }

  .table-lineup-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    max-width: 260px;
  }

  .table-action-group {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .table-btn-icon {
    background: #f1f5f9;
    border: 1px solid rgba(0, 0, 0, 0.06);
    border-radius: 6px;
    padding: 6px;
    cursor: pointer;
    color: #475569;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .table-btn-icon:hover {
    background: #e2e8f0;
  }

  .table-btn {
    font-size: 11px;
    padding: 4px 10px;
    flex: initial;
  }

  .table-done-icon {
    margin-left: 8px;
  }

  /* --------------------------------------------------------------------------
     6. Empty State
     -------------------------------------------------------------------------- */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 48px 24px;
    text-align: center;
  }

  .empty-title {
    font-family: 'Outfit', sans-serif;
    font-size: 18px;
    font-weight: 700;
    color: #0f172a;
    margin: 0;
  }

  .empty-desc {
    font-size: 13px;
    color: #64748b;
    max-width: 400px;
    margin: 0;
  }

  /* --------------------------------------------------------------------------
     7. Modals & Drawers
     -------------------------------------------------------------------------- */
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 23, 42, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 16px;
  }

  .modal-card {
    width: 100%;
    max-width: 500px;
    box-shadow: 0 20px 40px -12px rgba(0, 0, 0, 0.2);
  }

  .drawer-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .drawer-header h2 {
    font-family: 'Outfit', sans-serif;
    font-size: 18px;
    font-weight: 800;
    color: #0f172a;
    margin: 0;
  }

  .modal-subtitle {
    font-size: 12px;
    color: #64748b;
    margin-top: 4px;
  }

  .icon-close {
    background: none;
    border: none;
    cursor: pointer;
    color: #94a3b8;
    padding: 4px;
  }

  .icon-close:hover {
    color: #0f172a;
  }

  .song-ref, .reviewer-ref {
    font-size: 13px;
    color: #475569;
    margin-top: 6px;
  }

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 14px;
    margin-top: 14px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-group.flex-1 {
    flex: 1;
  }

  .form-row {
    display: flex;
    gap: 12px;
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
    box-shadow: 0 0 0 2px rgba(255, 107, 0, 0.2);
  }

  .radio-group {
    display: flex;
    gap: 16px;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 8px;
  }

  /* Responsive Adjustments */
  @media (max-width: 900px) {
    .funnel-tiles {
      grid-template-columns: repeat(3, 1fr);
    }

    .view-btn-text {
      display: none;
    }
  }

  @media (max-width: 600px) {
    .funnel-tiles {
      grid-template-columns: repeat(2, 1fr);
    }

    .toolbar-card {
      flex-direction: column;
      align-items: stretch;
      gap: 10px;
    }

    .search-box {
      max-width: 100%;
    }

    .filters-row {
      flex-direction: column;
      align-items: stretch;
      gap: 8px;
    }

    .select-wrapper {
      width: 100%;
    }

    .filter-select {
      width: 100%;
    }

    .view-switcher {
      justify-content: center;
      width: 100%;
    }

    .view-btn-text {
      display: inline !important;
    }

    .numbers-bento-grid {
      grid-template-columns: 1fr;
    }
  }
</style>


