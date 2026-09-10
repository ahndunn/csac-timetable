<script lang="ts">
  import type { SongVoteData, ScheduledSession, DayOfWeek } from '../types/timetable';
  import { getSlotAttendance } from '../engine/scheduler';
  import { Plus, X, AlertCircle } from '@lucide/svelte';
  import { tStore, currentLocale } from '$lib/i18n';
  import { DAY_DISPLAY_LABELS } from '../constants/timetableDefaults';

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

  let dayLabels = $derived(DAY_DISPLAY_LABELS[$currentLocale] || DAY_DISPLAY_LABELS.vi);
  let localizedDay = $derived(day ? (dayLabels[day]?.full || day) : '');
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
            {$tStore('slot_add_modal.title', { day: localizedDay, slot })}
          </h3>
        </div>
        <button type="button" class="modal-close-btn" onclick={onClose} aria-label={$tStore('slot_add_modal.close')}>
          <X size={16} />
        </button>
      </div>

      <div class="modal-body">
        <div style="font-size: 13px; color: var(--text-secondary);">
          {$tStore('slot_add_modal.intro')}
        </div>

        <div style="display: flex; flex-direction: column; gap: 10px;">
          {#each songs as song (song.id)}
            {@const attendance = getSlotAttendance(song, day, slot)}
            {@const doubleBookedMembers = existingInSlot.flatMap(existing => 
              song.members.filter(m => existing.allMembers.includes(m)).map(m => 
                $tStore('slot_add_modal.busy_in_song', { member: m, song: existing.songName })
              )
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
                    {$tStore('slot_add_modal.members_free', { available: attendance.availableMembers.length, total: song.members.length })}
                  </span>
                </div>

                {#if doubleBookedMembers.length > 0}
                  <div style="font-size: 11px; color: var(--danger-text); display: flex; align-items: center; gap: 4px;">
                    <AlertCircle size={12} />
                    <span>{$tStore('slot_add_modal.overlap_warning', { members: doubleBookedMembers.join(', ') })}</span>
                  </div>
                {/if}

                {#if attendance.absentMembers.length > 0 && doubleBookedMembers.length === 0}
                  <div style="font-size: 11px; color: var(--warning-text);">
                    {$tStore('slot_add_modal.absent_by_vote', { members: attendance.absentMembers.join(', ') })}
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
                {isAlreadyInSlot ? $tStore('slot_add_modal.already_assigned') : $tStore('slot_add_modal.assign_btn')}
              </button>
            </div>
          {/each}
        </div>
      </div>

      <div class="modal-footer">
        <button type="button" class="bento-btn" onclick={onClose}>
          {$tStore('slot_add_modal.close')}
        </button>
      </div>
    </div>
  </div>
{/if}
