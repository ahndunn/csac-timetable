<script lang="ts">
  import type { SongVoteData, ScheduledSession, DayOfWeek } from '../types/timetable';
  import { getSlotAttendance } from '../engine/scheduler';
  import { Plus, X, AlertCircle } from '@lucide/svelte';

  interface Props {
    day: DayOfWeek | null;
    slot: string | null;
    songs: SongVoteData[];
    schedule: ScheduledSession[];
    onClose: () => void;
    onAssignSong: (song: SongVoteData, day: DayOfWeek, slot: string) => void;
  }

  let { day, slot, songs, schedule, onClose, onAssignSong }: Props = $props();

  let existingInSlot = $derived(
    day && slot ? schedule.filter(s => s.day === day && s.slot === slot) : []
  );
</script>

{#if day && slot}
  <div class="modal-overlay" onclick={onClose} role="presentation">
    <div class="modal-dialog" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true">
      <div class="modal-header">
        <div style="display: flex; align-items: center; gap: 8px;">
          <Plus size={18} color="#1a73e8" />
          <h3 class="modal-header-title">
            Thêm bài tập vào {day} ({slot})
          </h3>
        </div>
        <button type="button" class="modal-close-btn" onclick={onClose} aria-label="Đóng">
          <X size={18} />
        </button>
      </div>

      <div class="modal-body">
        <div style="font-size: 13px; color: #4b5563;">
          Chọn bài hát bạn muốn phân bổ vào khung giờ này. Danh sách hiển thị tỷ lệ thành viên rảnh và cảnh báo trùng lịch:
        </div>

        <div style="display: flex; flex-direction: column; gap: 8px;">
          {#each songs as song (song.id)}
            {@const attendance = getSlotAttendance(song, day, slot)}
            {@const doubleBookedMembers = existingInSlot.flatMap(existing => 
              song.members.filter(m => existing.allMembers.includes(m)).map(m => `${m} (đã ở bài ${existing.songName})`)
            )}
            {@const isAlreadyInSlot = existingInSlot.some(s => s.songId === song.id)}

            <div
              style="display: flex; align-items: center; justify-content: space-between; padding: 10px 12px; border: 1px solid #e5e7eb; border-radius: 8px; background-color: {isAlreadyInSlot ? '#f3f4f6' : '#ffffff'};"
            >
              <div style="display: flex; flex-direction: column; gap: 4px;">
                <div style="display: flex; align-items: center; gap: 8px;">
                  <div
                    style="width: 12px; height: 12px; border-radius: 3px; background-color: {song.color.border};"
                  ></div>
                  <strong style="font-size: 14px; color: #1f2937;">
                    {song.name}
                  </strong>
                  <span
                    style="font-size: 11px; padding: 1px 6px; border-radius: 4px; background-color: {attendance.is100Percent ? '#dcfce7' : '#fee2e2'}; color: {attendance.is100Percent ? '#15803d' : '#b91c1c'}; font-weight: 600;"
                  >
                    {attendance.availableMembers.length}/{song.members.length} thành viên rảnh
                  </span>
                </div>

                {#if doubleBookedMembers.length > 0}
                  <div style="font-size: 11px; color: #dc2626; display: flex; align-items: center; gap: 4px;">
                    <AlertCircle size={12} />
                    <span>Trùng thành viên: {doubleBookedMembers.join(', ')}</span>
                  </div>
                {/if}

                {#if attendance.absentMembers.length > 0 && doubleBookedMembers.length === 0}
                  <div style="font-size: 11px; color: #d97706;">
                    Vắng theo vote: {attendance.absentMembers.join(', ')}
                  </div>
                {/if}
              </div>

              <button
                type="button"
                class="btn-gcal-secondary"
                disabled={isAlreadyInSlot}
                onclick={() => {
                  onAssignSong(song, day, slot);
                  onClose();
                }}
                style="font-size: 12px; padding: 5px 12px; background-color: {isAlreadyInSlot ? '#e5e7eb' : '#ffffff'};"
              >
                {isAlreadyInSlot ? 'Đã xếp ở ô này' : 'Xếp vào ô này'}
              </button>
            </div>
          {/each}
        </div>
      </div>

      <div class="modal-footer">
        <button type="button" class="btn-gcal-secondary" onclick={onClose}>
          Đóng
        </button>
      </div>
    </div>
  </div>
{/if}
