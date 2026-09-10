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
  <div
    class="modal-overlay"
    onclick={onClose}
    onkeydown={(e) => { if (e.key === 'Escape') onClose(); }}
    role="presentation"
  >
    <div
      class="modal-dialog"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
    >
      <div class="modal-header">
        <div style="display: flex; align-items: center; gap: 8px;">
          <Plus size={18} color="var(--accent)" />
          <h3 class="modal-header-title">
            Thêm bài tập vào {day} ({slot})
          </h3>
        </div>
        <button type="button" class="modal-close-btn" onclick={onClose} aria-label="Đóng">
          <X size={16} />
        </button>
      </div>

      <div class="modal-body">
        <div style="font-size: 13px; color: var(--text-secondary);">
          Chọn bài hát bạn muốn phân bổ vào khung giờ này. Danh sách hiển thị tỷ lệ thành viên rảnh và cảnh báo trùng lịch:
        </div>

        <div style="display: flex; flex-direction: column; gap: 10px;">
          {#each songs as song (song.id)}
            {@const attendance = getSlotAttendance(song, day, slot)}
            {@const doubleBookedMembers = existingInSlot.flatMap(existing => 
              song.members.filter(m => existing.allMembers.includes(m)).map(m => `${m} (đã ở bài ${existing.songName})`)
            )}
            {@const isAlreadyInSlot = existingInSlot.some(s => s.songId === song.id)}

            <div
              class="bento-card"
              style="display: flex; align-items: center; justify-content: space-between; padding: 12px 14px; gap: 10px; {isAlreadyInSlot ? 'background: var(--surface-card-subtle); opacity: 0.75;' : ''}"
            >
              <div style="display: flex; flex-direction: column; gap: 4px;">
                <div style="display: flex; align-items: center; gap: 8px;">
                  <div
                    style="width: 12px; height: 12px; border-radius: var(--radius-circle); background-color: {song.color.border};"
                  ></div>
                  <strong style="font-size: 14px; color: var(--text-primary);">
                    {song.name}
                  </strong>
                  <span
                    class="bento-pill {attendance.is100Percent ? 'is-active' : ''}"
                    style="font-size: 10px; {attendance.is100Percent ? '' : 'color: var(--danger-text); background: var(--danger-light);'}"
                  >
                    {attendance.availableMembers.length}/{song.members.length} rảnh
                  </span>
                </div>

                {#if doubleBookedMembers.length > 0}
                  <div style="font-size: 11px; color: var(--danger-text); display: flex; align-items: center; gap: 4px;">
                    <AlertCircle size={12} />
                    <span>Trùng: {doubleBookedMembers.join(', ')}</span>
                  </div>
                {/if}

                {#if attendance.absentMembers.length > 0 && doubleBookedMembers.length === 0}
                  <div style="font-size: 11px; color: var(--warning-text);">
                    Vắng theo vote: {attendance.absentMembers.join(', ')}
                  </div>
                {/if}
              </div>

              <button
                type="button"
                class="bento-btn {isAlreadyInSlot ? '' : 'bento-btn-primary'}"
                disabled={isAlreadyInSlot}
                onclick={() => {
                  onAssignSong(song, day, slot);
                  onClose();
                }}
                style="font-size: 12px; padding: 6px 12px;"
              >
                {isAlreadyInSlot ? 'Đã xếp ở ô này' : 'Xếp vào đây'}
              </button>
            </div>
          {/each}
        </div>
      </div>

      <div class="modal-footer">
        <button type="button" class="bento-btn" onclick={onClose}>
          Đóng
        </button>
      </div>
    </div>
  </div>
{/if}
