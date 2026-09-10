<script lang="ts">
  import type { SongVoteData, DayOfWeek } from '../types/timetable';
  import { DAYS_OF_WEEK, DEFAULT_TIME_SLOTS, DAY_DISPLAY_LABELS } from '../constants/timetableDefaults';
  import { X, CheckSquare, Square } from '@lucide/svelte';
  import { tStore, currentLocale } from '$lib/i18n';

  interface Props {
    song: SongVoteData | null;
    onClose: () => void;
    onToggleVote: (songId: string, day: DayOfWeek, slot: string, member: string) => void;
  }

  let { song, onClose, onToggleVote }: Props = $props();

  const days = DAYS_OF_WEEK;
  const slots = DEFAULT_TIME_SLOTS;

  let dayLabels = $derived(DAY_DISPLAY_LABELS[$currentLocale] || DAY_DISPLAY_LABELS.vi);
</script>

{#if song}
  <div
    class="modal-overlay"
    onclick={onClose}
    onkeydown={(e) => { if (e.key === 'Escape') onClose(); }}
    role="presentation"
  >
    <div
      class="modal-dialog"
      style="max-width: 900px;"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
    >
      <div class="modal-header">
        <div style="display: flex; align-items: center; gap: 8px;">
          <div
            style="width: 14px; height: 14px; border-radius: var(--radius-circle); background-color: {song.color.border};"
          ></div>
          <h3 class="modal-header-title">
            {$tStore('raw_vote_modal.title', { song: song.name })}
          </h3>
        </div>
        <button type="button" class="modal-close-btn" onclick={onClose} aria-label={$tStore('raw_vote_modal.close')}>
          <X size={16} />
        </button>
      </div>

      <div class="modal-body" style="overflow-x: auto; padding: 16px;">
        <div style="font-size: 12px; color: var(--text-muted); margin-bottom: 8px;">
          {$tStore('raw_vote_modal.hint')}
        </div>

        <table class="raw-vote-table">
          <thead>
            <!-- Sheet Title Row -->
            <tr>
              <th
                colspan={3 + song.members.length}
                style="font-size: 13px; color: var(--accent); padding: 10px; background: var(--accent-light);"
              >
                {song.weekTitle || $tStore('raw_vote_modal.default_title')}
              </th>
            </tr>

            <!-- Header row 1 -->
            <tr>
              <th rowspan={2} style="width: 80px;">{$tStore('raw_vote_modal.th_day')}</th>
              <th rowspan={2} style="width: 90px;">{$tStore('raw_vote_modal.th_time')}</th>
              <th colspan={song.members.length}>{$tStore('raw_vote_modal.th_members')}</th>
              <th rowspan={2} style="width: 120px;">{$tStore('raw_vote_modal.th_notes')}</th>
            </tr>

            <!-- Header row 2: Member names -->
            <tr>
              {#each song.members as m (m)}
                <th>{m}</th>
              {/each}
            </tr>
          </thead>

          <tbody>
            {#each days as day}
              {@const localizedDay = dayLabels[day]?.full || day}
              {#each slots as slot, slotIdx}
                {@const noteKey = `${day}__${slot}`}
                {@const note = song.notes[noteKey] || ''}

                <tr>
                  {#if slotIdx === 0}
                    <td rowspan={slots.length} style="font-weight: 700; color: var(--text-primary); vertical-align: middle;">
                      {localizedDay}
                    </td>
                  {/if}

                  <td style="font-size: 11px;">{slot}</td>

                  {#each song.members as member (member)}
                    {@const key = `${day}__${slot}__${member}`}
                    {@const isChecked = !!song.availability[key]}

                    <td
                      class="{isChecked ? 'is-checked' : ''}"
                      style="cursor: pointer;"
                      onclick={() => onToggleVote(song.id, day, slot, member)}
                      title={$tStore('raw_vote_modal.toggle_tooltip', { member })}
                    >
                      {#if isChecked}
                        <CheckSquare size={16} color="var(--success-text)" />
                      {:else}
                        <Square size={16} color="var(--text-muted)" />
                      {/if}
                    </td>
                  {/each}

                  <td style="font-size: 11px; text-align: left; padding: 4px 8px;">
                    {note}
                  </td>
                </tr>
              {/each}
            {/each}
          </tbody>
        </table>
      </div>

      <div class="modal-footer">
        <button type="button" class="bento-btn bento-btn-primary" onclick={onClose}>
          {$tStore('raw_vote_modal.close')}
        </button>
      </div>
    </div>
  </div>
{/if}
