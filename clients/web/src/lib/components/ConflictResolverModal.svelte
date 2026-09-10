<script lang="ts">
  import type { UnresolvedSong, ConflictItem, DayOfWeek, SongVoteData } from '../types/timetable';
  import { AlertTriangle, X, Check } from '@lucide/svelte';
  import { tStore, currentLocale } from '$lib/i18n';
  import { DAY_DISPLAY_LABELS } from '../constants/timetableDefaults';

  interface Props {
    unresolved: UnresolvedSong[];
    conflicts: ConflictItem[];
    songs: SongVoteData[];
    onClose: () => void;
    onManualAssign: (songId: string, day: DayOfWeek, slot: string) => void;
  }

  let { unresolved, conflicts, songs, onClose, onManualAssign }: Props = $props();

  let songsMap = $derived(new Map(songs.map(s => [s.id, s])));
  let dayLabels = $derived(DAY_DISPLAY_LABELS[$currentLocale] || DAY_DISPLAY_LABELS.vi);
</script>

<div
  class="modal-overlay"
  onclick={onClose}
  onkeydown={(e) => { if (e.key === 'Escape') onClose(); }}
  role="presentation"
>
  <div
    class="modal-dialog"
    style="max-width: 680px;"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="modal-header">
      <div style="display: flex; align-items: center; gap: 8px;">
        <AlertTriangle size={20} color="var(--warning)" />
        <h3 class="modal-header-title">{$tStore('conflict_modal.title')}</h3>
      </div>
      <button type="button" class="modal-close-btn" onclick={onClose} aria-label={$tStore('conflict_modal.close')}>
        <X size={16} />
      </button>
    </div>

    <div class="modal-body">
      <div style="font-size: 13px; color: var(--text-secondary); line-height: 1.5;">
        {$tStore('conflict_modal.intro')}
      </div>

      <!-- Unresolved Songs Section -->
      {#each unresolved as item (item.songId)}
        {@const song = songsMap.get(item.songId)}
        {@const color = song?.color}

        <div class="conflict-card">
          <div class="conflict-card-header">
            <div style="display: flex; align-items: center; gap: 8px;">
              <div
                style="width: 12px; height: 12px; border-radius: var(--radius-circle); background-color: {color?.border || 'var(--warning)'};"
              ></div>
              <strong style="font-size: 15px; color: var(--text-primary);">
                {item.songName}
              </strong>
              <span
                class="bento-pill"
                style="font-size: 11px; color: var(--danger-text); background: var(--danger-light);"
              >
                {$tStore('conflict_modal.assigned_ratio', { assigned: item.assignedSessions, target: item.targetSessions })}
              </span>
            </div>
          </div>

          <div style="font-size: 12px; color: var(--warning-text); line-height: 1.4;">
            {#each item.reasons as r, i (i)}
              <div>• {r}</div>
            {/each}
          </div>

          <div style="font-size: 12px; font-weight: 700; color: var(--text-primary); margin-top: 4px;">
            {$tStore('conflict_modal.best_suggestions')}
          </div>

          <div style="display: flex; flex-direction: column; gap: 8px;">
            {#each item.candidates.slice(0, 5) as cand, cIdx (cIdx)}
              {@const hasDoubleBook = cand.conflictingMembers.length > 0}
              {@const isAllFree = cand.absentMembers.length === 0}
              {@const dayName = dayLabels[cand.day]?.full || cand.day}

              <div class="candidate-slot-item">
                <div style="display: flex; flex-direction: column; gap: 2px;">
                  <div style="display: flex; align-items: center; gap: 6px;">
                    <strong style="color: var(--text-primary);">{dayName}</strong>
                    <span style="color: var(--text-secondary);">({cand.slot})</span>
                    <span
                      class="bento-pill {isAllFree ? 'is-active' : ''}"
                      style="font-size: 10px; padding: 1px 6px; {isAllFree ? '' : 'color: var(--warning-text); background: var(--warning-light);'}"
                    >
                      {$tStore('conflict_modal.members_free', { available: cand.availableCount, total: cand.totalCount })}
                    </span>
                  </div>

                  <div style="font-size: 11px; color: var(--text-muted);">
                    {#if hasDoubleBook}
                      <span style="color: var(--danger-text);">
                        {$tStore('conflict_modal.conflict_with_song', {
                          members: cand.conflictingMembers.join(', '),
                          songs: cand.conflictingSongs.join(', ')
                        })}
                      </span>
                    {:else if !isAllFree}
                      <span>
                        {$tStore('conflict_modal.absent_by_vote', { members: cand.absentMembers.join(', ') })}
                      </span>
                    {:else}
                      <span style="color: var(--success-text);">
                        {$tStore('conflict_modal.all_free_no_conflict')}
                      </span>
                    {/if}
                  </div>
                </div>

                <button
                  type="button"
                  class="bento-btn {isAllFree ? 'bento-btn-primary' : ''}"
                  style="font-size: 12px; padding: 6px 12px;"
                  onclick={() => onManualAssign(item.songId, cand.day, cand.slot)}
                >
                  <Check size={14} />
                  <span>{$tStore('conflict_modal.choose_slot')}</span>
                </button>
              </div>
            {/each}
          </div>
        </div>
      {/each}

      {#if unresolved.length === 0 && conflicts.length === 0}
        <div style="text-align: center; padding: 24px; color: var(--success-text); font-weight: 600;">
          {$tStore('conflict_modal.no_conflicts_msg')}
        </div>
      {/if}
    </div>

    <div class="modal-footer">
      <button type="button" class="bento-btn bento-btn-primary" onclick={onClose}>
        {$tStore('conflict_modal.close')}
      </button>
    </div>
  </div>
</div>
