<script lang="ts">
  import type { ScheduledSession, SongVoteData, DayOfWeek, ConflictItem, UnresolvedSong } from '../types/timetable';
  import { DAYS_OF_WEEK, DAY_DISPLAY_LABELS, DEFAULT_TIME_SLOTS } from '../constants/timetableDefaults';
  import { AlertTriangle, CheckCircle, Plus, Info, Upload, FileSpreadsheet, Layers, Files } from '@lucide/svelte';
  import { getWeekDays, isSameDay } from '../utils/dateUtils';
  import { tStore, currentLocale } from '$lib/i18n';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';
  import { cn } from '$lib/utils';

  interface Props {
    schedule: ScheduledSession[];
    songs: SongVoteData[];
    conflicts: ConflictItem[];
    unresolved: UnresolvedSong[];
    selectedMember: string | null;
    onSelectSession: (session: ScheduledSession) => void;
    onOpenSlotAdd: (day: DayOfWeek, slot: string) => void;
    onOpenConflictResolver: () => void;
    onLoadSampleMultiTab?: () => void;
    onLoadSampleSingleTab?: () => void;
    onOpenUpload?: () => void;
    onDownloadTemplate?: () => void;
    selectedWeekStart: Date;
    days?: DayOfWeek[];
    slots?: string[];
  }

  let {
    schedule,
    songs,
    conflicts,
    unresolved,
    selectedMember,
    onSelectSession,
    onOpenSlotAdd,
    onOpenConflictResolver,
    onLoadSampleMultiTab,
    onLoadSampleSingleTab,
    onOpenUpload,
    onDownloadTemplate,
    selectedWeekStart,
    days = DAYS_OF_WEEK,
    slots = DEFAULT_TIME_SLOTS,
  }: Props = $props();

  let weekDays = $derived(getWeekDays(selectedWeekStart));
  let totalSessionsRequested = $derived(songs.reduce((acc, s) => acc + s.targetSessions, 0));
  let totalSessionsScheduled = $derived(schedule.length);
  let hasUnresolved = $derived(unresolved.length > 0);
  let hasMemberConflict = $derived(conflicts.some(c => c.type === 'member_double_booked'));

  function getSlotConflicts(day: DayOfWeek, slot: string) {
    return conflicts.filter(c => c.day === day && c.slot === slot);
  }
</script>

<main class="flex-1 flex flex-col p-4 gap-4 overflow-y-auto bg-background">
  <!-- Top Status & Conflict Alert Banner -->
  <div
    class={cn(
      "flex flex-wrap items-center justify-between gap-3 p-3.5 rounded-xl border transition-all duration-200",
      hasUnresolved || hasMemberConflict
        ? "bg-amber-50/80 border-amber-200 text-amber-900 dark:bg-amber-950/20 dark:border-amber-900/50 dark:text-amber-200"
        : totalSessionsScheduled > 0
          ? "bg-emerald-50/80 border-emerald-200 text-emerald-900 dark:bg-emerald-950/20 dark:border-emerald-900/50 dark:text-emerald-200"
          : "bg-white border-slate-200 text-slate-700 dark:bg-card dark:border-slate-800 dark:text-slate-200"
    )}
  >
    <div class="flex items-center gap-3">
      {#if hasUnresolved || hasMemberConflict}
        <AlertTriangle size={20} class="text-amber-500 shrink-0" />
        <div class="flex flex-col">
          <div class="text-xs font-bold">
            {$tStore('calendar.status_conflict_title', {
              scheduled: totalSessionsScheduled,
              requested: totalSessionsRequested,
              conflicts: conflicts.length
            })}
          </div>
          <div class="text-[11px] text-amber-700/80 dark:text-amber-300/80">
            {$tStore('calendar.status_conflict_desc')}
          </div>
        </div>
      {:else if totalSessionsScheduled > 0}
        <CheckCircle size={20} class="text-emerald-500 shrink-0" />
        <div class="flex flex-col">
          <div class="text-xs font-bold">
            {$tStore('calendar.status_all_good_title', { count: totalSessionsScheduled })}
          </div>
          <div class="text-[11px] text-emerald-700/80 dark:text-emerald-300/80">
            {$tStore('calendar.status_all_good_desc')}
          </div>
        </div>
      {:else}
        <Info size={20} class="text-primary shrink-0" />
        <div class="flex flex-col">
          <div class="text-xs font-bold">
            {$tStore('calendar.status_ready_title', { songs: songs.length, requested: totalSessionsRequested })}
          </div>
          <div class="text-[11px] text-slate-500">
            {$tStore('calendar.status_ready_desc')}
          </div>
        </div>
      {/if}
    </div>

    {#if hasUnresolved || hasMemberConflict}
      <Button
        variant="default"
        size="sm"
        onclick={onOpenConflictResolver}
      >
        <AlertTriangle size={14} class="mr-1.5" />
        <span>{$tStore('calendar.resolve_conflicts_btn')}</span>
      </Button>
    {/if}
  </div>

  <!-- Empty state if zero songs loaded -->
  {#if songs.length === 0}
    <Card class="flex flex-col items-center justify-center p-12 text-center border-dashed border-2 border-slate-200 bg-white/60 dark:border-slate-800 dark:bg-card/50">
      <div class="flex h-14 w-14 items-center justify-center rounded-2xl bg-primary/10 text-primary mb-4 dark:bg-primary/20">
        <FileSpreadsheet size={28} />
      </div>
      <h3 class="text-base font-bold text-slate-900 dark:text-slate-100 mb-1">
        {$tStore('calendar.empty_title')}
      </h3>
      <p class="text-xs text-slate-500 max-w-sm mb-6">
        {$tStore('calendar.empty_subtitle')}
      </p>

      <div class="flex flex-wrap items-center justify-center gap-3">
        {#if onOpenUpload}
          <Button variant="default" size="sm" onclick={onOpenUpload}>
            <Upload size={14} class="mr-1.5" />
            <span>{$tStore('calendar.empty_upload_btn')}</span>
          </Button>
        {/if}
        {#if onLoadSampleMultiTab}
          <Button variant="outline" size="sm" onclick={onLoadSampleMultiTab}>
            <Layers size={14} class="mr-1.5 text-blue-600" />
            <span>{$tStore('calendar.empty_sample_multi')}</span>
          </Button>
        {/if}
        {#if onLoadSampleSingleTab}
          <Button variant="outline" size="sm" onclick={onLoadSampleSingleTab}>
            <Files size={14} class="mr-1.5 text-emerald-600" />
            <span>{$tStore('calendar.empty_sample_single')}</span>
          </Button>
        {/if}
      </div>
    </Card>
  {/if}

  <!-- Bento Calendar Viewport -->
  <div class="flex-1 overflow-x-auto rounded-[18px] border border-black/[0.08] bg-white shadow-sm dark:border-white/[0.08] dark:bg-card">
    <table class="w-full border-collapse text-left min-w-[760px]">
      <thead>
        <tr class="border-b border-slate-200 bg-slate-50/80 dark:border-slate-800 dark:bg-slate-900/50">
          <th class="w-20 p-3 text-center text-[11px] font-bold text-slate-400 uppercase tracking-wider border-r border-slate-200 dark:border-slate-800">GMT+7</th>
          {#each days as day, idx}
            {@const dayDate = weekDays[idx]}
            {@const dateNum = dayDate ? dayDate.getDate() : idx + 1}
            {@const isToday = dayDate ? isSameDay(dayDate, new Date()) : false}
            {@const labels = DAY_DISPLAY_LABELS[$currentLocale] || DAY_DISPLAY_LABELS.vi}
            <th class={cn("p-3 border-r border-slate-200 dark:border-slate-800 last:border-r-0", isToday && "bg-primary/10")}>
              <div class="flex items-center justify-between">
                <span class="text-xs font-bold text-slate-800 dark:text-slate-200">{labels[day]?.short || day}</span>
                <span class={cn("text-[11px] font-semibold text-slate-400", isToday && "text-primary font-bold")}>
                  {$tStore('calendar.day_date', { date: dateNum })}
                </span>
              </div>
            </th>
          {/each}
        </tr>
      </thead>

      <tbody>
        {#each slots as slot}
          <tr class="border-b border-slate-100 dark:border-slate-800/60 last:border-b-0">
            <!-- Time Gutter -->
            <td class="p-2.5 text-center text-xs font-semibold text-slate-500 border-r border-slate-200 bg-slate-50/40 dark:border-slate-800 dark:bg-slate-900/20">{slot}</td>

            <!-- Day Columns -->
            {#each days as day}
              {@const sessionsInSlot = schedule.filter(s => s.day === day && s.slot === slot)}
              {@const slotConflicts = getSlotConflicts(day, slot)}
              {@const containsSelectedMember = selectedMember
                ? sessionsInSlot.some(s => s.allMembers.includes(selectedMember))
                : false}

              <td class={cn("group/slot relative p-1.5 align-top border-r border-slate-100 min-h-[64px] transition-colors dark:border-slate-800/60 last:border-r-0 hover:bg-slate-50/50 dark:hover:bg-slate-800/30", containsSelectedMember && "bg-primary/10 border-primary")}>
                <div class="flex flex-col gap-1.5 min-h-[50px]">
                  <!-- Conflict notification in cell if any -->
                  {#if slotConflicts.length > 0}
                    <div
                      class="flex items-center gap-1 rounded px-1.5 py-0.5 text-[10px] font-bold bg-rose-50 text-rose-700 border border-rose-200"
                      title={slotConflicts.map(c => c.message).join('\n')}
                    >
                      <AlertTriangle size={10} />
                      <span>{$tStore('calendar.conflict_member_cell')}</span>
                    </div>
                  {/if}

                  <!-- Scheduled Event Cards in Bento Modular Style -->
                  {#each sessionsInSlot as sess (sess.id)}
                    {@const isMemberInThisSong = selectedMember ? sess.allMembers.includes(selectedMember) : true}
                    {@const isDimmed = selectedMember && !isMemberInThisSong}
                    {@const isPerfect = sess.absentMembers.length === 0}

                    <div
                      class={cn(
                        "flex flex-col gap-1 rounded-lg border border-l-4 bg-white p-2 shadow-xs cursor-pointer transition-all duration-200 hover:shadow-md hover:scale-[1.01] dark:bg-slate-900",
                        isDimmed && "opacity-40 grayscale"
                      )}
                      style="border-left-color: {sess.color.border};"
                      onclick={() => onSelectSession(sess)}
                      onkeydown={(e) => { if (e.key === 'Enter') onSelectSession(sess); }}
                      role="button"
                      tabindex="0"
                      title={$tStore('calendar.card_tooltip')}
                    >
                      <div class="flex items-center justify-between gap-1">
                        <span class="text-xs font-bold text-slate-900 truncate dark:text-slate-100">
                          {sess.songName}
                        </span>
                        <Badge variant="secondary" class="text-[9px] px-1 py-0">
                          {$tStore('calendar.room_tag', { room: sess.room })}
                        </Badge>
                      </div>

                      <div class={cn("text-[10px] font-semibold", isPerfect ? "text-emerald-600" : "text-amber-600")}>
                        {#if isPerfect}
                          ✓ {$tStore('calendar.attendance_perfect', { count: sess.allMembers.length, total: sess.allMembers.length })}
                        {:else}
                          ⚠ {$tStore('calendar.attendance_partial', { count: sess.absentMembers.length })}
                        {/if}
                      </div>

                      <div class="text-[10px] text-slate-400 truncate">
                        {sess.allMembers.join(', ')}
                      </div>
                    </div>
                  {/each}

                  <!-- Quick add button on hover -->
                  <button
                    type="button"
                    class="opacity-0 group-hover/slot:opacity-100 flex items-center justify-center h-6 w-full rounded border border-dashed border-slate-300 text-slate-400 hover:border-primary hover:text-primary hover:bg-primary/10 transition-all text-xs"
                    onclick={() => onOpenSlotAdd(day, slot)}
                    title={$tStore('calendar.add_slot_tooltip')}
                    aria-label={$tStore('calendar.add_slot_tooltip')}
                  >
                    <Plus size={13} />
                  </button>
                </div>
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</main>
