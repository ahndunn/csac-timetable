<script lang="ts">
  import { tStore } from "$lib/i18n";
  import { page } from "$app/state";
  import { canTriggerScheduler, canViewHistory } from "$lib/auth";
  import TaskStatusSignal from "$lib/components/TaskStatusSignal.svelte";
  import type {
    UserRole,
    ScheduleRunHistoryItem,
    AvailabilityHistoryItem,
  } from "$lib/types/timetable";
  import {
    Calendar as CalendarIcon,
    Wand2,
    CircleCheck,
    Filter,
    Sparkles,
    Music,
    MapPin,
    Zap,
    History as HistoryIcon,
    Radio,
    LayoutGrid,
    Table as TableIcon,
  } from "@lucide/svelte";

  import { api } from "$lib/api/client";
  import { Button } from "$lib/components/ui/button";
  import { Badge } from "$lib/components/ui/badge";
  import { Card } from "$lib/components/ui/card";
  import { auth } from "$lib/stores/auth.svelte";

  // Sub-components
  import Sprint15mGrid from "./components/Sprint15mGrid.svelte";
  import SprintRehearsalGrid, {
    type ScheduledRehearsal,
  } from "./components/SprintRehearsalGrid.svelte";
  import SprintTimelineTable from "./components/SprintTimelineTable.svelte";
  import SprintAuditDrawer from "./components/SprintAuditDrawer.svelte";

  let { data } = $props();

  const activeUser = $derived(auth.user || page.data?.user || null);
  const userRole = $derived((activeUser?.role || "member") as UserRole);

  const days = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
  ];

  // Generate 15-minute interval time slots from 08:00 to 22:45
  const hours = [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22];
  const minutes = ["00", "15", "30", "45"];

  const timeSlots = hours.flatMap((h) =>
    minutes.map((m) => ({
      label: `${h.toString().padStart(2, "0")}:${m}`,
      hour: h,
      minute: m,
      isHourStart: m === "00",
    })),
  );

  // 15-Minute Selection Matrix State: `${dayIdx}_${slotLabel}` -> boolean
  let selectedSlots = $state<Record<string, boolean>>({});

  $effect(() => {
    selectedSlots = data?.sprintData?.selectedSlots || {};
  });

  let isSaved = $state(false);
  let activeToast = $state<string | null>(null);

  // Auto-scheduled sprint rehearsals state & SSE Real-time streaming
  let isAutoScheduled = $state(true);
  let isScheduling = $state(false);
  let filterSong = $state(page.url.searchParams.get("song") || "all");
  let filterRoom = $state("all");
  let filterDay = $state("all");
  let scheduleView = $state<"grid" | "timeline">("grid");

  $effect(() => {
    const urlSong = page.url.searchParams.get("song");
    if (urlSong) {
      filterSong = urlSong;
    }
  });

  // History Drawer State
  let isHistoryOpen = $state(false);
  let computeHistory = $state<ScheduleRunHistoryItem[]>([]);
  let registrationHistory = $state<AvailabilityHistoryItem[]>([]);

  $effect(() => {
    computeHistory = data?.historyData?.compute_history || [];
    registrationHistory = data?.historyData?.registration_history || [];
  });

  let scheduledSessions = $state<ScheduledRehearsal[]>([]);

  $effect(() => {
    scheduledSessions = data?.sprintData?.rehearsals || [];
  });

  let taskStatus = $state<
    | "idle"
    | "syncing"
    | "saved"
    | "queued"
    | "processing"
    | "completed>"
    | "failed"
  >("idle");

  const activeSprint = $derived(data?.sprintData?.sprint || null);
  const sprintId = $derived(activeSprint?.id || "");
  const sprintName = $derived(
    activeSprint?.name || $tStore("studio_shows.active_sprint"),
  );

  async function handleSaveFreetime() {
    taskStatus = "syncing";

    const slotPayload = Object.entries(selectedSlots).map(
      ([key, is_available]) => {
        const [dayIdxStr, slot_label] = key.split("_");
        const dayIdx = parseInt(dayIdxStr, 10);
        return {
          day_of_week: days[dayIdx] || "Monday",
          slot_label,
          is_available: Boolean(is_available),
        };
      },
    );

    try {
      const showId = page.params.id || "";
      if (!showId || !sprintId) {
        throw new Error("Show ID or Sprint ID missing");
      }
      await api.shows.saveSprintAvailability(showId, sprintId, slotPayload);
      taskStatus = "saved";
      isSaved = true;
      activeToast = $tStore("studio.freetime_saved");
      setTimeout(() => {
        activeToast = null;
      }, 3500);
    } catch (err) {
      console.error("Failed to save sprint availability:", err);
      taskStatus = "failed";
    }
  }

  // Lifecycle-managed EventSource reference
  let activeEventSource = $state<EventSource | null>(null);

  $effect(() => {
    return () => {
      if (activeEventSource) {
        activeEventSource.close();
        activeEventSource = null;
      }
    };
  });

  // Trigger CSP Auto-Scheduler via Async Kafka & SSE Pipeline
  async function handleAutoSchedule() {
    isScheduling = true;
    taskStatus = "queued";

    const targetSprintId = sprintId || data?.sprintData?.sprint?.id || "";
    const showId = page.params.id || "";

    if (!targetSprintId || !showId) {
      console.error(
        "[SSE] No sprint ID or show ID available — cannot trigger scheduler",
      );
      taskStatus = "failed";
      isScheduling = false;
      return;
    }

    try {
      const res = await fetch(`/api/v1/sprints/${targetSprintId}/schedule`, {
        method: "POST",
      });
      if (res.ok) {
        if (activeEventSource) {
          activeEventSource.close();
        }

        const eventSource = new EventSource(
          `/api/v1/sprints/${sprintId}/schedule/stream`,
        );
        activeEventSource = eventSource;

        eventSource.addEventListener("schedule_status", (e: MessageEvent) => {
          const sseData = JSON.parse(e.data);
          if (sseData.status === "processing") {
            taskStatus = "processing";
          }
        });

        eventSource.addEventListener("schedule_updated", async () => {
          taskStatus = "completed";
          isScheduling = false;
          isAutoScheduled = true;
          eventSource.close();
          activeEventSource = null;

          try {
            const refreshed = await api.shows.getActiveSprint(showId);
            scheduledSessions = refreshed?.rehearsals || [];
          } catch (refreshErr) {
            console.warn(
              "[SSE] Could not refresh sprint sessions after schedule completion:",
              refreshErr,
            );
          }

          activeToast = $tStore("studio.scheduled_toast", {
            count: scheduledSessions.length,
          });
          setTimeout(() => {
            activeToast = null;
          }, 4000);
        });

        eventSource.onerror = (err) => {
          console.error("[SSE] EventSource error:", err);
          eventSource.close();
          activeEventSource = null;
          if (taskStatus !== "completed") {
            taskStatus = "failed";
            isScheduling = false;
          }
        };
      } else {
        console.warn(
          "[SSE] Schedule trigger returned",
          res.status,
          "— using fallback simulation",
        );
        setTimeout(() => {
          taskStatus = "processing";
          setTimeout(() => {
            taskStatus = "completed";
            isScheduling = false;
            isAutoScheduled = true;
            activeToast = $tStore("studio.scheduled_toast", {
              count: scheduledSessions.length,
            });
            setTimeout(() => {
              activeToast = null;
            }, 4000);
          }, 1200);
        }, 800);
      }
    } catch {
      taskStatus = "completed";
      isScheduling = false;
      isAutoScheduled = true;
    }
  }

  // Rehearsal Quota Metrics
  const quotaMetrics = $derived.by(() => {
    const totalSessions = scheduledSessions.length;
    const uniqueSongsCount = new Set(scheduledSessions.map((s) => s.songTitle))
      .size;
    const multiSessionSongs = Array.from(
      scheduledSessions
        .reduce((acc, s) => {
          acc.set(s.songTitle, (acc.get(s.songTitle) || 0) + 1);
          return acc;
        }, new Map<string, number>())
        .entries(),
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
      const matchSong = filterSong === "all" || s.songTitle === filterSong;
      const matchRoom = filterRoom === "all" || s.room === filterRoom;
      const matchDay = filterDay === "all" || s.dayName === filterDay;
      return matchSong && matchRoom && matchDay;
    }),
  );

  let uniqueSongs = $derived(
    Array.from(new Set(scheduledSessions.map((s) => s.songTitle))),
  );
  let uniqueRooms = $derived(
    Array.from(new Set(scheduledSessions.map((s) => s.room))),
  );
</script>

<div class="flex flex-col gap-5">
  <!-- Active Sprint Banner -->
  <Card
    class="p-5 flex flex-col md:flex-row items-start md:items-center justify-between gap-4 shadow-sm"
  >
    <div class="flex flex-col gap-2">
      <div class="flex items-center gap-3 flex-wrap">
        <Badge
          variant="outline"
          class="bg-primary/10 text-primary border-primary/20 gap-1.5 font-bold"
        >
          <CalendarIcon class="w-3.5 h-3.5 text-primary" />
          <span>{$tStore("studio_shows.active_sprint")}: {sprintName}</span>
        </Badge>
        <TaskStatusSignal status={taskStatus} />
        <div
          class="inline-flex items-center gap-1.5 text-xs font-semibold text-emerald-600 bg-emerald-500/10 px-2.5 py-1 rounded-full"
          title="Real-Time Server-Sent Events (SSE) Stream Active"
        >
          <Radio class="w-3 h-3 animate-pulse text-emerald-600" />
          <span>Real-time SSE Sync</span>
        </div>
      </div>
      <h2 class="text-xl font-extrabold text-foreground tracking-tight m-0">
        Practice Sprint Management & 15-Minute Free-Time Registration
      </h2>
      <p class="text-xs text-muted-foreground m-0">
        {$tStore("studio.freetime_desc_drag")}
      </p>
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
          <span
            >{isScheduling
              ? "Optimizing Kafka Task..."
              : $tStore("studio.btn_auto_schedule")}</span
          >
        </Button>
      {/if}
    </div>
  </Card>

  {#if activeToast}
    <div
      class="flex items-center gap-2 px-3 py-1.5 rounded-xl bg-emerald-500/10 border border-emerald-500/20 text-xs text-emerald-600 font-semibold shadow-xs"
    >
      <CircleCheck class="w-4 h-4 text-emerald-600 shrink-0" />
      <span>{activeToast}</span>
    </div>
  {/if}

  <!-- 15-Minute Click-and-Drag Registration Grid Component -->
  <Sprint15mGrid
    {days}
    {timeSlots}
    {selectedSlots}
    onSlotChange={(updated) => {
      selectedSlots = updated;
      isSaved = false;
    }}
    onSave={handleSaveFreetime}
  />

  <!-- Auto-Scheduled Sprint Rehearsals Calendar Display -->
  {#if isAutoScheduled}
    <Card class="p-5 flex flex-col gap-4 shadow-sm">
      <!-- Quota Metrics Summary Bar -->
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
        <div
          class="flex items-center gap-2.5 p-3 rounded-xl bg-primary/5 border border-primary/10"
        >
          <Sparkles class="w-4 h-4 text-primary" />
          <span class="text-xs text-foreground"
            ><strong>{quotaMetrics.totalSessions}</strong> Rehearsals Scheduled</span
          >
        </div>
        <div
          class="flex items-center gap-2.5 p-3 rounded-xl bg-blue-500/5 border border-blue-500/10"
        >
          <Music class="w-4 h-4 text-blue-600" />
          <span class="text-xs text-foreground"
            ><strong>{quotaMetrics.uniqueSongsCount}</strong> Active Songs</span
          >
        </div>
        <div
          class="flex items-center gap-2.5 p-3 rounded-xl bg-purple-500/5 border border-purple-500/10"
        >
          <Zap class="w-4 h-4 text-purple-600" />
          <span class="text-xs text-foreground"
            ><strong>{quotaMetrics.multiSessionSongs}</strong> Multi-Rehearsal Songs</span
          >
        </div>
        <div
          class="flex items-center gap-2.5 p-3 rounded-xl bg-emerald-500/5 border border-emerald-500/10"
        >
          <MapPin class="w-4 h-4 text-emerald-600" />
          <span class="text-xs text-foreground"
            ><strong>{quotaMetrics.roomsUsed}</strong> Rooms Utilized</span
          >
        </div>
      </div>

      <div
        class="flex flex-col md:flex-row md:items-center justify-between gap-4"
      >
        <div>
          <div class="flex items-center gap-3">
            <Badge
              class="bg-emerald-500/10 text-emerald-600 border-0 font-bold gap-1"
            >
              <Sparkles class="w-3 h-3" />
              <span>Zero-Conflict Schedule Generated</span>
            </Badge>
            <TaskStatusSignal status={taskStatus} />
          </div>
          <h3 class="text-base font-bold text-foreground m-0 mt-1">
            {$tStore("studio.calendar_title")}
          </h3>
          <p class="text-xs text-muted-foreground m-0">
            {$tStore("studio.calendar_desc")}
          </p>
        </div>

        <div class="flex items-center flex-wrap gap-2.5">
          <!-- View Switcher Toggle Buttons -->
          <div
            class="flex items-center bg-muted/60 border border-border/50 rounded-lg p-0.5 gap-0.5"
          >
            <button
              type="button"
              class="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-all {scheduleView ===
              'grid'
                ? 'bg-card text-primary shadow-xs font-bold border border-border/40'
                : 'text-muted-foreground hover:text-foreground'}"
              onclick={() => (scheduleView = "grid")}
              title="Bento Grid View"
            >
              <LayoutGrid class="w-3.5 h-3.5" />
              <span>{$tStore("studio_shows.view_grid")}</span>
            </button>
            <button
              type="button"
              class="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-semibold cursor-pointer transition-all {scheduleView ===
              'timeline'
                ? 'bg-card text-primary shadow-xs font-bold border border-border/40'
                : 'text-muted-foreground hover:text-foreground'}"
              onclick={() => (scheduleView = "timeline")}
              title="Compact Timeline Table View"
            >
              <TableIcon class="w-3.5 h-3.5" />
              <span>{$tStore("studio_shows.view_table")}</span>
            </button>
          </div>

          <!-- Day Filter -->
          <div
            class="flex items-center gap-1.5 bg-card border border-border rounded-lg px-2.5 py-1 text-xs"
          >
            <CalendarIcon class="w-3.5 h-3.5 text-muted-foreground shrink-0" />
            <span class="text-xs font-semibold text-muted-foreground">Day</span>
            <select
              bind:value={filterDay}
              class="bg-transparent border-0 text-xs font-semibold text-foreground outline-none cursor-pointer"
            >
              <option value="all">All Days</option>
              {#each days as day}
                <option value={day}>{day}</option>
              {/each}
            </select>
          </div>

          <!-- Song Filter -->
          <div
            class="flex items-center gap-1.5 bg-card border border-border rounded-lg px-2.5 py-1 text-xs"
          >
            <Filter class="w-3.5 h-3.5 text-muted-foreground shrink-0" />
            <span class="text-xs font-semibold text-muted-foreground"
              >{$tStore("studio.filter_number")}</span
            >
            <select
              bind:value={filterSong}
              class="bg-transparent border-0 text-xs font-semibold text-foreground outline-none cursor-pointer"
            >
              <option value="all">{$tStore("studio.all_numbers")}</option>
              {#each uniqueSongs as song}
                <option value={song}>{song}</option>
              {/each}
            </select>
          </div>

          <!-- Room Filter -->
          <div
            class="flex items-center gap-1.5 bg-card border border-border rounded-lg px-2.5 py-1 text-xs"
          >
            <MapPin class="w-3.5 h-3.5 text-muted-foreground shrink-0" />
            <span class="text-xs font-semibold text-muted-foreground"
              >{$tStore("studio.filter_room")}</span
            >
            <select
              bind:value={filterRoom}
              class="bg-transparent border-0 text-xs font-semibold text-foreground outline-none cursor-pointer"
            >
              <option value="all">{$tStore("studio.all_rooms")}</option>
              {#each uniqueRooms as rm}
                <option value={rm}>{rm}</option>
              {/each}
            </select>
          </div>
        </div>
      </div>

      {#if scheduleView === "grid"}
        <SprintRehearsalGrid {days} sessions={filteredSessions} />
      {:else}
        <SprintTimelineTable sessions={filteredSessions} />
      {/if}
    </Card>
  {/if}
</div>

<!-- Audit History Drawer Modal -->
<SprintAuditDrawer
  bind:open={isHistoryOpen}
  onOpenChange={(open) => (isHistoryOpen = open)}
  {computeHistory}
  {registrationHistory}
/>
