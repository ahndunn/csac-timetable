<script lang="ts">
  import type { ScheduledSession, SongVoteData, DayOfWeek, ConflictItem, UnresolvedSong } from '../types/timetable';
  import { DAYS_OF_WEEK, DAY_SHORT_LABELS, DEFAULT_TIME_SLOTS } from '../constants/timetableDefaults';
  import { AlertTriangle, CheckCircle, Plus, Info, Upload, Sparkles, FileSpreadsheet, Layers, Files } from '@lucide/svelte';
  import { getWeekDays, isSameDay } from '../utils/dateUtils';

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
        <AlertTriangle size={18} color="#b45309" />
      {:else if totalSessionsScheduled > 0}
        <CheckCircle size={18} color="#16a34a" />
      {:else}
        <Info size={18} color="#4b5563" />
      {/if}

      <span>
        {#if songs.length === 0}
          Chưa có dữ liệu. Vui lòng bấm "Dữ liệu mẫu" hoặc "Tải file Excel" để bắt đầu.
        {:else if hasUnresolved}
          Có {unresolved.length} bài chưa thể xếp lịch tự động do quá tải phòng hoặc trùng lịch.
        {:else if hasMemberConflict}
          Có xung đột trùng lịch giữa các thành viên!
        {:else if totalSessionsScheduled > 0}
          Lịch tập tối ưu hoàn tất: Đã xếp {totalSessionsScheduled}/{totalSessionsRequested} buổi với 100% chuyên cần!
        {:else}
          Đã tải {songs.length} bài hát ({totalSessionsRequested} buổi). Bấm "Tự Động Xếp Lịch" để bắt đầu.
        {/if}
      </span>
    </div>

    <div class="status-right">
      {#if hasUnresolved}
        <button
          type="button"
          class="btn-status-resolve"
          onclick={onOpenConflictResolver}
          title="Mở bảng hỗ trợ giải quyết xung đột"
        >
          <span>Xem & Giải Quyết Xung Đột ({unresolved.length})</span>
        </button>
      {/if}
    </div>
  </div>

  <!-- Empty State Onboarding -->
  {#if songs.length === 0}
    <div class="empty-state-card">
      <div class="empty-state-icon">
        <FileSpreadsheet size={48} color="#1a73e8" />
      </div>
      <h2>Chào mừng bạn đến với CSAC Timetable Studio!</h2>
      <p>
        Hệ thống tự động xếp lịch tập phòng nhạc không trùng giờ cho các bài hát của CLB.
      </p>

      <div class="empty-state-actions">
        {#if onOpenUpload}
          <button type="button" class="btn-gcal-primary" onclick={onOpenUpload}>
            <Upload size={16} />
            <span>Tải lên file Excel bầu chọn (.xlsx)</span>
          </button>
        {/if}
        {#if onLoadSampleMultiTab}
          <button type="button" class="btn-gcal-sample" onclick={onLoadSampleMultiTab}>
            <Layers size={16} />
            <span>Thử file Multi-Tab (Chọn Sheet)</span>
          </button>
        {/if}
        {#if onLoadSampleSingleTab}
          <button type="button" class="btn-gcal-secondary" onclick={onLoadSampleSingleTab}>
            <Files size={16} />
            <span>Nạp 5 File Đơn Lẻ</span>
          </button>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Google Calendar Viewport -->
  <div class="calendar-viewport">
    <table class="gcal-table">
      <thead class="gcal-thead">
        <tr>
          <th class="gcal-time-col-header">GMT+7</th>
          {#each days as day, idx}
            {@const dayDate = weekDays[idx]}
            {@const dateNum = dayDate ? dayDate.getDate() : idx + 1}
            {@const isToday = dayDate ? isSameDay(dayDate, new Date()) : false}
            <th class="gcal-day-header {isToday ? 'today' : ''}">
              <div class="day-header-title">{DAY_SHORT_LABELS[day]}</div>
              <div class="day-header-number">{dateNum}</div>
            </th>
          {/each}
        </tr>
      </thead>

      <tbody>
        {#each slots as slot}
          <tr>
            <td class="gcal-time-cell">{slot}</td>

            {#each days as day}
              {@const sessionsInSlot = schedule.filter(s => s.day === day && s.slot === slot)}
              {@const slotConflicts = getSlotConflicts(day, slot)}
              {@const containsSelectedMember = selectedMember ? sessionsInSlot.some(s => s.allMembers.includes(selectedMember)) : false}

              <td class="gcal-slot-cell {containsSelectedMember ? 'highlight-member' : ''}">
                <div class="slot-events-container">
                  {#if slotConflicts.length > 0}
                    <div class="event-conflict-tag" title={slotConflicts.map(c => c.message).join('\n')}>
                      ⚠ Trùng lịch!
                    </div>
                  {/if}

                  {#each sessionsInSlot as sess (sess.id)}
                    {@const isMemberInThisSong = selectedMember ? sess.allMembers.includes(selectedMember) : true}
                    {@const isDimmed = selectedMember && !isMemberInThisSong}
                    {@const isPerfect = sess.absentMembers.length === 0}

                    <div
                      class="event-card"
                      style="background-color: {sess.color.bg}; border-left-color: {sess.color.border}; opacity: {isDimmed ? 0.35 : 1};"
                      onclick={() => onSelectSession(sess)}
                      onkeydown={(e) => { if (e.key === 'Enter') onSelectSession(sess); }}
                      role="button"
                      tabindex="0"
                      title="Bấm để xem chi tiết buổi tập"
                    >
                      <div class="event-card-top">
                        <span class="event-song-name" style="color: {sess.color.text};">
                          {sess.songName}
                        </span>
                        {#if sess.room > 1}
                          <span class="event-room-tag">P{sess.room}</span>
                        {/if}
                      </div>

                      <div class="event-card-members">
                        👥 {sess.availableMembers.length}/{sess.allMembers.length}
                        {#if !isPerfect}
                          <span class="event-absent-flag" title="Thiếu: {sess.absentMembers.join(', ')}">
                            (-{sess.absentMembers.length})
                          </span>
                        {/if}
                      </div>

                      {#if sess.note}
                        <div class="event-card-note" title={sess.note}>
                          📝 {sess.note}
                        </div>
                      {/if}
                    </div>
                  {/each}

                  <button
                    type="button"
                    class="btn-slot-add"
                    onclick={() => onOpenSlotAdd(day, slot)}
                    title="Thêm lịch tập thủ công vào {day} ({slot})"
                    aria-label="Thêm buổi tập"
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
