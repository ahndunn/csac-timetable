<script lang="ts">
  import { onMount } from 'svelte';
  import { api, ApiError } from '$lib/api/client';
  import { auth } from '$lib/stores/auth.svelte';
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore } from '$lib/i18n';
  import {
    Music,
    Users,
    Guitar,
    Calendar,
    CheckCircle2,
    Clock,
    AlertTriangle,
    Shield,
    Sparkles,
    Plus,
    X,
    Filter,
    Layers,
    Sliders,
    Award,
    Check,
    Send,
    Radio,
    FileCheck
  } from '@lucide/svelte';

  // Perspectives: DM/Admin vs Performer/PM
  let viewRole = $state<'dm' | 'performer'>('dm');
  let activeTab = $state<'numbers' | 'instruments' | 'freetime'>('numbers');

  // Live Data State
  let musicNumbers = $state<any[]>([]);
  let sprints = $state<any[]>([]);
  let activeSprint = $state<any | null>(null);
  let sprintTasks = $state<any[]>([]);
  let instruments = $state<any[]>([]);
  let reservations = $state<any[]>([]);
  let isLoading = $state(true);
  let errorMsg = $state<string | null>(null);
  let successMsg = $state<string | null>(null);

  // Modals & Forms
  let isCreateNumberOpen = $state(false);
  let newNumber = $state({
    title: '',
    genre: '',
    target_sessions_per_week: 2,
    description: '',
  });

  let isRegisterGearOpen = $state(false);
  let newGear = $state({
    name: '',
    code: '',
    category: 'Strings',
    ownership_type: 'member_owned' as 'club_property' | 'member_owned',
    availability_status: 'free_to_borrow' as 'free_to_borrow' | 'unavailable',
    custody_location: '',
    notes: '',
  });

  let isReserveOpen = $state(false);
  let reservePayload = $state({
    instrument_id: '',
    music_number_id: '',
    day_of_week: 'Thứ Bảy',
    slot_label: '18h - 19h',
    notes: '',
  });

  let isQcModalOpen = $state(false);
  let activeQcTask = $state<any | null>(null);
  let qcFeedbackText = $state('');

  // Free-time matrix
  const WEEKDAYS = ['Thứ Hai', 'Thứ Ba', 'Thứ Tư', 'Thứ Năm', 'Thứ Sáu', 'Thứ Bảy', 'Chủ Nhật'];
  const SLOTS = ['17h - 18h', '18h - 19h', '19h - 20h', '20h - 21h'];
  let freeTimeSelections = $state<Record<string, boolean>>({});

  // Filter state for instruments
  let gearFilter = $state<'all' | 'club' | 'member'>('all');

  async function loadData() {
    isLoading = true;
    errorMsg = null;
    try {
      const [numsRes, sprintsRes, instRes] = await Promise.all([
        api.music.listNumbers().catch(() => []),
        api.sprints.list().catch(() => []),
        api.music.listInstruments().catch(() => ({ instruments: [], reservations: [] })),
      ]);

      musicNumbers = numsRes;
      sprints = sprintsRes;
      instruments = instRes.instruments || [];
      reservations = instRes.reservations || [];

      if (sprints.length > 0) {
        activeSprint = sprints.find((s: any) => s.is_active) || sprints[0];
        if (activeSprint) {
          sprintTasks = await api.sprints.listTasks(activeSprint.id).catch(() => []);
        }
      }
    } catch (err: any) {
      errorMsg = err.message || 'Error loading studio data';
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    loadData();
  });

  async function handleCreateNumber() {
    if (!newNumber.title.trim()) return;
    try {
      await api.music.createNumber(newNumber);
      isCreateNumberOpen = false;
      newNumber = { title: '', genre: '', target_sessions_per_week: 2, description: '' };
      successMsg = 'Music number created successfully!';
      setTimeout(() => (successMsg = null), 4000);
      await loadData();
    } catch (err: any) {
      errorMsg = err.message;
    }
  }

  async function handleRegisterGear() {
    if (!newGear.name.trim() || !newGear.code.trim()) return;
    try {
      await api.music.registerInstrument(newGear);
      isRegisterGearOpen = false;
      newGear = {
        name: '',
        code: '',
        category: 'Strings',
        ownership_type: 'member_owned',
        availability_status: 'free_to_borrow',
        custody_location: '',
        notes: '',
      };
      successMsg = 'Instrument registered into club fleet!';
      setTimeout(() => (successMsg = null), 4000);
      await loadData();
    } catch (err: any) {
      errorMsg = err.message;
    }
  }

  async function handleReserveInstrument() {
    if (!reservePayload.instrument_id || !reservePayload.music_number_id) return;
    try {
      await api.music.reserveInstrument(reservePayload);
      isReserveOpen = false;
      successMsg = 'Instrument allocated without conflicts!';
      setTimeout(() => (successMsg = null), 4000);
      await loadData();
    } catch (err: any) {
      errorMsg = err.message;
    }
  }

  async function handleToggleGearStatus(inst: any) {
    const nextStatus = inst.availability_status === 'free_to_borrow' ? 'unavailable' : 'free_to_borrow';
    try {
      await api.music.updateInstrumentStatus(inst.id, { availability_status: nextStatus });
      await loadData();
    } catch (err: any) {
      errorMsg = err.message;
    }
  }

  function openQcModal(task: any) {
    activeQcTask = task;
    qcFeedbackText = task.qc_feedback || '';
    isQcModalOpen = true;
  }

  async function submitQcDecision(decision: 'passed' | 'blocked') {
    if (!activeSprint || !activeQcTask) return;
    try {
      await api.sprints.reviewTask(activeSprint.id, activeQcTask.id, {
        status: decision,
        qc_feedback: qcFeedbackText,
      });
      isQcModalOpen = false;
      successMsg = `QC review verdict: ${decision.toUpperCase()} recorded!`;
      setTimeout(() => (successMsg = null), 4000);
      sprintTasks = await api.sprints.listTasks(activeSprint.id);
    } catch (err: any) {
      errorMsg = err.message;
    }
  }

  function toggleFreeTimeSlot(day: string, slot: string) {
    const key = `${day}::${slot}`;
    freeTimeSelections = {
      ...freeTimeSelections,
      [key]: !freeTimeSelections[key],
    };
  }

  async function handleSaveFreeTime() {
    if (!activeSprint) return;
    const slots = [];
    for (const day of WEEKDAYS) {
      for (const slot of SLOTS) {
        const key = `${day}::${slot}`;
        if (freeTimeSelections[key]) {
          slots.push({ day_of_week: day, slot_label: slot, is_available: true });
        }
      }
    }

    try {
      await api.sprints.submitAvailability(activeSprint.id, { slots });
      successMsg = $tStore('studio.freetime_saved');
      setTimeout(() => (successMsg = null), 4000);
    } catch (err: any) {
      errorMsg = err.message;
    }
  }

  let scheduleStubMsg = $state<string | null>(null);

  async function triggerScheduleStub() {
    if (!activeSprint) return;
    try {
      await api.sprints.scheduleStub(activeSprint.id);
    } catch (err: any) {
      scheduleStubMsg = err.message || $tStore('studio.schedule_stub_alert');
    }
  }

  let filteredInstruments = $derived.by(() => {
    if (gearFilter === 'club') return instruments.filter(i => i.ownership_type === 'club_property');
    if (gearFilter === 'member') return instruments.filter(i => i.ownership_type === 'member_owned');
    return instruments;
  });
</script>

<svelte:head>
  <title>{$tStore('studio.page_title')}</title>
</svelte:head>

<Navbar />

<div class="studio-container">
  <!-- Top Navigation & Role Switcher -->
  <div class="bento-card studio-header">
    <div>
      <div class="tag-row">
        <span class="bento-tag"><Sparkles size={13} color="var(--accent)" /> CSAC MUSIC STUDIO</span>
        <span class="bento-tag live-tag">LIVE INFRA</span>
      </div>
      <h1 class="studio-heading">{$tStore('studio.heading')}</h1>
      <p class="studio-subheading">{$tStore('studio.subheading')}</p>
    </div>

    <div class="role-switcher-box">
      <span class="role-hint">{$tStore('hub.role_indicator') || 'Chế độ xem:'}</span>
      <div class="bento-tabs">
        <button
          type="button"
          class="bento-tab-btn {viewRole === 'dm' ? 'is-active' : ''}"
          onclick={() => (viewRole = 'dm')}
        >
          <Shield size={14} />
          <span>{$tStore('studio.role_dm')}</span>
        </button>
        <button
          type="button"
          class="bento-tab-btn {viewRole === 'performer' ? 'is-active' : ''}"
          onclick={() => (viewRole = 'performer')}
        >
          <Music size={14} />
          <span>{$tStore('studio.role_performer')}</span>
        </button>
      </div>
    </div>
  </div>

  <!-- Status Alerts -->
  {#if successMsg}
    <div class="bento-card success-banner">
      <CheckCircle2 size={18} color="var(--success)" />
      <span>{successMsg}</span>
    </div>
  {/if}
  {#if errorMsg}
    <div class="bento-card error-banner">
      <AlertTriangle size={18} color="var(--danger)" />
      <span>{errorMsg}</span>
      <button type="button" class="close-btn" onclick={() => (errorMsg = null)}><X size={14} /></button>
    </div>
  {/if}

  <!-- Main Sub-navigation Tabs -->
  <div class="tab-strip">
    <button
      type="button"
      class="subtab-btn {activeTab === 'numbers' ? 'is-active' : ''}"
      onclick={() => (activeTab = 'numbers')}
    >
      <Layers size={16} />
      <span>{$tStore('studio.tab_numbers')}</span>
    </button>
    <button
      type="button"
      class="subtab-btn {activeTab === 'instruments' ? 'is-active' : ''}"
      onclick={() => (activeTab = 'instruments')}
    >
      <Guitar size={16} />
      <span>{$tStore('studio.tab_instruments')}</span>
      <span class="badge-counter">{instruments.length}</span>
    </button>
    <button
      type="button"
      class="subtab-btn {activeTab === 'freetime' ? 'is-active' : ''}"
      onclick={() => (activeTab = 'freetime')}
    >
      <Calendar size={16} />
      <span>{$tStore('studio.tab_freetime')}</span>
    </button>
  </div>

  <!-- TAB 1: MUSIC NUMBERS & SPRINT BOARD -->
  {#if activeTab === 'numbers'}
    <div class="bento-grid-two">
      <!-- Left Column: Numbers List -->
      <div class="bento-card numbers-panel">
        <div class="panel-header">
          <div>
            <h2 class="panel-title">{$tStore('studio.numbers_title')}</h2>
            <span class="panel-subtitle">{musicNumbers.length} tiết mục trong danh mục</span>
          </div>
          {#if viewRole === 'dm'}
            <button
              type="button"
              class="bento-btn btn-accent"
              onclick={() => (isCreateNumberOpen = true)}
            >
              <Plus size={15} />
              <span>{$tStore('studio.create_number_btn')}</span>
            </button>
          {/if}
        </div>

        {#if musicNumbers.length === 0}
          <div class="empty-state">
            <Music size={32} color="var(--text-muted)" />
            <p>{$tStore('studio.no_numbers')}</p>
          </div>
        {:else}
          <div class="numbers-list">
            {#each musicNumbers as item (item.number.id)}
              <div class="bento-card number-card">
                <div class="number-card-top">
                  <div>
                    <span class="bento-pill genre-pill">{item.number.genre || 'Acoustic'}</span>
                    <h3 class="number-title">{item.number.title}</h3>
                  </div>
                  <span class="bento-pill status-pill is-{item.number.status}">
                    {item.number.status.replace(/_/g, ' ').toUpperCase()}
                  </span>
                </div>

                <p class="number-desc">{item.number.description || 'Không có mô tả thêm'}</p>

                <div class="number-meta-row">
                  <div class="meta-item">
                    <Clock size={13} />
                    <span>{$tStore('studio.target_sessions', { count: item.number.target_sessions_per_week })}</span>
                  </div>
                  <div class="meta-item">
                    <Users size={13} />
                    <span>{$tStore('studio.lineup_title', { count: item.members.length })}</span>
                  </div>
                </div>

                <!-- Performer lineup tags -->
                {#if item.members.length > 0}
                  <div class="members-tag-wrap">
                    {#each item.members as member}
                      <span class="member-role-tag {member.is_lead ? 'is-lead' : ''}">
                        <strong>{member.instrument_role}:</strong>
                        <span>{member.user_id.slice(0, 8)}...</span>
                      </span>
                    {/each}
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Right Column: Agile Practice Sprint Board -->
      <div class="bento-card sprint-panel">
        <div class="panel-header">
          <div>
            <div class="sprint-tag-row">
              <span class="bento-pill is-active">AGILE SDLC</span>
              <span class="sprint-name">{activeSprint?.name || 'Sprint 1'}</span>
            </div>
            <h2 class="panel-title">{$tStore('studio.sprint_board_title')}</h2>
          </div>

          <button
            type="button"
            class="bento-btn btn-primary"
            onclick={triggerScheduleStub}
            title="Kích hoạt giải thuật CSP xếp lịch"
          >
            <Sparkles size={15} color="var(--accent)" />
            <span>{$tStore('studio.btn_auto_schedule')}</span>
          </button>
        </div>

        {#if activeSprint?.sprint_goal}
          <div class="sprint-goal-box">
            <strong>Mục tiêu Sprint:</strong> {activeSprint.sprint_goal}
          </div>
        {/if}

        <div class="tasks-board">
          {#each sprintTasks as task (task.id)}
            <div class="bento-card task-card task-type-{task.task_type}">
              <div class="task-card-header">
                <span class="task-type-badge type-{task.task_type}">
                  {#if task.task_type === 'study'}
                    📘 {$tStore('studio.task_type_study')}
                  {:else if task.task_type === 'create'}
                    🎨 {$tStore('studio.task_type_create')}
                  {:else}
                    🔍 {$tStore('studio.task_type_qc')}
                  {/if}
                </span>
                <span class="task-status-badge status-{task.status}">
                  {task.status.replace(/_/g, ' ')}
                </span>
              </div>

              <h4 class="task-title">{task.title}</h4>
              {#if task.description}
                <p class="task-desc">{task.description}</p>
              {/if}

              {#if task.qc_feedback}
                <div class="qc-feedback-box">
                  <Award size={13} color="var(--accent)" />
                  <em>{$tStore('studio.qc_feedback', { feedback: task.qc_feedback })}</em>
                </div>
              {/if}

              <!-- QC Action Button for PM / DM -->
              {#if task.task_type === 'review_qc'}
                <div class="task-action-row">
                  <button
                    type="button"
                    class="bento-btn mini-btn"
                    onclick={() => openQcModal(task)}
                  >
                    <FileCheck size={13} />
                    <span>Duyệt QC Bài</span>
                  </button>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    </div>
  {/if}

  <!-- TAB 2: INSTRUMENT FLEET & CUSTODY -->
  {#if activeTab === 'instruments'}
    <div class="bento-card instruments-container">
      <div class="panel-header">
        <div>
          <h2 class="panel-title">{$tStore('studio.instruments_title')}</h2>
          <span class="panel-subtitle">Theo dõi tài sản CLB & nhạc cụ mượn từ thành viên</span>
        </div>

        <div class="gear-actions-row">
          <div class="bento-tabs">
            <button
              type="button"
              class="bento-tab-btn {gearFilter === 'all' ? 'is-active' : ''}"
              onclick={() => (gearFilter = 'all')}
            >
              {$tStore('studio.filter_all_gear')}
            </button>
            <button
              type="button"
              class="bento-tab-btn {gearFilter === 'club' ? 'is-active' : ''}"
              onclick={() => (gearFilter = 'club')}
            >
              {$tStore('studio.filter_club_gear')}
            </button>
            <button
              type="button"
              class="bento-tab-btn {gearFilter === 'member' ? 'is-active' : ''}"
              onclick={() => (gearFilter = 'member')}
            >
              {$tStore('studio.filter_member_gear')}
            </button>
          </div>

          <button
            type="button"
            class="bento-btn btn-accent"
            onclick={() => (isRegisterGearOpen = true)}
          >
            <Plus size={15} />
            <span>{$tStore('studio.register_gear_btn')}</span>
          </button>
        </div>
      </div>

      <!-- Instrument Fleet Grid -->
      <div class="instruments-grid">
        {#each filteredInstruments as inst (inst.id)}
          <div class="bento-card instrument-card">
            <div class="inst-top">
              <div>
                <span class="bento-pill category-pill">{inst.category} • {inst.code}</span>
                <h3 class="inst-name">{inst.name}</h3>
              </div>
              <span class="inst-availability-badge status-{inst.availability_status}">
                {#if inst.availability_status === 'free_to_borrow'}
                  {$tStore('studio.status_free')}
                {:else if inst.availability_status === 'in_use'}
                  {$tStore('studio.status_in_use')}
                {:else if inst.availability_status === 'unavailable'}
                  {$tStore('studio.status_unavailable')}
                {:else}
                  {$tStore('studio.status_maintenance')}
                {/if}
              </span>
            </div>

            <!-- Ownership and Custody Info -->
            <div class="inst-info-block">
              <div class="info-line">
                <strong>Sở hữu:</strong>
                {#if inst.ownership_type === 'club_property'}
                  <span class="tag-club">🏛️ {$tStore('studio.ownership_club')}</span>
                {:else}
                  <span class="tag-member">👤 Đồ cá nhân</span>
                {/if}
              </div>
              <div class="info-line">
                <strong>Vị trí / Đang giữ:</strong>
                <span class="custody-text">{inst.custody_location || 'Kho CLB'}</span>
              </div>
              {#if inst.notes}
                <div class="inst-notes">{inst.notes}</div>
              {/if}
            </div>

            <!-- Instrument Actions -->
            <div class="inst-actions">
              <button
                type="button"
                class="bento-btn mini-btn"
                onclick={() => handleToggleGearStatus(inst)}
                title="Bật/tắt trạng thái cho mượn"
              >
                <span>{inst.availability_status === 'free_to_borrow' ? 'Khóa (Không cho mượn)' : 'Mở cho mượn'}</span>
              </button>

              <button
                type="button"
                class="bento-btn mini-btn btn-primary"
                onclick={() => {
                  reservePayload.instrument_id = inst.id;
                  isReserveOpen = true;
                }}
              >
                <Clock size={12} />
                <span>{$tStore('studio.reserve_btn')}</span>
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- TAB 3: SPRINT FREE-TIME REGISTRATION -->
  {#if activeTab === 'freetime'}
    <div class="bento-card freetime-container">
      <div class="panel-header">
        <div>
          <h2 class="panel-title">{$tStore('studio.freetime_title')}</h2>
          <p class="panel-subtitle">{$tStore('studio.freetime_desc')}</p>
        </div>

        <button
          type="button"
          class="bento-btn btn-accent"
          onclick={handleSaveFreeTime}
        >
          <Check size={15} />
          <span>{$tStore('studio.btn_save_freetime')}</span>
        </button>
      </div>

      <!-- Free-time Grid -->
      <div class="freetime-table-wrapper">
        <table class="freetime-table">
          <thead>
            <tr>
              <th class="th-slot">Khung Giờ</th>
              {#each WEEKDAYS as day}
                <th class="th-day">{day}</th>
              {/each}
            </tr>
          </thead>
          <tbody>
            {#each SLOTS as slot}
              <tr>
                <td class="td-slot-label">{slot}</td>
                {#each WEEKDAYS as day}
                  {@const key = `${day}::${slot}`}
                  {@const isChecked = !!freeTimeSelections[key]}
                  <td
                    class="td-cell {isChecked ? 'is-available' : ''}"
                    onclick={() => toggleFreeTimeSlot(day, slot)}
                  >
                    <div class="cell-content">
                      {#if isChecked}
                        <Check size={16} color="white" />
                        <span class="cell-label">Rảnh</span>
                      {:else}
                        <span class="cell-dash">—</span>
                      {/if}
                    </div>
                  </td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>

<!-- MODAL: Create Music Number -->
{#if isCreateNumberOpen}
  <div class="modal-backdrop" onclick={() => (isCreateNumberOpen = false)} role="presentation">
    <div class="bento-card modal-box" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="modal-header">
        <h3 class="modal-title">Tạo Tiết Mục Biểu Diễn Mới</h3>
        <button type="button" class="close-btn" onclick={() => (isCreateNumberOpen = false)}><X size={16} /></button>
      </div>
      <div class="modal-body">
        <label class="form-label">
          <span>Tên Bài Hát / Tiết Mục *</span>
          <input type="text" bind:value={newNumber.title} placeholder="Ví dụ: Bật Tình Yêu Lên" class="bento-input" />
        </label>
        <label class="form-label">
          <span>Thể Loại Âm Nhạc</span>
          <input type="text" bind:value={newNumber.genre} placeholder="Ví dụ: Pop R&B, Acoustic" class="bento-input" />
        </label>
        <label class="form-label">
          <span>Số Buổi Tập Mong Muốn / Tuần</span>
          <input type="number" min="1" max="7" bind:value={newNumber.target_sessions_per_week} class="bento-input" />
        </label>
        <label class="form-label">
          <span>Mô Tả Tiết Mục</span>
          <textarea bind:value={newNumber.description} placeholder="Ghi chú về nhạc cụ hoặc phong cách phối..." class="bento-input"></textarea>
        </label>
      </div>
      <div class="modal-footer">
        <button type="button" class="bento-btn" onclick={() => (isCreateNumberOpen = false)}>Hủy bỏ</button>
        <button type="button" class="bento-btn btn-accent" onclick={handleCreateNumber}>Lưu Tiết Mục</button>
      </div>
    </div>
  </div>
{/if}

<!-- MODAL: Register Instrument -->
{#if isRegisterGearOpen}
  <div class="modal-backdrop" onclick={() => (isRegisterGearOpen = false)} role="presentation">
    <div class="bento-card modal-box" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="modal-header">
        <h3 class="modal-title">Đăng Ký Nhạc Cụ Mới Vào Hệ Thống</h3>
        <button type="button" class="close-btn" onclick={() => (isRegisterGearOpen = false)}><X size={16} /></button>
      </div>
      <div class="modal-body">
        <label class="form-label">
          <span>Tên Nhạc Cụ / Thiết Bị *</span>
          <input type="text" bind:value={newGear.name} placeholder="Ví dụ: Fender Player Plus Stratocaster" class="bento-input" />
        </label>
        <label class="form-label">
          <span>Mã Định Danh (Code) *</span>
          <input type="text" bind:value={newGear.code} placeholder="Ví dụ: GTR-02" class="bento-input" />
        </label>
        <label class="form-label">
          <span>Phân Loại</span>
          <select bind:value={newGear.category} class="bento-input">
            <option value="Strings">Dây (Guitar / Bass / Violin)</option>
            <option value="Keyboard">Phím (Piano / Synthesizer / Organ)</option>
            <option value="Percussion">Gõ (Trống Jazz / Cajon / Pad)</option>
            <option value="Amplifier">Amplifier / Amp Cabinet</option>
            <option value="Audio Gear">Micro / Sound Card / Bàn Mixer / Pedal</option>
          </select>
        </label>
        <label class="form-label">
          <span>Loại Sở Hữu</span>
          <select bind:value={newGear.ownership_type} class="bento-input">
            <option value="member_owned">Đồ cá nhân của thành viên</option>
            <option value="club_property">Tài sản sở hữu của CSAC</option>
          </select>
        </label>
        <label class="form-label">
          <span>Vị Trí / Ai Đang Giữ</span>
          <input type="text" bind:value={newGear.custody_location} placeholder="Ví dụ: Giữ bởi Minh Pháp hoặc Tủ đồ CLB" class="bento-input" />
        </label>
        <label class="form-label">
          <span>Trạng Thái Cho Mượn Ban Đầu</span>
          <select bind:value={newGear.availability_status} class="bento-input">
            <option value="free_to_borrow">Sẵn sàng cho các bài khác mượn (Free to borrow)</option>
            <option value="unavailable">Chỉ dùng cho bài của tôi (Unavailable)</option>
          </select>
        </label>
      </div>
      <div class="modal-footer">
        <button type="button" class="bento-btn" onclick={() => (isRegisterGearOpen = false)}>Hủy bỏ</button>
        <button type="button" class="bento-btn btn-accent" onclick={handleRegisterGear}>Đăng Ký</button>
      </div>
    </div>
  </div>
{/if}

<!-- MODAL: Reserve Instrument -->
{#if isReserveOpen}
  <div class="modal-backdrop" onclick={() => (isReserveOpen = false)} role="presentation">
    <div class="bento-card modal-box" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="modal-header">
        <h3 class="modal-title">Đặt Mượn Nhạc Cụ Cho Buổi Tập</h3>
        <button type="button" class="close-btn" onclick={() => (isReserveOpen = false)}><X size={16} /></button>
      </div>
      <div class="modal-body">
        <label class="form-label">
          <span>Chọn Tiết Mục Biểu Diễn *</span>
          <select bind:value={reservePayload.music_number_id} class="bento-input">
            <option value="">-- Chọn bài hát --</option>
            {#each musicNumbers as num}
              <option value={num.number.id}>{num.number.title}</option>
            {/each}
          </select>
        </label>
        <label class="form-label">
          <span>Thứ Trong Tuần</span>
          <select bind:value={reservePayload.day_of_week} class="bento-input">
            {#each WEEKDAYS as day}
              <option value={day}>{day}</option>
            {/each}
          </select>
        </label>
        <label class="form-label">
          <span>Khung Giờ Tập</span>
          <select bind:value={reservePayload.slot_label} class="bento-input">
            {#each SLOTS as slot}
              <option value={slot}>{slot}</option>
            {/each}
          </select>
        </label>
      </div>
      <div class="modal-footer">
        <button type="button" class="bento-btn" onclick={() => (isReserveOpen = false)}>Hủy bỏ</button>
        <button type="button" class="bento-btn btn-accent" onclick={handleReserveInstrument}>Xác Nhận Đặt Mượn</button>
      </div>
    </div>
  </div>
{/if}

<!-- MODAL: QC Review -->
{#if isQcModalOpen}
  <div class="modal-backdrop" onclick={() => (isQcModalOpen = false)} role="presentation">
    <div class="bento-card modal-box" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="modal-header">
        <h3 class="modal-title">Kiểm Định Chất Lượng Tiết Mục (QC Review)</h3>
        <button type="button" class="close-btn" onclick={() => (isQcModalOpen = false)}><X size={16} /></button>
      </div>
      <div class="modal-body">
        <p><strong>Nhiệm vụ:</strong> {activeQcTask?.title}</p>
        <label class="form-label">
          <span>Nhận Xét & Đánh Giá Của Người Duyệt QC:</span>
          <textarea bind:value={qcFeedbackText} placeholder="Ghi chú về nhịp, cao độ, phối âm hoặc đoạn cần khắc phục..." class="bento-input" rows="4"></textarea>
        </label>
      </div>
      <div class="modal-footer">
        <button type="button" class="bento-btn" onclick={() => submitQcDecision('blocked')} style="color: var(--danger);">
          Yêu Cầu Sửa Lại (Blocked)
        </button>
        <button type="button" class="bento-btn btn-accent" onclick={() => submitQcDecision('passed')}>
          Phê Duyệt Đạt QC (Pass)
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- MODAL: Schedule Stub Info -->
{#if scheduleStubMsg}
  <div class="modal-backdrop" onclick={() => (scheduleStubMsg = null)} role="presentation">
    <div class="bento-card modal-box" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="modal-header">
        <h3 class="modal-title">Thông Báo Giải Thuật Xếp Lịch CSP</h3>
        <button type="button" class="close-btn" onclick={() => (scheduleStubMsg = null)}><X size={16} /></button>
      </div>
      <div class="modal-body">
        <p>{scheduleStubMsg}</p>
      </div>
      <div class="modal-footer">
        <button type="button" class="bento-btn btn-accent" onclick={() => (scheduleStubMsg = null)}>Đã hiểu</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .studio-container {
    max-width: 1300px;
    margin: 0 auto;
    padding: 24px 20px 80px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .studio-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    flex-wrap: wrap;
    gap: 16px;
    padding: 24px 28px;
    background: linear-gradient(135deg, rgba(255, 107, 0, 0.05), rgba(255, 255, 255, 0.95));
  }

  .tag-row {
    display: flex;
    gap: 8px;
    margin-bottom: 8px;
  }

  .live-tag {
    background: rgba(34, 197, 94, 0.15);
    color: #15803d;
    font-weight: 700;
  }

  .studio-heading {
    font-size: 24px;
    font-weight: 800;
    color: var(--text-primary);
    margin: 0 0 6px;
  }

  .studio-subheading {
    font-size: 14px;
    color: var(--text-secondary);
    margin: 0;
    max-width: 650px;
  }

  .role-switcher-box {
    display: flex;
    flex-direction: column;
    gap: 6px;
    align-items: flex-end;
  }

  .role-hint {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .tab-strip {
    display: flex;
    gap: 8px;
    border-bottom: 1px solid var(--border-subtle);
    padding-bottom: 4px;
  }

  .subtab-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 18px;
    border: none;
    background: transparent;
    font-size: 14px;
    font-weight: 600;
    color: var(--text-secondary);
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: all 0.2s;
  }

  .subtab-btn:hover {
    background: var(--surface-card-subtle);
    color: var(--text-primary);
  }

  .subtab-btn.is-active {
    background: var(--surface-card);
    color: var(--accent);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
    border-bottom: 2px solid var(--accent);
  }

  .badge-counter {
    background: var(--surface-card-subtle);
    font-size: 11px;
    padding: 2px 6px;
    border-radius: 99px;
    font-weight: 700;
  }

  .bento-grid-two {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }

  @media (max-width: 900px) {
    .bento-grid-two {
      grid-template-columns: 1fr;
    }
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 16px;
    flex-wrap: wrap;
    gap: 12px;
  }

  .panel-title {
    font-size: 18px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
  }

  .panel-subtitle {
    font-size: 12px;
    color: var(--text-muted);
  }

  .numbers-list, .tasks-board {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .number-card {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .number-card-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .genre-pill {
    font-size: 10px;
    text-transform: uppercase;
    margin-bottom: 4px;
  }

  .number-title {
    font-size: 16px;
    font-weight: 700;
    margin: 0;
    color: var(--text-primary);
  }

  .number-desc {
    font-size: 13px;
    color: var(--text-secondary);
    margin: 0;
  }

  .number-meta-row {
    display: flex;
    gap: 16px;
    font-size: 12px;
    color: var(--text-muted);
  }

  .meta-item {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .members-tag-wrap {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .member-role-tag {
    font-size: 11px;
    padding: 3px 8px;
    background: var(--surface-card-subtle);
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
  }

  .member-role-tag.is-lead {
    border-color: var(--accent);
    color: var(--accent-text);
  }

  .sprint-tag-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 4px;
  }

  .sprint-name {
    font-size: 12px;
    font-weight: 700;
    color: var(--accent);
  }

  .sprint-goal-box {
    padding: 10px 14px;
    background: rgba(255, 107, 0, 0.04);
    border: 1px solid rgba(255, 107, 0, 0.15);
    border-radius: var(--radius-sm);
    font-size: 12px;
    color: var(--text-primary);
    margin-bottom: 14px;
  }

  .task-card {
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .task-card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .task-type-badge {
    font-size: 11px;
    font-weight: 700;
    padding: 2px 6px;
    border-radius: 4px;
  }

  .task-type-badge.type-review_qc {
    background: rgba(255, 107, 0, 0.12);
    color: var(--accent);
  }

  .task-status-badge {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    padding: 2px 6px;
    border-radius: 4px;
  }

  .task-status-badge.status-passed {
    background: rgba(34, 197, 94, 0.15);
    color: #15803d;
  }

  .task-status-badge.status-under_review {
    background: rgba(234, 179, 8, 0.15);
    color: #854d0e;
  }

  .task-title {
    font-size: 14px;
    font-weight: 700;
    margin: 0;
  }

  .task-desc {
    font-size: 12px;
    color: var(--text-secondary);
    margin: 0;
  }

  .qc-feedback-box {
    padding: 8px 12px;
    background: var(--surface-card-subtle);
    border-radius: 6px;
    font-size: 12px;
    display: flex;
    align-items: center;
    gap: 6px;
    color: var(--text-primary);
  }

  /* Instrument Grid */
  .instruments-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 16px;
  }

  .instrument-card {
    padding: 18px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 14px;
  }

  .inst-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .category-pill {
    font-size: 10px;
    margin-bottom: 4px;
  }

  .inst-name {
    font-size: 15px;
    font-weight: 700;
    margin: 0;
  }

  .inst-availability-badge {
    font-size: 10px;
    font-weight: 700;
    padding: 3px 8px;
    border-radius: 99px;
    text-transform: uppercase;
  }

  .inst-availability-badge.status-free_to_borrow {
    background: rgba(34, 197, 94, 0.15);
    color: #15803d;
  }

  .inst-availability-badge.status-unavailable {
    background: rgba(100, 116, 139, 0.15);
    color: #475569;
  }

  .inst-info-block {
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-size: 12px;
  }

  .tag-club {
    color: #0284c7;
    font-weight: 600;
  }

  .tag-member {
    color: var(--accent);
    font-weight: 600;
  }

  .inst-notes {
    font-style: italic;
    color: var(--text-muted);
    font-size: 11px;
  }

  .inst-actions {
    display: flex;
    gap: 8px;
    border-top: 1px solid var(--border-subtle);
    padding-top: 10px;
  }

  /* Free Time Grid */
  .freetime-table-wrapper {
    overflow-x: auto;
  }

  .freetime-table {
    width: 100%;
    border-collapse: collapse;
    text-align: center;
  }

  .th-slot, .th-day {
    padding: 12px;
    font-size: 12px;
    font-weight: 700;
    border-bottom: 2px solid var(--border-subtle);
    color: var(--text-secondary);
  }

  .td-slot-label {
    padding: 12px;
    font-size: 13px;
    font-weight: 600;
    border-bottom: 1px solid var(--border-subtle);
    text-align: left;
  }

  .td-cell {
    padding: 8px;
    border: 1px solid var(--border-subtle);
    cursor: pointer;
    transition: background 0.15s;
  }

  .td-cell:hover {
    background: rgba(255, 107, 0, 0.08);
  }

  .td-cell.is-available {
    background: var(--success);
    color: white;
  }

  .cell-content {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    min-height: 36px;
  }

  .cell-label {
    font-size: 11px;
    font-weight: 700;
  }

  .cell-dash {
    color: var(--text-muted);
  }

  /* Modals */
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(4px);
  }

  .modal-box {
    width: 90%;
    max-width: 480px;
    padding: 24px;
    background: var(--surface-card);
    border-radius: var(--radius-md);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .modal-title {
    font-size: 16px;
    font-weight: 700;
    margin: 0;
  }

  .modal-body {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-bottom: 18px;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
  }

  .form-label {
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .bento-input {
    padding: 8px 12px;
    border: 1px solid var(--border-card);
    border-radius: var(--radius-sm);
    font-size: 13px;
    background: var(--surface-card);
  }

  .mini-btn {
    padding: 5px 10px;
    font-size: 11px;
  }

  .btn-accent {
    background: var(--accent);
    color: white;
    border: none;
  }

  .btn-accent:hover {
    background: #ea580c;
  }

  .btn-primary {
    background: var(--surface-card);
    border: 1px solid var(--accent);
    color: var(--accent);
  }

  .success-banner {
    padding: 12px 16px;
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.3);
    color: #15803d;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .error-banner {
    padding: 12px 16px;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #b91c1c;
    font-weight: 600;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .close-btn {
    background: none;
    border: none;
    cursor: pointer;
    color: inherit;
  }
</style>
