<script lang="ts">
  import type { ScheduledSession, SongVoteData, DayOfWeek, ConflictItem, UnresolvedSong } from '../types/timetable';
  import { DAYS_OF_WEEK, DAY_DISPLAY_LABELS, DEFAULT_TIME_SLOTS } from '../constants/timetableDefaults';
  import { AlertTriangle, CheckCircle, Plus, Info, Upload, FileSpreadsheet, Layers, Files } from '@lucide/svelte';
  import { getWeekDays, isSameDay } from '../utils/dateUtils';
  import { tStore, currentLocale } from '$lib/i18n';

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

<main class="calendar-main">
  <!-- Top Status & Conflict Alert Banner -->
  <div
    class="status-banner {hasUnresolved || hasMemberConflict ? 'has-conflict' : totalSessionsScheduled > 0 ? 'all-good' : ''}"
  >
    <div class="status-left">
      {#if hasUnresolved || hasMemberConflict}
        <AlertTriangle size={18} color="var(--warning)" />
      {:else if totalSessionsScheduled > 0}
        <CheckCircle size={18} color="var(--success)" />
      {:else}
        <Info size={18} color="var(--text-muted)" />
      {/if}

      <span>
        {#if songs.length === 0}
          {$tStore('calendar.status_no_data')}
        {:else if totalSessionsScheduled === 0}
          {$tStore('calendar.status_need_schedule', { songs: songs.length, sessions: totalSessionsRequested })}
        {:else if hasUnresolved || hasMemberConflict}
          {$tStore('calendar.status_partial_conflict', {
            scheduled: totalSessionsScheduled,
            requested: totalSessionsRequested,
            unresolved: unresolved.length,
          })}
          {#if hasMemberConflict} {$tStore('calendar.status_member_conflict')}{/if}
        {:else}
          {$tStore('calendar.status_perfect', {
            scheduled: totalSessionsScheduled,
            requested: totalSessionsRequested,
            songs: songs.length,
          })}
        {/if}
      </span>
    </div>

    {#if hasUnresolved || hasMemberConflict}
      <button
        type="button"
        class="status-btn-fix"
        onclick={onOpenConflictResolver}
        title={$tStore('conflict_modal.title')}
      >
        <AlertTriangle size={14} />
        <span>{$tStore('calendar.btn_resolve_conflicts', { count: unresolved.length + conflicts.length })}</span>
      </button>
    {/if}
  </div>

  <!-- Empty State Panel when no songs loaded -->
  {#if songs.length === 0}
    <div class="empty-state-bento">
      <div class="empty-state-title">
        {$tStore('calendar.empty_title')}
      </div>
      <div class="empty-state-subtitle">
        {$tStore('calendar.empty_desc')}
      </div>
      <div class="empty-state-actions">
        {#if onOpenUpload}
          <button type="button" class="bento-btn bento-btn-primary" onclick={onOpenUpload}>
            <Upload size={15} />
            <span>{$tStore('navbar.upload_files')}</span>
          </button>
        {/if}
        {#if onLoadSampleMultiTab}
          <button
            type="button"
            class="bento-btn"
            onclick={onLoadSampleMultiTab}
            title={$tStore('navbar.sample_multitab_desc')}
          >
            <Layers size={15} color="var(--accent)" />
            <span>{$tStore('navbar.sample_multitab_title')}</span>
          </button>
        {/if}
        {#if onLoadSampleSingleTab}
          <button
            type="button"
            class="bento-btn"
            onclick={onLoadSampleSingleTab}
            title={$tStore('navbar.sample_singletab_desc')}
          >
            <Files size={15} color="var(--accent)" />
            <span>{$tStore('navbar.sample_singletab_title')}</span>
          </button>
        {/if}
        {#if onDownloadTemplate}
          <button type="button" class="bento-btn" onclick={onDownloadTemplate}>
            <FileSpreadsheet size={15} />
            <span>{$tStore('navbar.download_template')}</span>
          </button>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Bento Calendar Viewport -->
  <div class="calendar-grid-container">
    <table class="calendar-table">
      <thead>
        <tr>
          <th class="cal-th-time">GMT+7</th>
          {#each days as day, idx}
            {@const dayDate = weekDays[idx]}
            {@const dateNum = dayDate ? dayDate.getDate() : idx + 1}
            {@const isToday = dayDate ? isSameDay(dayDate, new Date()) : false}
            {@const labels = DAY_DISPLAY_LABELS[$currentLocale] || DAY_DISPLAY_LABELS.vi}
            <th class="cal-th-day {isToday ? 'is-today' : ''}">
              <div class="cal-day-title">{labels[day]?.short || day}</div>
              <div class="cal-day-date">{$tStore('calendar.day_date', { date: dateNum })}</div>
            </th>
          {/each}
        </tr>
      </thead>

      <tbody>
        {#each slots as slot}
          <tr>
            <!-- Time Gutter -->
            <td class="cal-time-cell">{slot}</td>

            <!-- Day Columns -->
            {#each days as day}
              {@const sessionsInSlot = schedule.filter(s => s.day === day && s.slot === slot)}
              {@const slotConflicts = getSlotConflicts(day, slot)}
              {@const containsSelectedMember = selectedMember
                ? sessionsInSlot.some(s => s.allMembers.includes(selectedMember))
                : false}

              <td class="cal-slot-cell" style="{containsSelectedMember ? 'border-color: var(--accent); background: var(--accent-light);' : ''}">
                <div class="cal-slot-inner">
                  <!-- Conflict notification in cell if any -->
                  {#if slotConflicts.length > 0}
                    <div
                      class="bento-pill"
                      style="color: var(--danger-text); background: var(--danger-light); border-color: rgba(239, 68, 68, 0.3); margin-bottom: 2px;"
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
                      class="cal-event-card {isDimmed ? 'is-dimmed' : ''}"
                      style="border-left-color: {sess.color.border};"
                      onclick={() => onSelectSession(sess)}
                      onkeydown={(e) => { if (e.key === 'Enter') onSelectSession(sess); }}
                      role="button"
                      tabindex="0"
                      title={$tStore('calendar.card_tooltip')}
                    >
                      <div style="display: flex; align-items: center; justify-content: space-between; gap: 4px;">
                        <span class="event-song-name">
                          {sess.songName}
                        </span>
                        <span class="bento-pill" style="font-size: 9px; padding: 1px 5px;">
                          {$tStore('calendar.room_tag', { room: sess.room })}
                        </span>
                      </div>

                      <div class="event-attendance-badge {isPerfect ? 'is-perfect' : 'is-warning'}">
                        {#if isPerfect}
                          ✓ {$tStore('calendar.attendance_perfect', { count: sess.allMembers.length, total: sess.allMembers.length })}
                        {:else}
                          ⚠ {$tStore('calendar.attendance_partial', { count: sess.absentMembers.length })}
                        {/if}
                      </div>

                      <div style="font-size: 10px; color: var(--text-muted); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; padding-left: 2px;">
                        {sess.allMembers.join(', ')}
                      </div>
                    </div>
                  {/each}

                  <!-- Quick add button on hover -->
                  <button
                    type="button"
                    class="slot-quick-add-btn"
                    onclick={() => onOpenSlotAdd(day, slot)}
                    title={$tStore('calendar.add_slot_tooltip')}
                    aria-label={$tStore('calendar.add_slot_tooltip')}
                  >
                    <Plus size={14} />
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
