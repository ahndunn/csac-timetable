<script lang="ts">
  import type { SongVoteData, DayOfWeek } from '../types/timetable';
  import { DAYS_OF_WEEK, DEFAULT_TIME_SLOTS } from '../constants/timetableDefaults';
  import { X, CheckSquare, Square } from '@lucide/svelte';

  interface Props {
    song: SongVoteData | null;
    onClose: () => void;
    onToggleVote: (songId: string, day: DayOfWeek, slot: string, member: string) => void;
  }

  let { song, onClose, onToggleVote }: Props = $props();

  const days = DAYS_OF_WEEK;
  const slots = DEFAULT_TIME_SLOTS;
</script>

{#if song}
  <div class="modal-overlay" onclick={onClose} role="presentation">
    <div
      class="modal-dialog"
      style="max-width: 880px;"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
    >
      <div
        class="modal-header"
        style="background-color: {song.color.bg}; border-bottom: 2px solid {song.color.border};"
      >
        <div style="display: flex; align-items: center; gap: 8px;">
          <div
            style="width: 14px; height: 14px; border-radius: 3px; background-color: {song.color.border};"
          ></div>
          <h3 class="modal-header-title" style="color: {song.color.text};">
            Bảng Vote Chi Tiết: {song.name}
          </h3>
        </div>
        <button type="button" class="modal-close-btn" onclick={onClose} aria-label="Đóng">
          <X size={18} />
        </button>
      </div>

      <div class="modal-body" style="overflow-x: auto; padding: 12px;">
        <div style="font-size: 12px; color: #6b7280; margin-bottom: 8px;">
          Bạn có thể bấm vào các ô checkbox để bật/tắt trạng thái rảnh của từng thành viên trực tiếp:
        </div>

        <table style="width: 100%; border-collapse: collapse; font-size: 12px;">
          <thead>
            <tr style="background-color: #f3f4f6;">
              <th style="border: 1px solid #d1d5db; padding: 6px; text-align: left;">Thành viên</th>
              {#each days as day}
                <th
                  colspan={slots.length}
                  style="border: 1px solid #d1d5db; padding: 6px; text-align: center; background-color: #e5e7eb;"
                >
                  {day}
                </th>
              {/each}
            </tr>
            <tr style="background-color: #f9fafb;">
              <th style="border: 1px solid #d1d5db; padding: 4px;"></th>
              {#each days as day}
                {#each slots as slot}
                  <th
                    style="border: 1px solid #d1d5db; padding: 4px; text-align: center; font-size: 10px; font-weight: normal; color: #4b5563;"
                  >
                    {slot.split(' - ')[0]}
                  </th>
                {/each}
              {/each}
            </tr>
          </thead>
          <tbody>
            {#each song.members as member}
              <tr>
                <td style="border: 1px solid #d1d5db; padding: 6px 8px; font-weight: 500;">
                  {member}
                </td>
                {#each days as day}
                  {#each slots as slot}
                    {@const isAvail = !!song.availability[`${day}__${slot}__${member}`]}
                    <td
                      style="border: 1px solid #d1d5db; text-align: center; padding: 4px; cursor: pointer; background-color: {isAvail ? '#ecfdf5' : '#ffffff'};"
                      onclick={() => onToggleVote(song.id, day, slot, member)}
                    >
                      {#if isAvail}
                        <CheckSquare size={14} color="#059669" />
                      {:else}
                        <Square size={14} color="#d1d5db" />
                      {/if}
                    </td>
                  {/each}
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <div class="modal-footer">
        <button type="button" class="btn-gcal-primary" onclick={onClose}>
          Xong
        </button>
      </div>
    </div>
  </div>
{/if}
