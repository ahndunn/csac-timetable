<script lang="ts">
  import type { UnresolvedSong, ConflictItem, DayOfWeek, SongVoteData } from '../types/timetable';
  import { AlertTriangle, X, Check } from '@lucide/svelte';

  interface Props {
    unresolved: UnresolvedSong[];
    conflicts: ConflictItem[];
    songs: SongVoteData[];
    onClose: () => void;
    onManualAssign: (songId: string, day: DayOfWeek, slot: string) => void;
  }

  let { unresolved, conflicts, songs, onClose, onManualAssign }: Props = $props();

  let songsMap = $derived(new Map(songs.map(s => [s.id, s])));
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
        <h3 class="modal-header-title">Xử lý xung đột & Cấu hình thủ công</h3>
      </div>
      <button type="button" class="modal-close-btn" onclick={onClose} aria-label="Đóng">
        <X size={16} />
      </button>
    </div>

    <div class="modal-body">
      <div style="font-size: 13px; color: var(--text-secondary); line-height: 1.5;">
        Hệ thống phát hiện một số bài hát chưa thể xếp đủ số buổi tự động vì các thành viên bị trùng lịch ở nhiều bài khác nhau hoặc không có khung giờ 100% rảnh. Dưới đây là các phương án khả thi nhất để bạn chọn thủ công:
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
                Mới xếp {item.assignedSessions}/{item.targetSessions} buổi
              </span>
            </div>
          </div>

          <div style="font-size: 12px; color: var(--warning-text); line-height: 1.4;">
            {#each item.reasons as r, i (i)}
              <div>• {r}</div>
            {/each}
          </div>

          <div style="font-size: 12px; font-weight: 700; color: var(--text-primary); margin-top: 4px;">
            Gợi ý khung giờ tốt nhất để xếp bổ sung:
          </div>

          <div style="display: flex; flex-direction: column; gap: 8px;">
            {#each item.candidates.slice(0, 5) as cand, cIdx (cIdx)}
              {@const hasDoubleBook = cand.conflictingMembers.length > 0}
              {@const isAllFree = cand.absentMembers.length === 0}

              <div class="candidate-slot-item">
                <div style="display: flex; flex-direction: column; gap: 2px;">
                  <div style="display: flex; align-items: center; gap: 6px;">
                    <strong style="color: var(--text-primary);">{cand.day}</strong>
                    <span style="color: var(--text-secondary);">({cand.slot})</span>
                    <span
                      class="bento-pill {isAllFree ? 'is-active' : ''}"
                      style="font-size: 10px; padding: 1px 6px; {isAllFree ? '' : 'color: var(--warning-text); background: var(--warning-light);'}"
                    >
                      {cand.availableCount}/{cand.totalCount} thành viên rảnh
                    </span>
                  </div>

                  <div style="font-size: 11px; color: var(--text-muted);">
                    {#if hasDoubleBook}
                      <span style="color: var(--danger-text);">
                        ⚠ Trùng {cand.conflictingMembers.join(', ')} với bài {cand.conflictingSongs.join(', ')}
                      </span>
                    {:else if !isAllFree}
                      <span>
                        Vắng theo bảng vote: {cand.absentMembers.join(', ')}
                      </span>
                    {:else}
                      <span style="color: var(--success-text);">
                        ✓ Tất cả thành viên đều rảnh và chưa có lịch trùng
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
                  <span>Chọn ô này</span>
                </button>
              </div>
            {/each}
          </div>
        </div>
      {/each}

      {#if unresolved.length === 0 && conflicts.length === 0}
        <div style="text-align: center; padding: 24px; color: var(--success-text); font-weight: 600;">
          ✓ Không có xung đột hoặc bài hát nào bị thiếu buổi!
        </div>
      {/if}
    </div>

    <div class="modal-footer">
      <button type="button" class="bento-btn bento-btn-primary" onclick={onClose}>
        Đóng
      </button>
    </div>
  </div>
</div>
