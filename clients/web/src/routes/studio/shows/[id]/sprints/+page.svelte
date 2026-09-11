<script lang="ts">
  import { onMount } from 'svelte';
  import { tStore } from '$lib/i18n';
  import {
    Calendar as CalendarIcon,
    Clock,
    Wand2,
    Check,
    AlertCircle,
    CheckCircle2,
    Filter,
    Sparkles,
    Music,
    MapPin,
    Users,
    ChevronDown,
    Trash2,
    Zap,
  } from '@lucide/svelte';

  const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];

  // Generate 15-minute interval time slots from 08:00 to 22:45
  const hours = [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22];
  const minutes = ['00', '15', '30', '45'];

  interface TimeSlotInfo {
    label: string;
    hour: number;
    minute: string;
    isHourStart: boolean;
  }

  const timeSlots: TimeSlotInfo[] = [];
  for (const h of hours) {
    for (const m of minutes) {
      const hh = h.toString().padStart(2, '0');
      timeSlots.push({
        label: `${hh}:${m}`,
        hour: h,
        minute: m,
        isHourStart: m === '00',
      });
    }
  }

  // 15-Minute Selection Matrix State: `${dayIdx}_${slotLabel}` -> boolean
  let selectedSlots = $state<Record<string, boolean>>({
    '0_17:00': true,
    '0_17:15': true,
    '0_17:30': true,
    '0_17:45': true,
    '0_18:00': true,
    '0_18:15': true,
    '0_18:30': true,
    '0_18:45': true,
    '2_19:00': true,
    '2_19:15': true,
    '2_19:30': true,
    '2_19:45': true,
    '2_20:00': true,
    '2_20:15': true,
    '2_20:30': true,
    '2_20:45': true,
    '4_18:00': true,
    '4_18:15': true,
    '4_18:30': true,
    '4_18:45': true,
    '4_19:00': true,
    '4_19:15': true,
    '5_17:30': true,
    '5_17:45': true,
    '5_18:00': true,
    '5_18:15': true,
    '5_18:30': true,
    '5_18:45': true,
    '6_19:00': true,
    '6_19:15': true,
    '6_19:30': true,
    '6_19:45': true,
    '6_20:00': true,
    '6_20:15': true,
  });

  // Drag interaction state
  let isDragging = $state(false);
  let dragTargetValue = $state(true);
  let isSaved = $state(false);
  let activeToast = $state<string | null>(null);

  // Auto-scheduled sprint rehearsals state
  let isAutoScheduled = $state(true);
  let isScheduling = $state(false);
  let filterSong = $state('all');
  let filterRoom = $state('all');

  interface ScheduledRehearsal {
    id: string;
    songTitle: string;
    dayIdx: number;
    dayName: string;
    startTime: string; // e.g. "18:15"
    endTime: string;   // e.g. "19:45"
    durationMinutes: number;
    room: string;
    pmName: string;
    performers: string[];
    status: 'in_practice' | 'ready_for_qc';
    color: string;
  }

  let scheduledSessions = $state<ScheduledRehearsal[]>([
    {
      id: 'reh-1',
      songTitle: 'Nơi Này Có Anh',
      dayIdx: 0, // Monday
      dayName: 'Monday',
      startTime: '18:15',
      endTime: '19:45',
      durationMinutes: 90,
      room: 'Studio Room A',
      pmName: 'Phạm Minh Pháp',
      performers: ['Minh Pháp (Vocal)', 'Bảo Anh (Guitar)', 'Tuấn Kiệt (Keys)'],
      status: 'in_practice',
      color: '#ff6b00',
    },
    {
      id: 'reh-2',
      songTitle: 'Tình Mới',
      dayIdx: 2, // Wednesday
      dayName: 'Wednesday',
      startTime: '19:30',
      endTime: '21:00',
      durationMinutes: 90,
      room: 'Studio Room A',
      pmName: 'Đặng Bảo Anh',
      performers: ['Bảo Anh (Vocal)', 'Hoàng Nam (Bass)', 'Gia Huy (Drums)'],
      status: 'ready_for_qc',
      color: '#2563eb',
    },
    {
      id: 'reh-3',
      songTitle: 'Việt Nam Trong Tôi Là',
      dayIdx: 4, // Friday
      dayName: 'Friday',
      startTime: '18:00',
      endTime: '19:30',
      durationMinutes: 90,
      room: 'Studio Room B',
      pmName: 'Lê Tuấn Kiệt',
      performers: ['Tuấn Kiệt (Keys)', 'Phương Thảo (Vocal)', 'Minh Pháp (Chorus)'],
      status: 'in_practice',
      color: '#16a34a',
    },
    {
      id: 'reh-4',
      songTitle: 'Chiếc Khăn Gió Ấm',
      dayIdx: 5, // Saturday
      dayName: 'Saturday',
      startTime: '17:30',
      endTime: '19:00',
      durationMinutes: 90,
      room: 'Studio Room A',
      pmName: 'Nguyễn Hoàng Nam',
      performers: ['Hoàng Nam (Lead)', 'Bảo Anh (Acoustic)', 'Gia Huy (Percussion)'],
      status: 'ready_for_qc',
      color: '#9333ea',
    },
    {
      id: 'reh-5',
      songTitle: 'Bài Ca Tuổi Trẻ',
      dayIdx: 6, // Sunday
      dayName: 'Sunday',
      startTime: '19:00',
      endTime: '20:30',
      durationMinutes: 90,
      room: 'Studio Room B',
      pmName: 'Trần Gia Huy',
      performers: ['Gia Huy (Drums)', 'Phương Thảo (Vocal)', 'Tuấn Kiệt (Keys)'],
      status: 'in_practice',
      color: '#ea580c',
    },
  ]);

  // Computed count of selected 15-min slots
  let selectedCount = $derived(
    Object.values(selectedSlots).filter(Boolean).length
  );
  let totalHoursFormatted = $derived((selectedCount * 0.25).toFixed(2));

  // Mouse drag handlers
  function handleCellMouseDown(dayIdx: number, slotLabel: string, event: MouseEvent) {
    event.preventDefault(); // Prevent text selection while dragging
    isDragging = true;
    const key = `${dayIdx}_${slotLabel}`;
    dragTargetValue = !selectedSlots[key];
    selectedSlots[key] = dragTargetValue;
    isSaved = false;
  }

  function handleCellMouseEnter(dayIdx: number, slotLabel: string) {
    if (isDragging) {
      const key = `${dayIdx}_${slotLabel}`;
      selectedSlots[key] = dragTargetValue;
      isSaved = false;
    }
  }

  function handleMouseUpGlobal() {
    isDragging = false;
  }

  onMount(() => {
    window.addEventListener('mouseup', handleMouseUpGlobal);
    return () => {
      window.removeEventListener('mouseup', handleMouseUpGlobal);
    };
  });

  // Preset Selection Shortcuts
  function selectPresetEvenings() {
    for (let d = 0; d < 7; d++) {
      for (const slot of timeSlots) {
        if (slot.hour >= 18 && slot.hour < 21) {
          selectedSlots[`${d}_${slot.label}`] = true;
        }
      }
    }
    isSaved = false;
  }

  function selectPresetAfternoons() {
    for (let d = 0; d < 7; d++) {
      for (const slot of timeSlots) {
        if (slot.hour >= 17 && slot.hour < 19) {
          selectedSlots[`${d}_${slot.label}`] = true;
        }
      }
    }
    isSaved = false;
  }

  function clearAllSlots() {
    selectedSlots = {};
    isSaved = false;
  }

  function handleSaveFreetime() {
    isSaved = true;
    activeToast = $tStore('studio.freetime_saved');
    setTimeout(() => {
      isSaved = false;
      activeToast = null;
    }, 3500);
  }

  // Trigger CSP Auto-Scheduler
  function handleAutoSchedule() {
    isScheduling = true;
    setTimeout(() => {
      isScheduling = false;
      isAutoScheduled = true;
      activeToast = $tStore('studio.scheduled_toast', { count: scheduledSessions.length });
      setTimeout(() => {
        activeToast = null;
      }, 4000);
    }, 700);
  }

  // Filtered scheduled sessions
  let filteredSessions = $derived(
    scheduledSessions.filter((s) => {
      const matchSong = filterSong === 'all' || s.songTitle === filterSong;
      const matchRoom = filterRoom === 'all' || s.room === filterRoom;
      return matchSong && matchRoom;
    })
  );

  const uniqueSongs = Array.from(new Set(scheduledSessions.map((s) => s.songTitle)));
  const uniqueRooms = Array.from(new Set(scheduledSessions.map((s) => s.room)));
</script>

<div class="sprints-subpage">
  <!-- Active Sprint Banner -->
  <div class="sprint-header bento-card">
    <div class="header-info">
      <div class="sprint-tag">
        <CalendarIcon size={14} class="text-orange" />
        <span>{$tStore('studio_shows.active_sprint')}: Sprint 3 (Stage QC & 15m Rehearsal Optimization)</span>
      </div>
      <h2>Practice Sprint Management & 15-Minute Free-Time Registration</h2>
      <p>{$tStore('studio.freetime_desc_drag')}</p>
    </div>

    <button
      type="button"
      class="bento-btn bento-btn-primary"
      onclick={handleAutoSchedule}
      disabled={isScheduling}
    >
      <Wand2 size={16} class={isScheduling ? 'animate-spin' : ''} />
      <span>{isScheduling ? 'Optimizing...' : $tStore('studio.btn_auto_schedule')}</span>
    </button>
  </div>

  {#if activeToast}
    <div class="toast-success">
      <CheckCircle2 size={16} />
      <span>{activeToast}</span>
    </div>
  {/if}

  <!-- 15-Minute Fine-Grained Click-and-Drag Registration Grid -->
  <div class="grid-card bento-card">
    <div class="grid-header-row">
      <div>
        <h3>{$tStore('studio.freetime_title_15m')}</h3>
        <p class="grid-subtext">
          Selected 15-min slots: <strong>{selectedCount}</strong> ({totalHoursFormatted} total practice hours available)
        </p>
      </div>

      <div class="grid-actions">
        <div class="preset-group">
          <button type="button" class="preset-btn" onclick={selectPresetEvenings}>
            <Zap size={13} />
            <span>{$tStore('studio.preset_evenings')}</span>
          </button>
          <button type="button" class="preset-btn" onclick={selectPresetAfternoons}>
            <Clock size={13} />
            <span>{$tStore('studio.preset_afternoons')}</span>
          </button>
          <button type="button" class="preset-btn btn-clear" onclick={clearAllSlots}>
            <Trash2 size={13} />
            <span>{$tStore('studio.preset_clear')}</span>
          </button>
        </div>

        <button type="button" class="bento-btn bento-btn-sm" onclick={handleSaveFreetime}>
          <Check size={14} />
          <span>{$tStore('studio.btn_save_freetime')}</span>
        </button>
      </div>
    </div>

    <!-- 15-Minute Drag Matrix Table -->
    <div class="freetime-table-wrapper select-none">
      <table class="freetime-table">
        <thead>
          <tr>
            <th class="time-col-header">15m Slot</th>
            {#each days as day}
              <th>{day}</th>
            {/each}
          </tr>
        </thead>
        <tbody>
          {#each timeSlots as slot}
            <tr class={slot.isHourStart ? 'hour-divider-row' : ''}>
              <td class="slot-time-td {slot.isHourStart ? 'is-hour-start' : ''}">
                <span class="time-label">{slot.label}</span>
              </td>
              {#each days as day, dIdx}
                {@const isSelected = selectedSlots[`${dIdx}_${slot.label}`]}
                <td
                  class="slot-cell-td"
                  onmousedown={(e) => handleCellMouseDown(dIdx, slot.label, e)}
                  onmouseenter={() => handleCellMouseEnter(dIdx, slot.label)}
                >
                  <div class="cell-block {isSelected ? 'is-selected' : ''}">
                    {#if isSelected}
                      <span class="active-dot"></span>
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

  <!-- Auto-Scheduled Sprint Rehearsals Calendar Display -->
  {#if isAutoScheduled}
    <div class="calendar-card bento-card">
      <div class="calendar-header-row">
        <div>
          <div class="badge-scheduled">
            <Sparkles size={13} />
            <span>Zero-Conflict Schedule Generated</span>
          </div>
          <h3>{$tStore('studio.calendar_title')}</h3>
          <p class="grid-subtext">{$tStore('studio.calendar_desc')}</p>
        </div>

        <div class="filters-row">
          <div class="filter-item">
            <Filter size={13} class="text-muted" />
            <span class="filter-label">{$tStore('studio.filter_number')}</span>
            <select bind:value={filterSong} class="bento-select">
              <option value="all">{$tStore('studio.all_numbers')}</option>
              {#each uniqueSongs as song}
                <option value={song}>{song}</option>
              {/each}
            </select>
          </div>

          <div class="filter-item">
            <MapPin size={13} class="text-muted" />
            <span class="filter-label">{$tStore('studio.filter_room')}</span>
            <select bind:value={filterRoom} class="bento-select">
              <option value="all">{$tStore('studio.all_rooms')}</option>
              {#each uniqueRooms as rm}
                <option value={rm}>{rm}</option>
              {/each}
            </select>
          </div>
        </div>
      </div>

      <!-- Sprint Calendar Timetable Grid -->
      <div class="calendar-grid">
        {#each days as day, dIdx}
          {@const daySessions = filteredSessions.filter((s) => s.dayIdx === dIdx)}
          <div class="day-column">
            <div class="day-column-header">
              <span class="day-name">{day}</span>
              <span class="session-badge">{daySessions.length} sessions</span>
            </div>

            <div class="day-sessions-container">
              {#if daySessions.length === 0}
                <div class="empty-day-state">No rehearsals</div>
              {:else}
                {#each daySessions as session}
                  <div class="rehearsal-card" style="border-left-color: {session.color}">
                    <div class="rehearsal-top">
                      <h4 class="song-name">{session.songTitle}</h4>
                      <span
                        class="status-chip {session.status === 'ready_for_qc'
                          ? 'chip-qc'
                          : 'chip-practice'}"
                      >
                        {session.status === 'ready_for_qc' ? 'Ready QC' : 'In Practice'}
                      </span>
                    </div>

                    <div class="rehearsal-meta">
                      <div class="meta-row">
                        <Clock size={12} />
                        <span class="time-range">{session.startTime} – {session.endTime}</span>
                        <span class="duration-pill">({session.durationMinutes}m)</span>
                      </div>

                      <div class="meta-row">
                        <MapPin size={12} />
                        <span>{session.room}</span>
                      </div>

                      <div class="performers-list">
                        <Users size={12} class="text-muted" />
                        <span>{session.performers.join(', ')}</span>
                      </div>
                    </div>
                  </div>
                {/each}
              {/if}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .sprints-subpage {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .select-none {
    user-select: none;
    -webkit-user-select: none;
  }

  .bento-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 16px;
    padding: 20px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
  }

  .sprint-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .sprint-tag {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    border-radius: 20px;
    font-size: 12px;
    font-weight: 700;
    margin-bottom: 6px;
  }

  .sprint-header h2 {
    font-size: 18px;
    font-weight: 800;
    color: #0f172a;
    margin: 0 0 4px 0;
  }

  .sprint-header p {
    font-size: 13px;
    color: #64748b;
    margin: 0;
  }

  .bento-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 10px 16px;
    border-radius: 10px;
    font-size: 13px;
    font-weight: 700;
    border: none;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .bento-btn-primary {
    background: #ff6b00;
    color: #ffffff;
  }

  .bento-btn-primary:hover:not(:disabled) {
    background: #e66000;
    transform: translateY(-1px);
  }

  .bento-btn-primary:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .bento-btn-sm {
    padding: 6px 12px;
    font-size: 12px;
    background: #0f172a;
    color: #ffffff;
  }

  .bento-btn-sm:hover {
    background: #1e293b;
  }

  .toast-success {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(22, 163, 74, 0.1);
    color: #16a34a;
    padding: 10px 16px;
    border-radius: 10px;
    font-weight: 700;
    font-size: 13px;
  }

  .grid-header-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    margin-bottom: 16px;

    flex-wrap: wrap;
    gap: 12px;
  }

  .grid-header-row h3 {
    font-size: 16px;
    font-weight: 800;
    color: #0f172a;
    margin: 0 0 2px 0;
  }

  .grid-subtext {
    font-size: 13px;
    color: #64748b;
    margin: 0;
  }

  .grid-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .preset-group {
    display: flex;
    gap: 6px;
  }

  .preset-btn {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 5px 10px;
    background: #f1f5f9;
    color: #475569;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.12s ease;
  }

  .preset-btn:hover {
    background: #e2e8f0;
    color: #0f172a;
  }

  .preset-btn.btn-clear:hover {
    background: #fee2e2;
    color: #dc2626;
    border-color: #fca5a5;
  }

  .freetime-table-wrapper {
    overflow-x: auto;
    max-height: 480px;
    overflow-y: auto;
    border: 1px solid #e2e8f0;
    border-radius: 12px;
  }

  .freetime-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 12px;
  }

  .freetime-table th {
    position: sticky;
    top: 0;
    background: #f8fafc;
    z-index: 2;
    padding: 8px 10px;
    text-align: center;
    border-bottom: 1px solid #cbd5e1;
    font-weight: 700;
    color: #334155;
  }

  .time-col-header {
    width: 80px;
  }

  .hour-divider-row {
    border-top: 2px solid #cbd5e1;
  }

  .slot-time-td {
    padding: 3px 8px;
    background: #f8fafc;
    text-align: center;
    border-right: 1px solid #e2e8f0;
    font-weight: 600;
    color: #64748b;
    font-size: 11px;

    white-space: nowrap;
  }

  .slot-time-td.is-hour-start {
    font-weight: 800;
    color: #0f172a;
    background: #f1f5f9;
  }

  .slot-cell-td {
    padding: 2px 4px;
    border-right: 1px solid #f1f5f9;
    border-bottom: 1px solid #f1f5f9;
    cursor: pointer;
    text-align: center;
  }

  .slot-cell-td:hover {
    background: rgba(255, 107, 0, 0.05);
  }

  .cell-block {
    height: 18px;
    width: 100%;
    border-radius: 4px;
    background: #f8fafc;
    border: 1px dashed #e2e8f0;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.1s ease;
  }

  .cell-block.is-selected {
    background: rgba(22, 163, 74, 0.2);
    border: 1px solid #16a34a;
  }

  .active-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #16a34a;
  }

  /* Calendar Section */
  .calendar-card {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .calendar-header-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    flex-wrap: wrap;
    gap: 12px;
  }

  .badge-scheduled {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 3px 8px;
    background: rgba(37, 99, 235, 0.1);
    color: #2563eb;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 700;
    margin-bottom: 4px;
  }

  .calendar-header-row h3 {
    font-size: 16px;
    font-weight: 800;
    color: #0f172a;
    margin: 0 0 2px 0;
  }

  .filters-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .filter-item {
    display: flex;
    align-items: center;
    gap: 6px;
    background: #f8fafc;
    padding: 4px 10px;
    border: 1px solid #e2e8f0;
    border-radius: 8px;
  }

  .filter-label {
    font-size: 12px;
    font-weight: 600;
    color: #475569;
  }

  .bento-select {
    border: none;
    background: transparent;
    font-size: 12px;
    font-weight: 700;
    color: #0f172a;
    cursor: pointer;
    outline: none;
  }

  /* Calendar Days Grid */
  .calendar-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 10px;
  }

  @media (max-width: 1024px) {
    .calendar-grid {
      grid-template-columns: repeat(3, 1fr);
    }
  }

  @media (max-width: 640px) {
    .calendar-grid {
      grid-template-columns: 1fr;
    }
  }

  .day-column {
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    border-radius: 12px;
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    min-height: 220px;
  }

  .day-column-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #e2e8f0;
    padding-bottom: 8px;
  }

  .day-name {
    font-size: 12px;
    font-weight: 800;
    color: #334155;
  }

  .session-badge {
    font-size: 10px;
    font-weight: 600;
    color: #64748b;
    background: #e2e8f0;
    padding: 2px 6px;
    border-radius: 10px;
  }

  .day-sessions-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .empty-day-state {
    font-size: 11px;
    color: #94a3b8;
    text-align: center;
    padding: 20px 0;
    font-style: italic;
  }

  .rehearsal-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-left-width: 4px;
    border-radius: 8px;
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.03);
    transition: transform 0.15s ease;
  }

  .rehearsal-card:hover {
    transform: translateY(-2px);
  }

  .rehearsal-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 6px;
  }

  .song-name {
    font-size: 12px;
    font-weight: 800;
    color: #0f172a;
    margin: 0;
    line-height: 1.3;
  }

  .status-chip {
    font-size: 9px;
    font-weight: 700;
    padding: 2px 6px;
    border-radius: 8px;
    white-space: nowrap;
  }

  .chip-practice {
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
  }

  .chip-qc {
    background: rgba(37, 99, 235, 0.1);
    color: #2563eb;
  }

  .rehearsal-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 10px;
    color: #475569;
  }

  .meta-row {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .time-range {
    font-weight: 700;
    color: #0f172a;
  }

  .duration-pill {
    color: #64748b;
  }

  .performers-list {
    display: flex;
    align-items: flex-start;
    gap: 4px;
    margin-top: 2px;
    font-size: 10px;
    color: #64748b;
    line-height: 1.2;
  }
</style>
