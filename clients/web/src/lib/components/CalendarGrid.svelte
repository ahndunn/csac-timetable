<script lang="ts">
  import type { ScheduledSession, SongVoteData, DayOfWeek, ConflictItem, UnresolvedSong } from '../types/timetable';
  import { DAYS_OF_WEEK, DAY_SHORT_LABELS, DEFAULT_TIME_SLOTS } from '../constants/timetableDefaults';
  import { AlertTriangle, CheckCircle, Plus, Info, Upload, FileSpreadsheet, Layers, Files } from '@lucide/svelte';
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
          Chưa có dữ liệu. Vui lòng bấm "Dữ liệu mẫu" hoặc "Tải file Excel" để bắt đầu.
        {:else if totalSessionsScheduled === 0}
          Đã tải {songs.length} bài hát ({totalSessionsRequested} buổi tập yêu cầu). Hãy bấm "Tự động xếp lịch".
        {:else if hasUnresolved || hasMemberConflict}
          Đã xếp <strong>{totalSessionsScheduled}/{totalSessionsRequested}</strong> buổi.
          {#if hasUnresolved} Còn {unresolved.length} bài chưa xếp đủ số buổi do xung đột.{/if}
          {#if hasMemberConflict} Có xung đột trùng giờ thành viên!{/if}
        {:else}
          Hoàn hảo! Đã xếp đủ <strong>{totalSessionsScheduled}/{totalSessionsRequested}</strong> buổi tập cho {songs.length} bài hát không trùng giờ bất kỳ ai.
        {/if}
      </span>
    </div>

    {#if hasUnresolved || hasMemberConflict}
      <button
        type="button"
        class="status-btn-fix"
        onclick={onOpenConflictResolver}
        title="Xem chi tiết & Xử lý xung đột"
      >
        <AlertTriangle size={14} />
        <span>Xử lý xung đột ({unresolved.length + conflicts.length})</span>
      </button>
    {/if}
  </div>

  <!-- Empty State Panel when no songs loaded -->
  {#if songs.length === 0}
    <div class="empty-state-bento">
      <div class="empty-state-title">
        Chưa có bài hát nào trong hệ thống
      </div>
      <div class="empty-state-subtitle">
        Bạn có thể tải lên các file Excel vote lịch của nhóm, tải về file Excel dữ liệu mẫu của 5 bài hát để xem thử, hoặc bấm "Dữ liệu mẫu" để trải nghiệm xếp lịch ngay.
      </div>
      <div class="empty-state-actions">
        {#if onOpenUpload}
          <button type="button" class="bento-btn bento-btn-primary" onclick={onOpenUpload}>
            <Upload size={15} />
            <span>Tải file Excel lên</span>
          </button>
        {/if}
        {#if onLoadSampleMultiTab}
          <button
            type="button"
            class="bento-btn"
            onclick={onLoadSampleMultiTab}
            title="Thử nghiệm nạp 1 file Excel 5 tab (kích hoạt hộp thoại chọn tab)"
          >
            <Layers size={15} color="var(--accent)" />
            <span>Test Excel nhiều tab</span>
          </button>
        {/if}
        {#if onLoadSampleSingleTab}
          <button
            type="button"
            class="bento-btn"
            onclick={onLoadSampleSingleTab}
            title="Thử nghiệm nạp 5 file Excel mỗi file 1 tab"
          >
            <Files size={15} color="var(--accent)" />
            <span>Test 5 file (1 tab)</span>
          </button>
        {/if}
        {#if onDownloadTemplate}
          <button type="button" class="bento-btn" onclick={onDownloadTemplate}>
            <FileSpreadsheet size={15} />
            <span>Tải template Excel</span>
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
            <th class="cal-th-day {isToday ? 'is-today' : ''}">
              <div class="cal-day-title">{DAY_SHORT_LABELS[day]}</div>
              <div class="cal-day-date">Ngày {dateNum}</div>
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
                      <span>Trùng thành viên!</span>
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
                      title="Bấm để xem chi tiết buổi tập"
                    >
                      <div style="display: flex; align-items: center; justify-content: space-between; gap: 4px;">
                        <span class="event-song-name">
                          {sess.songName}
                        </span>
                        <span class="bento-pill" style="font-size: 9px; padding: 1px 5px;">P.{sess.room}</span>
                      </div>

                      <div class="event-attendance-badge {isPerfect ? 'is-perfect' : 'is-warning'}">
                        {#if isPerfect}
                          ✓ {sess.allMembers.length}/{sess.allMembers.length} đủ
                        {:else}
                          ⚠ Vắng {sess.absentMembers.length}
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
                    title="Thêm bài tập vào khung giờ này"
                    aria-label="Thêm bài"
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
