<script lang="ts">
  import { onMount } from 'svelte';
  import { tStore } from '$lib/i18n';
  import { page } from '$app/stores';
  import { canTriggerScheduler, canViewHistory, hasRole } from '$lib/auth';
  import type { UserRole, ScheduleRunHistoryItem, AvailabilityHistoryItem } from '$lib/types/timetable';
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
    History as HistoryIcon,
    Radio,
    X,
    Activity,
    LayoutGrid,
    Table,
  } from '@lucide/svelte';

  const userRole = $derived(($page.data?.user?.role || 'admin') as UserRole);

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

  // Auto-scheduled sprint rehearsals state & SSE Real-time streaming
  let isAutoScheduled = $state(true);
  let isScheduling = $state(false);
  let sseStatus = $state<'connected' | 'syncing' | 'idle'>('connected');
  let filterSong = $state('all');
  let filterRoom = $state('all');

  // History Drawer State (PM, DM, Moderator, Admin)
  let isHistoryOpen = $state(false);
  let activeHistoryTab = $state<'compute' | 'registration'>('compute');

  // Mock / Fetched Compute Run History
  let computeHistory = $state<ScheduleRunHistoryItem[]>([
    {
      id: 'run-101',
      sprintId: 'sprint-3',
      triggeredBy: 'user-001',
      triggeredByName: 'Minh Pháp (DM)',
      status: 'completed',
      durationMs: 420,
      score: 98.5,
      conflictCount: 0,
      createdAt: '2026-09-12 09:30:15',
      completedAt: '2026-09-12 09:30:16',
    },
    {
      id: 'run-100',
      sprintId: 'sprint-3',
      triggeredBy: 'user-002',
      triggeredByName: 'Hoàng Nam (Admin)',
      status: 'completed',
      durationMs: 650,
      score: 92.0,
      conflictCount: 1,
      createdAt: '2026-09-11 14:15:00',
      completedAt: '2026-09-11 14:15:01',
    },
  ]);

  // Mock / Fetched Free-Time Registration History
  let registrationHistory = $state<AvailabilityHistoryItem[]>([
    {
      id: 'reg-501',
      sprintId: 'sprint-3',
      userId: 'user-003',
      userName: 'Thu Hà (Member)',
      actorId: 'user-003',
      actorName: 'Thu Hà (Self)',
      action: 'ADD',
      dayOfWeek: 'Monday',
      slotLabel: '18:15',
      isAvailable: true,
      createdAt: '2026-09-12 10:12:00',
    },
    {
      id: 'reg-502',
      sprintId: 'sprint-3',
      userId: 'user-004',
      userName: 'Tuấn Kiệt (PM)',
      actorId: 'user-001',
      actorName: 'Minh Pháp (DM)',
      action: 'UPDATE',
      dayOfWeek: 'Friday',
      slotLabel: '19:00',
      isAvailable: true,
      createdAt: '2026-09-12 08:45:10',
    },
    {
      id: 'reg-503',
      sprintId: 'sprint-3',
      userId: 'user-005',
      userName: 'Bảo Anh (Member)',
      actorId: 'user-005',
      actorName: 'Bảo Anh (Self)',
      action: 'DELETE',
      dayOfWeek: 'Wednesday',
      slotLabel: '21:00',
      isAvailable: false,
      createdAt: '2026-09-11 19:30:22',
    },
  ]);

  interface ScheduledRehearsal {
    id: string;
    songTitle: string;
    sessionIndex: number;          // e.g. 1 (for #1 of 2)
    totalTargetRehearsals: number;  // e.g. 2
    dayIdx: number;
    dayName: string;
    startTime: string; // e.g. "18:15"
    endTime: string;   // e.g. "19:45"
    durationMinutes: number;
    room: string;
    pmName: string;
    performers: string[];
    status: 'in_practice' | 'ready_for_qc' | 'qc_approved' | 'stage_ready';
    color: string;
  }

  let scheduledSessions = $state<ScheduledRehearsal[]>([
    // Song 1: Hào Khí Việt Nam (3 rehearsals target, 3 scheduled)
    {
      id: 'reh-1a',
      songTitle: 'Hào Khí Việt Nam',
      sessionIndex: 1,
      totalTargetRehearsals: 3,
      dayIdx: 0, // Monday
      dayName: 'Monday',
      startTime: '18:15',
      endTime: '19:45',
      durationMinutes: 90,
      room: 'Studio Room A',
      pmName: 'Minh Pháp',
      performers: ['Minh Pháp (Vocal)', 'Hoàng Nam (Guitar)', 'Bảo Anh (Bass)', 'Thu Hà (Drums)'],
      status: 'stage_ready',
      color: '#ff6b00',
    },
    {
      id: 'reh-1b',
      songTitle: 'Hào Khí Việt Nam',
      sessionIndex: 2,
      totalTargetRehearsals: 3,
      dayIdx: 2, // Wednesday
      dayName: 'Wednesday',
      startTime: '19:30',
      endTime: '21:00',
      durationMinutes: 90,
      room: 'Studio Room A',
      pmName: 'Minh Pháp',
      performers: ['Minh Pháp (Vocal)', 'Hoàng Nam (Guitar)', 'Bảo Anh (Bass)', 'Thu Hà (Drums)'],
      status: 'stage_ready',
      color: '#ff6b00',
    },
    {
      id: 'reh-1c',
      songTitle: 'Hào Khí Việt Nam',
      sessionIndex: 3,
      totalTargetRehearsals: 3,
      dayIdx: 5, // Saturday
      dayName: 'Saturday',
      startTime: '17:00',
      endTime: '18:30',
      durationMinutes: 90,
      room: 'Studio Room A',
      pmName: 'Minh Pháp',
      performers: ['Minh Pháp (Vocal)', 'Hoàng Nam (Guitar)', 'Bảo Anh (Bass)', 'Thu Hà (Drums)'],
      status: 'stage_ready',
      color: '#ff6b00',
    },

    // Song 2: Đi Giữa Trời Rực Rỡ (2 rehearsals target, 2 scheduled)
    {
      id: 'reh-2a',
      songTitle: 'Đi Giữa Trời Rực Rỡ',
      sessionIndex: 1,
      totalTargetRehearsals: 2,
      dayIdx: 1, // Tuesday
      dayName: 'Tuesday',
      startTime: '18:30',
      endTime: '20:00',
      durationMinutes: 90,
      room: 'Studio Room B',
      pmName: 'Hoàng Nam',
      performers: ['Gia Huy (Vocal)', 'Hoàng Nam (Guitar)', 'Bảo Anh (Bass)', 'Thu Hà (Drums)'],
      status: 'qc_approved',
      color: '#2563eb',
    },
    {
      id: 'reh-2b',
      songTitle: 'Đi Giữa Trời Rực Rỡ',
      sessionIndex: 2,
      totalTargetRehearsals: 2,
      dayIdx: 4, // Friday
      dayName: 'Friday',
      startTime: '19:00',
      endTime: '20:30',
      durationMinutes: 90,
      room: 'Studio Room A',
      pmName: 'Hoàng Nam',
      performers: ['Gia Huy (Vocal)', 'Hoàng Nam (Guitar)', 'Bảo Anh (Bass)', 'Thu Hà (Drums)'],
      status: 'qc_approved',
      color: '#2563eb',
    },

    // Song 3: Giọt Sương Trên Mí Mắt (2 rehearsals target, 2 scheduled)
    {
      id: 'reh-3a',
      songTitle: 'Giọt Sương Trên Mí Mắt',
      sessionIndex: 1,
      totalTargetRehearsals: 2,
      dayIdx: 0, // Monday
      dayName: 'Monday',
      startTime: '20:00',
      endTime: '21:30',
      durationMinutes: 90,
      room: 'Studio Room B',
      pmName: 'Bảo Anh',
      performers: ['Minh Pháp (Vocal)', 'Tùng Dương (Guitar)', 'Bảo Anh (Bass)'],
      status: 'ready_for_qc',
      color: '#16a34a',
    },
    {
      id: 'reh-3b',
      songTitle: 'Giọt Sương Trên Mí Mắt',
      sessionIndex: 2,
      totalTargetRehearsals: 2,
      dayIdx: 3, // Thursday
      dayName: 'Thursday',
      startTime: '18:15',
      endTime: '19:45',
      durationMinutes: 90,
      room: 'Studio Room A',
      pmName: 'Bảo Anh',
      performers: ['Minh Pháp (Vocal)', 'Tùng Dương (Guitar)', 'Bảo Anh (Bass)'],
      status: 'ready_for_qc',
      color: '#16a34a',
    },

    // Song 4: Nối Vòng Tay Lớn (2 rehearsals target, 2 scheduled)
    {
      id: 'reh-4a',
      songTitle: 'Nối Vòng Tay Lớn',
      sessionIndex: 1,
      totalTargetRehearsals: 2,
      dayIdx: 1, // Tuesday
      dayName: 'Tuesday',
      startTime: '17:00',
      endTime: '18:30',
      durationMinutes: 90,
      room: 'Studio Room A',
      pmName: 'Thu Hà',
      performers: ['Anh Pha (Vocal)', 'Hoàng Nam (Guitar)', 'Bảo Anh (Bass)', 'Thu Hà (Drums)'],
      status: 'in_practice',
      color: '#9333ea',
    },
    {
      id: 'reh-4b',
      songTitle: 'Nối Vòng Tay Lớn',
      sessionIndex: 2,
      totalTargetRehearsals: 2,
      dayIdx: 4, // Friday
      dayName: 'Friday',
      startTime: '17:30',
      endTime: '19:00',
      durationMinutes: 90,
      room: 'Studio Room B',
      pmName: 'Thu Hà',
      performers: ['Anh Pha (Vocal)', 'Hoàng Nam (Guitar)', 'Bảo Anh (Bass)', 'Thu Hà (Drums)'],
      status: 'in_practice',
      color: '#9333ea',
    },

    // Song 5: Túy Âm (2 rehearsals target, 2 scheduled)
    {
      id: 'reh-5a',
      songTitle: 'Túy Âm',
      sessionIndex: 1,
      totalTargetRehearsals: 2,
      dayIdx: 2, // Wednesday
      dayName: 'Wednesday',
      startTime: '18:00',
      endTime: '19:30',
      durationMinutes: 90,
      room: 'Studio Room B',
      pmName: 'Gia Huy',
      performers: ['Gia Huy (Vocal)', 'Bảo Anh (Bass)', 'Phương Nhi (Keys)', 'Thu Hà (Drums)'],
      status: 'stage_ready',
      color: '#ea580c',
    },
    {
      id: 'reh-5b',
      songTitle: 'Túy Âm',
      sessionIndex: 2,
      totalTargetRehearsals: 2,
      dayIdx: 6, // Sunday
      dayName: 'Sunday',
      startTime: '19:00',
      endTime: '20:30',
      durationMinutes: 90,
      room: 'Studio Room A',
      pmName: 'Gia Huy',
      performers: ['Gia Huy (Vocal)', 'Bảo Anh (Bass)', 'Phương Nhi (Keys)', 'Thu Hà (Drums)'],
      status: 'stage_ready',
      color: '#ea580c',
    },

    // Song 6: Đất Nước Trọn Niềm Vui (2 rehearsals target, 2 scheduled)
    {
      id: 'reh-6a',
      songTitle: 'Đất Nước Trọn Niềm Vui',
      sessionIndex: 1,
      totalTargetRehearsals: 2,
      dayIdx: 3, // Thursday
      dayName: 'Thursday',
      startTime: '19:45',
      endTime: '21:15',
      durationMinutes: 90,
      room: 'Studio Room B',
      pmName: 'Minh Pháp',
      performers: ['Minh Pháp (Vocal)', 'Phương Nhi (Keys)', 'Thu Hà (Drums)'],
      status: 'ready_for_qc',
      color: '#0891b2',
    },
    {
      id: 'reh-6b',
      songTitle: 'Đất Nước Trọn Niềm Vui',
      sessionIndex: 2,
      totalTargetRehearsals: 2,
      dayIdx: 5, // Saturday
      dayName: 'Saturday',
      startTime: '18:45',
      endTime: '20:15',
      durationMinutes: 90,
      room: 'Studio Room B',
      pmName: 'Minh Pháp',
      performers: ['Minh Pháp (Vocal)', 'Phương Nhi (Keys)', 'Thu Hà (Drums)'],
      status: 'ready_for_qc',
      color: '#0891b2',
    },

    // Song 7: Khoảnh Khắc (1 rehearsal target, 1 scheduled)
    {
      id: 'reh-7a',
      songTitle: 'Khoảnh Khắc',
      sessionIndex: 1,
      totalTargetRehearsals: 1,
      dayIdx: 6, // Sunday
      dayName: 'Sunday',
      startTime: '17:30',
      endTime: '19:00',
      durationMinutes: 90,
      room: 'Studio Room B',
      pmName: 'Thu Hà',
      performers: ['Thu Hà (Vocal)', 'Tùng Dương (Guitar)', 'Bảo Anh (Bass)'],
      status: 'in_practice',
      color: '#4f46e5',
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

  // Schedule View Mode & Day Filter State
  let scheduleView = $state<'grid' | 'timeline'>('grid');
  let filterDay = $state('all');

  // Rehearsal Quota Metrics
  const quotaMetrics = $derived.by(() => {
    const totalSessions = scheduledSessions.length;
    const uniqueSongsCount = new Set(scheduledSessions.map((s) => s.songTitle)).size;
    const multiSessionSongs = Array.from(
      scheduledSessions.reduce((acc, s) => {
        acc.set(s.songTitle, (acc.get(s.songTitle) || 0) + 1);
        return acc;
      }, new Map<string, number>()).entries()
    ).filter(([_, count]) => count > 1).length;
    const roomsUsed = new Set(scheduledSessions.map((s) => s.room)).size;

    return {
      totalSessions,
      uniqueSongsCount,
      multiSessionSongs,
      roomsUsed,
    };
  });

  // Filtered scheduled sessions
  let filteredSessions = $derived(
    scheduledSessions.filter((s) => {
      const matchSong = filterSong === 'all' || s.songTitle === filterSong;
      const matchRoom = filterRoom === 'all' || s.room === filterRoom;
      const matchDay = filterDay === 'all' || s.dayName === filterDay;
      return matchSong && matchRoom && matchDay;
    })
  );

  const uniqueSongs = Array.from(new Set(scheduledSessions.map((s) => s.songTitle)));
  const uniqueRooms = Array.from(new Set(scheduledSessions.map((s) => s.room)));
</script>

<div class="sprints-subpage">
  <!-- Active Sprint Banner -->
  <div class="sprint-header bento-card">
    <div class="header-info">
      <div class="sprint-tag-row">
        <div class="sprint-tag">
          <CalendarIcon size={14} class="text-orange" />
          <span>{$tStore('studio_shows.active_sprint')}: Sprint 3 (Stage QC & 15m Rehearsal Optimization)</span>
        </div>
        <div class="sse-live-indicator" title="Real-Time Server-Sent Events (SSE) Stream Active">
          <Radio size={12} class="animate-pulse text-green" />
          <span>Real-time SSE Sync</span>
        </div>
      </div>
      <h2>Practice Sprint Management & 15-Minute Free-Time Registration</h2>
      <p>{$tStore('studio.freetime_desc_drag')}</p>
    </div>

    <div class="sprint-header-actions">
      {#if canViewHistory(userRole)}
        <button
          type="button"
          class="bento-btn bento-btn-subtle"
          onclick={() => (isHistoryOpen = true)}
          title="Inspect Audit Logs & Compute History"
        >
          <HistoryIcon size={16} />
          <span>Audit History</span>
        </button>
      {/if}

      {#if canTriggerScheduler(userRole)}
        <button
          type="button"
          class="bento-btn bento-btn-primary"
          onclick={handleAutoSchedule}
          disabled={isScheduling}
        >
          <Wand2 size={16} class={isScheduling ? 'animate-spin' : ''} />
          <span>{isScheduling ? 'Optimizing Kafka Task...' : $tStore('studio.btn_auto_schedule')}</span>
        </button>
      {/if}
    </div>
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
      <!-- Quota Metrics Summary Bar -->
      <div class="quota-summary-bar">
        <div class="quota-pill">
          <Sparkles size={14} class="text-orange" />
          <span><strong>{quotaMetrics.totalSessions}</strong> Rehearsals Scheduled</span>
        </div>
        <div class="quota-pill">
          <Music size={14} class="text-blue" />
          <span><strong>{quotaMetrics.uniqueSongsCount}</strong> Active Songs</span>
        </div>
        <div class="quota-pill">
          <Zap size={14} class="text-purple" />
          <span><strong>{quotaMetrics.multiSessionSongs}</strong> Multi-Rehearsal Songs</span>
        </div>
        <div class="quota-pill">
          <MapPin size={14} class="text-green" />
          <span><strong>{quotaMetrics.roomsUsed}</strong> Rooms Utilized</span>
        </div>
      </div>

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
          <!-- View Switcher Toggle Buttons -->
          <div class="view-toggle-group">
            <button
              type="button"
              class="view-toggle-btn {scheduleView === 'grid' ? 'is-active' : ''}"
              onclick={() => (scheduleView = 'grid')}
              title="Bento Grid View"
            >
              <LayoutGrid size={14} />
              <span>Grid</span>
            </button>
            <button
              type="button"
              class="view-toggle-btn {scheduleView === 'timeline' ? 'is-active' : ''}"
              onclick={() => (scheduleView = 'timeline')}
              title="Compact Timeline Table View"
            >
              <Table size={14} />
              <span>Timeline</span>
            </button>
          </div>

          <!-- Day Filter -->
          <div class="filter-item">
            <CalendarIcon size={13} class="text-muted" />
            <span class="filter-label">Day</span>
            <select bind:value={filterDay} class="bento-select">
              <option value="all">All Days</option>
              {#each days as day}
                <option value={day}>{day}</option>
              {/each}
            </select>
          </div>

          <!-- Song Filter -->
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

          <!-- Room Filter -->
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

      {#if scheduleView === 'grid'}
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
                        <div class="song-title-group">
                          <h4 class="song-name">{session.songTitle}</h4>
                          <span class="session-index-tag">#{session.sessionIndex} of {session.totalTargetRehearsals}</span>
                        </div>
                        <span
                          class="status-chip {session.status === 'stage_ready'
                            ? 'chip-success'
                            : session.status === 'qc_approved'
                              ? 'chip-approved'
                              : session.status === 'ready_for_qc'
                                ? 'chip-qc'
                                : 'chip-practice'}"
                        >
                          {session.status === 'stage_ready'
                            ? 'Stage Ready'
                            : session.status === 'qc_approved'
                              ? 'QC Approved'
                              : session.status === 'ready_for_qc'
                                ? 'Ready QC'
                                : 'In Practice'}
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
      {:else}
        <!-- Compact Timeline Table View -->
        <div class="timeline-table-wrapper">
          <table class="timeline-table">
            <thead>
              <tr>
                <th>Day & Time</th>
                <th>Song & Session</th>
                <th>PM Leader</th>
                <th>Studio Room</th>
                <th>Lineup Performers</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              {#if filteredSessions.length === 0}
                <tr>
                  <td colspan="6" class="empty-timeline-td">No rehearsals match the selected filters.</td>
                </tr>
              {:else}
                {#each filteredSessions as session}
                  <tr>
                    <td class="td-daytime">
                      <div class="daytime-pill">
                        <strong>{session.dayName}</strong>
                        <span>{session.startTime} – {session.endTime} ({session.durationMinutes}m)</span>
                      </div>
                    </td>
                    <td class="td-song">
                      <div class="song-timeline-info">
                        <strong class="timeline-song-title">{session.songTitle}</strong>
                        <span class="session-index-tag">Session #{session.sessionIndex} of {session.totalTargetRehearsals}</span>
                      </div>
                    </td>
                    <td class="td-pm">{session.pmName}</td>
                    <td class="td-room">
                      <span class="room-pill">{session.room}</span>
                    </td>
                    <td class="td-lineup">
                      <span class="lineup-text">{session.performers.join(', ')}</span>
                    </td>
                    <td class="td-status">
                      <span
                        class="status-chip {session.status === 'stage_ready'
                          ? 'chip-success'
                          : session.status === 'qc_approved'
                            ? 'chip-approved'
                            : session.status === 'ready_for_qc'
                              ? 'chip-qc'
                              : 'chip-practice'}"
                      >
                        {session.status === 'stage_ready'
                          ? 'Stage Ready'
                          : session.status === 'qc_approved'
                            ? 'QC Approved'
                            : session.status === 'ready_for_qc'
                              ? 'Ready QC'
                              : 'In Practice'}
                      </span>
                    </td>
                  </tr>
                {/each}
              {/if}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  {/if}

  <!-- Audit History Drawer Modal (PM, DM, Moderator, Admin) -->
  {#if isHistoryOpen}
    <div class="modal-backdrop" onclick={() => (isHistoryOpen = false)} role="presentation">
      <div class="modal-card bento-card history-modal" onclick={(e) => e.stopPropagation()} role="presentation">
        <div class="modal-header">
          <div class="modal-title-row">
            <HistoryIcon size={18} class="text-orange" />
            <h3>Audit History & Run Logs</h3>
          </div>
          <button type="button" class="close-btn" onclick={() => (isHistoryOpen = false)} aria-label="Close">
            <X size={16} />
          </button>
        </div>

        <div class="history-tab-bar">
          <button
            type="button"
            class="tab-btn {activeHistoryTab === 'compute' ? 'is-active' : ''}"
            onclick={() => (activeHistoryTab = 'compute')}
          >
            <Activity size={14} />
            <span>Schedule Compute History ({computeHistory.length})</span>
          </button>
          <button
            type="button"
            class="tab-btn {activeHistoryTab === 'registration' ? 'is-active' : ''}"
            onclick={() => (activeHistoryTab = 'registration')}
          >
            <Clock size={14} />
            <span>User Free-Time Registration History ({registrationHistory.length})</span>
          </button>
        </div>

        <div class="history-tab-content">
          {#if activeHistoryTab === 'compute'}
            <div class="history-table-wrapper">
              <table class="history-table">
                <thead>
                  <tr>
                    <th>Run ID</th>
                    <th>Triggered By</th>
                    <th>Status</th>
                    <th>Duration</th>
                    <th>CSP Score</th>
                    <th>Conflicts</th>
                    <th>Timestamp</th>
                  </tr>
                </thead>
                <tbody>
                  {#each computeHistory as item}
                    <tr>
                      <td class="font-mono text-bold">{item.id}</td>
                      <td>{item.triggeredByName}</td>
                      <td>
                        <span class="status-pill status-{item.status}">{item.status.toUpperCase()}</span>
                      </td>
                      <td>{item.durationMs}ms</td>
                      <td class="text-green font-bold">{item.score}%</td>
                      <td>{item.conflictCount} conflicts</td>
                      <td class="text-muted">{item.createdAt}</td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {:else}
            <div class="history-table-wrapper">
              <table class="history-table">
                <thead>
                  <tr>
                    <th>User</th>
                    <th>Actor</th>
                    <th>Action</th>
                    <th>Day</th>
                    <th>Slot</th>
                    <th>Status</th>
                    <th>Timestamp</th>
                  </tr>
                </thead>
                <tbody>
                  {#each registrationHistory as reg}
                    <tr>
                      <td class="font-bold">{reg.userName}</td>
                      <td>{reg.actorName}</td>
                      <td>
                        <span class="action-pill action-{reg.action}">{reg.action}</span>
                      </td>
                      <td>{reg.dayOfWeek}</td>
                      <td class="font-mono">{reg.slotLabel}</td>
                      <td>
                        <span class={reg.isAvailable ? 'text-green' : 'text-red'}>
                          {reg.isAvailable ? 'Available' : 'Unavailable'}
                        </span>
                      </td>
                      <td class="text-muted">{reg.createdAt}</td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {/if}
        </div>
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

  .sprint-tag-row {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 6px;
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
  }

  .sse-live-indicator {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 3px 8px;
    background: #f0fdf4;
    border: 1px solid #bbf7d0;
    color: #16a34a;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 700;
  }

  .sprint-header-actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .bento-btn-subtle {
    background: #f1f5f9;
    color: #334155;
    border: 1px solid #cbd5e1;
  }

  .bento-btn-subtle:hover {
    background: #e2e8f0;
    color: #0f172a;
  }

  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(15, 23, 42, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 999;
    padding: 20px;
  }

  .history-modal {
    width: 100%;
    max-width: 820px;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    gap: 16px;
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .modal-title-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .modal-title-row h3 {
    margin: 0;
    font-size: 17px;
    font-weight: 800;
    color: #0f172a;
  }

  .close-btn {
    background: none;
    border: none;
    color: #64748b;
    cursor: pointer;
    padding: 4px;
    border-radius: 6px;
  }

  .close-btn:hover {
    background: #f1f5f9;
    color: #0f172a;
  }

  .history-tab-bar {
    display: flex;
    gap: 8px;
    border-bottom: 1px solid #e2e8f0;
    padding-bottom: 8px;
  }

  .tab-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: 8px;
    font-size: 12px;
    font-weight: 600;
    color: #64748b;
    background: none;
    border: none;
    cursor: pointer;
  }

  .tab-btn.is-active {
    background: #ff6b00;
    color: #ffffff;
  }

  .history-table-wrapper {
    overflow-x: auto;
    max-height: 450px;
  }

  .history-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 12px;
  }

  .history-table th {
    background: #f8fafc;
    padding: 8px 12px;
    text-align: left;
    border-bottom: 1px solid #e2e8f0;
    color: #475569;
    font-weight: 700;
  }

  .history-table td {
    padding: 8px 12px;
    border-bottom: 1px solid #f1f5f9;
  }

  .status-pill {
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 800;
  }

  .status-completed { background: #dcfce7; color: #15803d; }
  .status-failed { background: #fee2e2; color: #dc2626; }
  .status-queued { background: #fef9c3; color: #a16207; }

  .action-pill {
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 800;
  }

  .action-ADD { background: #dcfce7; color: #15803d; }
  .action-UPDATE { background: #e0f2fe; color: #0369a1; }
  .action-DELETE { background: #fee2e2; color: #dc2626; }

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

  /* Quota Metrics Summary Bar */
  .quota-summary-bar {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 12px;
    padding: 10px 14px;
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    border-radius: 12px;
  }

  .quota-pill {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: #475569;
    background: #ffffff;
    padding: 4px 10px;
    border-radius: 8px;
    border: 1px solid #cbd5e1;
  }

  .quota-pill strong {
    color: #0f172a;
    font-weight: 800;
  }

  /* View Toggle Switcher */
  .view-toggle-group {
    display: flex;
    background: #e2e8f0;
    padding: 2px;
    border-radius: 8px;
  }

  .view-toggle-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 10px;
    font-size: 11px;
    font-weight: 700;
    color: #64748b;
    border: none;
    background: transparent;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .view-toggle-btn.is-active {
    background: #ffffff;
    color: #ff6b00;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .session-index-tag {
    font-size: 9px;
    font-weight: 700;
    color: #64748b;
    background: #f1f5f9;
    padding: 1px 5px;
    border-radius: 4px;
    border: 1px solid #cbd5e1;
  }

  .song-title-group {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .chip-approved {
    background: rgba(16, 185, 129, 0.1);
    color: #059669;
  }

  .chip-success {
    background: rgba(34, 197, 94, 0.15);
    color: #16a34a;
  }

  /* Timeline Table */
  .timeline-table-wrapper {
    overflow-x: auto;
    border: 1px solid #e2e8f0;
    border-radius: 10px;
    background: #ffffff;
  }

  .timeline-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 12px;
  }

  .timeline-table th {
    background: #f8fafc;
    color: #475569;
    font-weight: 700;
    text-align: left;
    padding: 10px 12px;
    border-bottom: 1px solid #e2e8f0;
    white-space: nowrap;
  }

  .timeline-table td {
    padding: 10px 12px;
    border-bottom: 1px solid #f1f5f9;
    color: #334155;
    vertical-align: middle;
  }

  .timeline-table tr:hover {
    background: #f8fafc;
  }

  .td-daytime .daytime-pill {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .td-daytime strong {
    color: #0f172a;
    font-size: 12px;
  }

  .td-daytime span {
    font-size: 11px;
    color: #64748b;
  }

  .song-timeline-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .timeline-song-title {
    color: #0f172a;
    font-weight: 800;
  }

  .room-pill {
    display: inline-block;
    padding: 2px 8px;
    background: #f1f5f9;
    border-radius: 6px;
    font-weight: 700;
    color: #475569;
    font-size: 11px;
  }

  .lineup-text {
    font-size: 11px;
    color: #64748b;
  }

  .empty-timeline-td {
    text-align: center;
    padding: 24px;
    color: #94a3b8;
    font-style: italic;
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
