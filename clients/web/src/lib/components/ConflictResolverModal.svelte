<script lang="ts">
  import type { UnresolvedSong, ConflictItem, DayOfWeek, SongVoteData } from '../types/timetable';
  import { AlertTriangle, Check, X, ArrowRight, UserX, Calendar } from '@lucide/svelte';

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

<div class="modal-overlay" onclick={onClose} role="presentation">
  <div
    class="modal-dialog"
    style="max-width: 680px;"
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
  >
    <div class="modal-header">
      <div style="display: flex; align-items: center; gap: 8px;">
        <AlertTriangle size={20} color="#d97706" />
        <h3 class="modal-header-title">Xử lý xung đột & Cấu hình thủ công</h3>
      </div>
      <button type="button" class="modal-close-btn" onclick={onClose} aria-label="Đóng">
        <X size={18} />
      </button>
    </div>

    <div class="modal-body">
      <div style="font-size: 13px; color: #4b5563; line-height: 1.5;">
        Hệ thống phát hiện một số bài hát chưa thể xếp đủ số buổi tự động vì các thành viên bị trùng lịch hoặc không có khung giờ 100% rảnh. Dưới đây là các phương án khả thi nhất để bạn chọn thủ công:
      </div>

      {#each unresolved as item (item.songId)}
        {@const song = songsMap.get(item.songId)}
        {@const color = song?.color}

        <div class="conflict-card">
          <div class="conflict-card-header">
            <div style="display: flex; align-items: center; gap: 8px;">
              <div
                style="width: 12px; height: 12px; border-radius: 3px; background-color: {color?.border || '#f59e0b'};"
              ></div>
              <strong style="font-size: 15px; color: #1f2937;">
                {item.songName}
              </strong>
              <span
                style="font-size: 11px; padding: 2px 8px; border-radius: 12px; background-color: #fee2e2; color: #b91c1c; fontWeight: 600;"
              >
                Mới xếp {item.assignedSessions}/{item.targetSessions} buổi
              </span>
            </div>
          </div>

          <div style="font-size: 12px; color: #b45309;">
            {#each item.reasons as r}
              <div>⚠ {r}</div>
            {/each}
          </div>

          <div style="font-size: 13px; font-weight: 600; color: #374151; margin-top: 4px;">
            Gợi ý khung giờ tốt nhất (sắp xếp theo độ khả thi):
          </div>

          <div class="candidate-slots-list">
            {#each item.candidates as cand}
              {@const is100 = cand.absentMembers.length === 0}
              <div class="candidate-slot-item">
                <div class="cand-left">
                  <span class="cand-time">
                    <Calendar size={13} />
                    <strong>{cand.day}</strong>, {cand.slot}
                  </span>

                  <span class="cand-attendance {is100 ? 'perfect' : 'partial'}">
                    👥 {cand.availableCount}/{cand.totalCount} người
                    {#if !is100}
                      (Vắng {cand.absentMembers.join(', ')})
                    {/if}
                  </span>

                  {#if cand.conflictingSongs.length > 0}
                    <span class="cand-conflict-warn">
                      ⚠ Trùng với: {cand.conflictingSongs.join(', ')} ({cand.conflictingMembers.join(', ')})
                    </span>
                  {/if}
                </div>

                <button
                  type="button"
                  class="btn-assign-slot"
                  onclick={() => onManualAssign(item.songId, cand.day, cand.slot)}
                >
                  <span>Chọn slot này</span>
                  <ArrowRight size={13} />
                </button>
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>

    <div class="modal-footer">
      <button type="button" class="btn-gcal-primary" onclick={onClose}>
        Đóng
      </button>
    </div>
  </div>
</div>
