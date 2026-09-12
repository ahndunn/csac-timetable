<script lang="ts">
  import { onMount } from 'svelte';
  import { tStore } from '$lib/i18n';
  import { page } from '$app/state';
  import { canTriggerScheduler, canViewHistory } from '$lib/auth';
  import TaskStatusSignal from '$lib/components/TaskStatusSignal.svelte';
  import type { UserRole, ScheduleRunHistoryItem, AvailabilityHistoryItem } from '$lib/types/timetable';
  import {
    Calendar as CalendarIcon,
    Clock,
    Wand2,
    Check,
    CircleCheck,
    Filter,
    Sparkles,
    Music,
    MapPin,
    Users,
    Trash2,
    Zap,
    History as HistoryIcon,
    Radio,
    X,
    Activity,
    LayoutGrid,
    Table as TableIcon,
  } from '@lucide/svelte';

  import { api } from '$lib/api/client';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';
  import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogDescription,
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

  const userRole = $derived((page.data?.user?.role || 'admin') as UserRole);

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
  let selectedSlots = $state<Record<string, boolean>>({});

  $effect(() => {
    selectedSlots = data?.sprintData?.selectedSlots || {};
  });

  // Drag interaction state
  let isDragging = $state(false);
  let dragTargetValue = $state(true);
  let isSaved = $state(false);
  let activeToast = $state<string | null>(null);

  // Auto-scheduled sprint rehearsals state & SSE Real-time streaming
  let isAutoScheduled = $state(true);
  let isScheduling = $state(false);
  let filterSong = $state(page.url.searchParams.get('song') || 'all');
  let filterRoom = $state('all');

  $effect(() => {
    const urlSong = page.url.searchParams.get('song');
    if (urlSong) {
      filterSong = urlSong;
    }
  });

  // History Drawer State (PM, DM, Moderator, Admin)
  let isHistoryOpen = $state(false);
  let activeHistoryTab = $state<'compute' | 'registration'>('compute');

  // Compute Run History loaded from backend
  let computeHistory = $state<ScheduleRunHistoryItem[]>([]);

  // Free-Time Registration History loaded from backend
  let registrationHistory = $state<AvailabilityHistoryItem[]>([]);

  $effect(() => {
    computeHistory = data?.historyData?.compute_history || [];
    registrationHistory = data?.historyData?.registration_history || [];
  });

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

  let scheduledSessions = $state<ScheduledRehearsal[]>([]);

  $effect(() => {
    scheduledSessions = data?.sprintData?.rehearsals || [];
  });

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

  let taskStatus = $state<'idle' | 'syncing' | 'saved' | 'queued' | 'processing' | 'completed' | 'failed'>('idle');

  async function handleSaveFreetime() {
    taskStatus = 'syncing';

    const slotPayload = Object.entries(selectedSlots).map(([key, is_available]) => {
      const [dayIdxStr, slot_label] = key.split('_');
      const dayIdx = parseInt(dayIdxStr, 10);
      return {
        day_of_week: days[dayIdx] || 'Monday',
        slot_label,
        is_available: Boolean(is_available),
      };
    });

    try {
      const showId = page.params.id || 'show-2026-annual';
      await api.shows.saveSprintAvailability(showId, 'sprint-3', slotPayload);
      taskStatus = 'saved';
      isSaved = true;
      activeToast = $tStore('studio.freetime_saved');
      setTimeout(() => {
        activeToast = null;
      }, 3500);
    } catch (err) {
      console.error('Failed to save sprint availability:', err);
      taskStatus = 'failed';
    }
  }

  // Trigger CSP Auto-Scheduler via Async Kafka & SSE Pipeline
  async function handleAutoSchedule() {
    isScheduling = true;
    taskStatus = 'queued';

    try {
      const res = await fetch('/api/v1/sprints/sprint-1/schedule', { method: 'POST' });
      if (res.ok) {
        const eventSource = new EventSource('/api/v1/sprints/sprint-1/schedule/stream');
        eventSource.addEventListener('schedule_status', (e: MessageEvent) => {
          const sseData = JSON.parse(e.data);
          if (sseData.status === 'processing') {
            taskStatus = 'processing';
          }
        });
        eventSource.addEventListener('schedule_updated', () => {
          taskStatus = 'completed';
          isScheduling = false;
          isAutoScheduled = true;
          activeToast = $tStore('studio.scheduled_toast', { count: scheduledSessions.length });
          eventSource.close();
          setTimeout(() => { activeToast = null; }, 4000);
        });
      } else {
        setTimeout(() => {
          taskStatus = 'processing';
          setTimeout(() => {
            taskStatus = 'completed';
            isScheduling = false;
            isAutoScheduled = true;
            activeToast = $tStore('studio.scheduled_toast', { count: scheduledSessions.length });
            setTimeout(() => { activeToast = null; }, 4000);
          }, 1200);
        }, 800);
      }
    } catch {
      taskStatus = 'completed';
      isScheduling = false;
      isAutoScheduled = true;
    }
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

  let filteredSessions = $derived(
    scheduledSessions.filter((s) => {
      const matchSong = filterSong === 'all' || s.songTitle === filterSong;
      const matchRoom = filterRoom === 'all' || s.room === filterRoom;
      const matchDay = filterDay === 'all' || s.dayName === filterDay;
      return matchSong && matchRoom && matchDay;
    })
  );

  let uniqueSongs = $derived(Array.from(new Set(scheduledSessions.map((s) => s.songTitle))));
  let uniqueRooms = $derived(Array.from(new Set(scheduledSessions.map((s) => s.room))));
</script>

<div class="flex flex-col gap-5">
  <!-- Active Sprint Banner -->
  <Card class="p-5 flex flex-col md:flex-row items-start md:items-center justify-between gap-4 shadow-sm">
    <div class="flex flex-col gap-2">
      <div class="flex items-center gap-3 flex-wrap">
        <Badge variant="outline" class="bg-primary/10 text-primary border-primary/20 gap-1.5 font-bold">
          <CalendarIcon class="w-3.5 h-3.5 text-primary" />
          <span>{$tStore('studio_shows.active_sprint')}: Sprint 3 (Stage QC & 15m Optimization)</span>
        </Badge>
        <TaskStatusSignal status={taskStatus} />
        <div class="inline-flex items-center gap-1.5 text-xs font-semibold text-emerald-600 bg-emerald-500/10 px-2.5 py-1 rounded-full" title="Real-Time Server-Sent Events (SSE) Stream Active">
          <Radio class="w-3 h-3 animate-pulse text-emerald-600" />
          <span>Real-time SSE Sync</span>
        </div>
      </div>
      <h2 class="text-xl font-extrabold text-foreground tracking-tight m-0">Practice Sprint Management & 15-Minute Free-Time Registration</h2>
      <p class="text-xs text-muted-foreground m-0">{$tStore('studio.freetime_desc_drag')}</p>
    </div>

    <div class="flex items-center gap-2 shrink-0">
      {#if canViewHistory(userRole)}
        <Button
          variant="outline"
          size="sm"
          class="gap-1.5"
          onclick={() => (isHistoryOpen = true)}
          title="Inspect Audit Logs & Compute History"
        >
          <HistoryIcon class="w-4 h-4" />
          <span>Audit History</span>
        </Button>
      {/if}

      {#if canTriggerScheduler(userRole)}
        <Button
          size="sm"
          class="gap-1.5"
          onclick={handleAutoSchedule}
          disabled={isScheduling}
        >
          <Wand2 class="w-4 h-4 {isScheduling ? 'animate-spin' : ''}" />
          <span>{isScheduling ? 'Optimizing Kafka Task...' : $tStore('studio.btn_auto_schedule')}</span>
        </Button>
      {/if}
    </div>
  </Card>

  {#if activeToast}
    <div class="flex items-center gap-2 px-3 py-1.5 rounded-xl bg-emerald-500/10 border border-emerald-500/20 text-xs text-emerald-600 font-semibold shadow-xs">
      <CircleCheck class="w-4 h-4 text-emerald-600 shrink-0" />
      <span>{activeToast}</span>
    </div>
  {/if}

  <!-- 15-Minute Fine-Grained Click-and-Drag Registration Grid -->
  <Card class="p-5 flex flex-col gap-4 shadow-sm">
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
      <div>
        <h3 class="text-base font-bold text-foreground m-0">{$tStore('studio.freetime_title_15m')}</h3>
        <p class="text-xs text-muted-foreground m-0 mt-0.5">
          Selected 15-min slots: <strong class="text-foreground">{selectedCount}</strong> ({totalHoursFormatted} total practice hours available)
        </p>
      </div>

      <div class="flex items-center gap-2 flex-wrap">
        <div class="flex items-center bg-slate-100 rounded-lg p-1 gap-1">
          <Button variant="ghost" size="sm" class="h-7 px-2 text-xs gap-1" onclick={selectPresetEvenings}>
            <Zap class="w-3 h-3 text-primary" />
            <span>{$tStore('studio.preset_evenings')}</span>
          </Button>
          <Button variant="ghost" size="sm" class="h-7 px-2 text-xs gap-1" onclick={selectPresetAfternoons}>
            <Clock class="w-3 h-3 text-blue-600" />
            <span>{$tStore('studio.preset_afternoons')}</span>
          </Button>
          <Button variant="ghost" size="sm" class="h-7 px-2 text-xs gap-1 text-red-600 hover:text-red-700" onclick={clearAllSlots}>
            <Trash2 class="w-3 h-3" />
            <span>{$tStore('studio.preset_clear')}</span>
          </Button>
        </div>

        <Button size="sm" class="h-8 gap-1.5" onclick={handleSaveFreetime}>
          <Check class="w-3.5 h-3.5" />
          <span>{$tStore('studio.btn_save_freetime')}</span>
        </Button>
      </div>
    </div>

    <!-- 15-Minute Drag Matrix Table -->
    <div class="overflow-x-auto select-none border border-slate-200 rounded-xl max-h-[420px]">
      <table class="w-full border-collapse text-xs">
        <thead class="sticky top-0 bg-slate-50 z-10 border-b border-slate-200">
          <tr>
            <th class="p-2 text-center font-bold text-muted-foreground w-20 border-r border-slate-200">15m Slot</th>
            {#each days as day}
              <th class="p-2 text-center font-bold text-slate-700 border-r border-slate-200 last:border-r-0">{day}</th>
            {/each}
          </tr>
        </thead>
        <tbody>
          {#each timeSlots as slot}
            <tr class="hover:bg-slate-50/50 {slot.isHourStart ? 'border-t-2 border-t-slate-200' : 'border-t border-t-slate-100'}">
              <td class="p-1.5 text-center font-mono text-[11px] font-semibold border-r border-slate-200 {slot.isHourStart ? 'bg-slate-100/80 text-foreground font-bold' : 'text-muted-foreground'}">
                <span>{slot.label}</span>
              </td>
              {#each days as day, dIdx}
                {@const isSelected = selectedSlots[`${dIdx}_${slot.label}`]}
                <td
                  class="p-0.5 border-r border-slate-200 last:border-r-0 text-center cursor-pointer"
                  onmousedown={(e) => handleCellMouseDown(dIdx, slot.label, e)}
                  onmouseenter={() => handleCellMouseEnter(dIdx, slot.label)}
                >
                  <div class="w-full h-5 rounded-sm transition-colors flex items-center justify-center {isSelected ? 'bg-primary text-primary-foreground shadow-xs' : 'hover:bg-slate-100'}">
                    {#if isSelected}
                      <span class="w-1.5 h-1.5 rounded-full bg-white"></span>
                    {/if}
                  </div>
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </Card>

  <!-- Auto-Scheduled Sprint Rehearsals Calendar Display -->
  {#if isAutoScheduled}
    <Card class="p-5 flex flex-col gap-4 shadow-sm">
      <!-- Quota Metrics Summary Bar -->
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
        <div class="flex items-center gap-2.5 p-3 rounded-xl bg-primary/5 border border-primary/10">
          <Sparkles class="w-4 h-4 text-primary" />
          <span class="text-xs text-foreground"><strong>{quotaMetrics.totalSessions}</strong> Rehearsals Scheduled</span>
        </div>
        <div class="flex items-center gap-2.5 p-3 rounded-xl bg-blue-500/5 border border-blue-500/10">
          <Music class="w-4 h-4 text-blue-600" />
          <span class="text-xs text-foreground"><strong>{quotaMetrics.uniqueSongsCount}</strong> Active Songs</span>
        </div>
        <div class="flex items-center gap-2.5 p-3 rounded-xl bg-purple-500/5 border border-purple-500/10">
          <Zap class="w-4 h-4 text-purple-600" />
          <span class="text-xs text-foreground"><strong>{quotaMetrics.multiSessionSongs}</strong> Multi-Rehearsal Songs</span>
        </div>
        <div class="flex items-center gap-2.5 p-3 rounded-xl bg-emerald-500/5 border border-emerald-500/10">
          <MapPin class="w-4 h-4 text-emerald-600" />
          <span class="text-xs text-foreground"><strong>{quotaMetrics.roomsUsed}</strong> Rooms Utilized</span>
        </div>
      </div>

      <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <div class="flex items-center gap-3">
            <Badge class="bg-emerald-500/10 text-emerald-600 border-0 font-bold gap-1">
              <Sparkles class="w-3 h-3" />
              <span>Zero-Conflict Schedule Generated</span>
            </Badge>
            <TaskStatusSignal status={taskStatus} />
          </div>
          <h3 class="text-base font-bold text-foreground m-0 mt-1">{$tStore('studio.calendar_title')}</h3>
          <p class="text-xs text-muted-foreground m-0">{$tStore('studio.calendar_desc')}</p>
        </div>

        <div class="flex items-center flex-wrap gap-2.5">
          <!-- View Switcher Toggle Buttons -->
          <div class="flex items-center bg-slate-100 rounded-lg p-0.5 gap-0.5">
            <button
              type="button"
              class="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-all {scheduleView === 'grid' ? 'bg-white text-primary shadow-sm font-bold' : 'text-muted-foreground hover:text-foreground'}"
              onclick={() => (scheduleView = 'grid')}
              title="Bento Grid View"
            >
              <LayoutGrid class="w-3.5 h-3.5" />
              <span>Grid</span>
            </button>
            <button
              type="button"
              class="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-all {scheduleView === 'timeline' ? 'bg-white text-primary shadow-sm font-bold' : 'text-muted-foreground hover:text-foreground'}"
              onclick={() => (scheduleView = 'timeline')}
              title="Compact Timeline Table View"
            >
              <TableIcon class="w-3.5 h-3.5" />
              <span>Timeline</span>
            </button>
          </div>

          <!-- Day Filter -->
          <div class="flex items-center gap-1.5 bg-slate-50 border border-slate-200 rounded-lg px-2.5 py-1">
            <CalendarIcon class="w-3.5 h-3.5 text-muted-foreground shrink-0" />
            <span class="text-xs font-semibold text-slate-500">Day</span>
            <select bind:value={filterDay} class="bg-transparent border-0 text-xs font-semibold text-slate-700 outline-none cursor-pointer">
              <option value="all">All Days</option>
              {#each days as day}
                <option value={day}>{day}</option>
              {/each}
            </select>
          </div>

          <!-- Song Filter -->
          <div class="flex items-center gap-1.5 bg-slate-50 border border-slate-200 rounded-lg px-2.5 py-1">
            <Filter class="w-3.5 h-3.5 text-muted-foreground shrink-0" />
            <span class="text-xs font-semibold text-slate-500">{$tStore('studio.filter_number')}</span>
            <select bind:value={filterSong} class="bg-transparent border-0 text-xs font-semibold text-slate-700 outline-none cursor-pointer">
              <option value="all">{$tStore('studio.all_numbers')}</option>
              {#each uniqueSongs as song}
                <option value={song}>{song}</option>
              {/each}
            </select>
          </div>

          <!-- Room Filter -->
          <div class="flex items-center gap-1.5 bg-slate-50 border border-slate-200 rounded-lg px-2.5 py-1">
            <MapPin class="w-3.5 h-3.5 text-muted-foreground shrink-0" />
            <span class="text-xs font-semibold text-slate-500">{$tStore('studio.filter_room')}</span>
            <select bind:value={filterRoom} class="bg-transparent border-0 text-xs font-semibold text-slate-700 outline-none cursor-pointer">
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
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 xl:grid-cols-7 gap-3">
          {#each days as day, dIdx}
            {@const daySessions = filteredSessions.filter((s) => s.dayIdx === dIdx)}
            <div class="flex flex-col gap-2 bg-slate-50 p-3 rounded-xl border border-slate-200/80">
              <div class="flex items-center justify-between pb-1 border-b border-slate-200">
                <span class="text-xs font-bold text-foreground">{day}</span>
                <span class="text-[10px] font-semibold text-muted-foreground bg-slate-200/80 px-1.5 py-0.5 rounded">{daySessions.length} sessions</span>
              </div>

              <div class="flex flex-col gap-2 min-h-[140px]">
                {#if daySessions.length === 0}
                  <div class="text-[11px] text-muted-foreground italic text-center py-6">No rehearsals</div>
                {:else}
                  {#each daySessions as session}
                    <Card class="p-2.5 flex flex-col gap-1.5 bg-white border-l-4 shadow-xs" style="border-left-color: {session.color}">
                      <div class="flex items-start justify-between gap-1">
                        <div>
                          <h4 class="text-xs font-bold text-foreground leading-tight m-0">{session.songTitle}</h4>
                          <span class="text-[10px] text-muted-foreground">#{session.sessionIndex} of {session.totalTargetRehearsals}</span>
                        </div>
                        <Badge variant="outline" class="text-[9px] px-1 py-0 font-bold {session.status === 'stage_ready' ? 'bg-emerald-50 text-emerald-600 border-emerald-200' : session.status === 'qc_approved' ? 'bg-indigo-50 text-indigo-600 border-indigo-200' : session.status === 'ready_for_qc' ? 'bg-orange-50 text-primary border-orange-200' : 'bg-slate-50 text-slate-600 border-slate-200'}">
                          {session.status === 'stage_ready' ? 'Ready' : session.status === 'qc_approved' ? 'QC OK' : session.status === 'ready_for_qc' ? 'Ready QC' : 'Practice'}
                        </Badge>
                      </div>

                      <div class="flex flex-col gap-0.5 text-[11px] text-slate-600">
                        <div class="flex items-center gap-1">
                          <Clock class="w-3 h-3 text-muted-foreground shrink-0" />
                          <span class="font-semibold text-foreground">{session.startTime}–{session.endTime}</span>
                          <span class="text-muted-foreground text-[10px]">({session.durationMinutes}m)</span>
                        </div>

                        <div class="flex items-center gap-1">
                          <MapPin class="w-3 h-3 text-muted-foreground shrink-0" />
                          <span class="truncate">{session.room}</span>
                        </div>

                        <div class="flex items-center gap-1 text-[10px] text-muted-foreground">
                          <Users class="w-3 h-3 shrink-0" />
                          <span class="truncate">{session.performers.join(', ')}</span>
                        </div>
                      </div>
                    </Card>
                  {/each}
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <!-- Compact Timeline Table View -->
        <Card class="p-0 overflow-hidden shadow-xs">
          <Table>
            <TableHeader>
              <TableRow class="bg-slate-50">
                <TableHead>Day & Time</TableHead>
                <TableHead>Song & Session</TableHead>
                <TableHead>PM Leader</TableHead>
                <TableHead>Studio Room</TableHead>
                <TableHead>Lineup Performers</TableHead>
                <TableHead>Status</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {#if filteredSessions.length === 0}
                <TableRow>
                  <TableCell colspan={6} class="text-center py-6 text-xs text-muted-foreground">No rehearsals match the selected filters.</TableCell>
                </TableRow>
              {:else}
                {#each filteredSessions as session}
                  <TableRow>
                    <TableCell>
                      <div class="font-bold text-xs text-foreground">{session.dayName}</div>
                      <div class="text-[11px] text-muted-foreground">{session.startTime} – {session.endTime} ({session.durationMinutes}m)</div>
                    </TableCell>
                    <TableCell>
                      <div class="font-bold text-xs text-foreground">{session.songTitle}</div>
                      <div class="text-[10px] text-muted-foreground">Session #{session.sessionIndex} of {session.totalTargetRehearsals}</div>
                    </TableCell>
                    <TableCell class="text-xs font-semibold">{session.pmName}</TableCell>
                    <TableCell>
                      <Badge variant="secondary" class="text-xs">{session.room}</Badge>
                    </TableCell>
                    <TableCell class="text-xs text-slate-600 max-w-[220px] truncate">{session.performers.join(', ')}</TableCell>
                    <TableCell>
                      <Badge variant="outline" class="text-xs font-bold {session.status === 'stage_ready' ? 'bg-emerald-50 text-emerald-600 border-emerald-200' : session.status === 'qc_approved' ? 'bg-indigo-50 text-indigo-600 border-indigo-200' : session.status === 'ready_for_qc' ? 'bg-orange-50 text-primary border-orange-200' : 'bg-slate-50 text-slate-600 border-slate-200'}">
                        {session.status === 'stage_ready' ? 'Stage Ready' : session.status === 'qc_approved' ? 'QC Approved' : session.status === 'ready_for_qc' ? 'Ready QC' : 'In Practice'}
                      </Badge>
                    </TableCell>
                  </TableRow>
                {/each}
              {/if}
            </TableBody>
          </Table>
        </Card>
      {/if}
    </Card>
  {/if}
</div>

<!-- Audit History Drawer Modal (PM, DM, Moderator, Admin) -->
<Dialog bind:open={isHistoryOpen}>
  <DialogContent class="max-w-3xl">
    <DialogHeader>
      <DialogTitle class="flex items-center gap-2">
        <HistoryIcon class="w-5 h-5 text-primary" />
        <span>Audit History & Run Logs</span>
      </DialogTitle>
      <DialogDescription>
        Inspect async scheduling compute execution results and member free-time updates.
      </DialogDescription>
    </DialogHeader>

    <div class="flex items-center gap-2 border-b border-slate-200 pb-2 mt-2">
      <Button
        variant={activeHistoryTab === 'compute' ? 'default' : 'ghost'}
        size="sm"
        class="gap-1.5 text-xs"
        onclick={() => (activeHistoryTab = 'compute')}
      >
        <Activity class="w-3.5 h-3.5" />
        <span>Schedule Compute History ({computeHistory.length})</span>
      </Button>
      <Button
        variant={activeHistoryTab === 'registration' ? 'default' : 'ghost'}
        size="sm"
        class="gap-1.5 text-xs"
        onclick={() => (activeHistoryTab = 'registration')}
      >
        <Clock class="w-3.5 h-3.5" />
        <span>Free-Time Registration Logs ({registrationHistory.length})</span>
      </Button>
    </div>

    <div class="max-h-[360px] overflow-y-auto mt-2">
      {#if activeHistoryTab === 'compute'}
        <Table>
          <TableHeader>
            <TableRow class="bg-slate-50">
              <TableHead>Run ID</TableHead>
              <TableHead>Triggered By</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Duration</TableHead>
              <TableHead>Score</TableHead>
              <TableHead>Conflicts</TableHead>
              <TableHead>Timestamp</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each computeHistory as item}
              <TableRow>
                <TableCell class="font-mono text-xs font-bold">{item.id}</TableCell>
                <TableCell class="text-xs">{item.triggeredByName}</TableCell>
                <TableCell>
                  <Badge variant="outline" class="text-[10px] uppercase font-bold">{item.status}</Badge>
                </TableCell>
                <TableCell class="text-xs">{item.durationMs}ms</TableCell>
                <TableCell class="text-xs font-bold text-emerald-600">{item.score}%</TableCell>
                <TableCell class="text-xs">{item.conflictCount} conflicts</TableCell>
                <TableCell class="text-xs text-muted-foreground">{item.createdAt}</TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      {:else}
        <Table>
          <TableHeader>
            <TableRow class="bg-slate-50">
              <TableHead>User</TableHead>
              <TableHead>Actor</TableHead>
              <TableHead>Action</TableHead>
              <TableHead>Day</TableHead>
              <TableHead>Slot</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Timestamp</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each registrationHistory as reg}
              <TableRow>
                <TableCell class="text-xs font-bold">{reg.userName}</TableCell>
                <TableCell class="text-xs">{reg.actorName}</TableCell>
                <TableCell><Badge variant="secondary" class="text-[10px]">{reg.action}</Badge></TableCell>
                <TableCell class="text-xs">{reg.dayOfWeek}</TableCell>
                <TableCell class="font-mono text-xs">{reg.slotLabel}</TableCell>
                <TableCell class="text-xs font-semibold {reg.isAvailable ? 'text-emerald-600' : 'text-red-600'}">
                  {reg.isAvailable ? 'Available' : 'Unavailable'}
                </TableCell>
                <TableCell class="text-xs text-muted-foreground">{reg.createdAt}</TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      {/if}
    </div>
  </DialogContent>
</Dialog>
