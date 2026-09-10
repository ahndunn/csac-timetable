<script lang="ts">
  import type { ScheduledSession, SongVoteData } from '../types/timetable';
  import { Clock, MapPin, Users, FileText, Trash2, X, Check, AlertCircle } from '@lucide/svelte';

  interface Props {
    session: ScheduledSession | null;
    songData?: SongVoteData;
    onClose: () => void;
    onDeleteSession: (sessionId: string) => void;
  }

  let { session, songData, onClose, onDeleteSession }: Props = $props();

  let isPerfect = $derived(session ? session.absentMembers.length === 0 : true);
</script>

{#if session}
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
        <div style="display: flex; align-items: center; gap: 10px;">
          <div
            style="width: 14px; height: 14px; border-radius: var(--radius-circle); background-color: {session.color.border};"
          ></div>
          <h3 class="modal-header-title">
            {session.songName}
          </h3>
        </div>
        <button type="button" class="modal-close-btn" onclick={onClose} aria-label="Đóng">
          <X size={16} />
        </button>
      </div>

      <div class="modal-body">
        <div style="display: flex; flex-direction: column; gap: 10px;">
          <div style="display: flex; align-items: center; gap: 10px; font-size: 13px; color: var(--text-primary);">
            <Clock size={16} color="var(--accent)" />
            <span>
              <strong>{session.day}</strong>, {session.slot}
            </span>
          </div>

          <div style="display: flex; align-items: center; gap: 10px; font-size: 13px; color: var(--text-primary);">
            <MapPin size={16} color="var(--accent)" />
            <span>Phòng tập {session.room}</span>
          </div>
        </div>

        <div class="bento-card" style="padding: 14px;">
          <div
            style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 10px;"
          >
            <span
              style="font-size: 13px; font-weight: 700; color: var(--text-primary); display: flex; align-items: center; gap: 6px;"
            >
              <Users size={15} color="var(--accent)" />
              <span>Thành viên ({session.availableMembers.length}/{session.allMembers.length})</span>
            </span>

            <span
              class="bento-pill {isPerfect ? 'is-accent' : ''}"
              style="{isPerfect ? '' : 'color: var(--danger-text); background: var(--danger-light);'}"
            >
              {isPerfect ? 'Đầy đủ 100%' : `Vắng ${session.absentMembers.length} người`}
            </span>
          </div>

          <div
            style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px; padding: 12px; background: var(--surface-card-subtle); border: 1px solid var(--border-card); border-radius: var(--radius-sm);"
          >
            {#each session.allMembers as m}
              {@const isAvail = session.availableMembers.includes(m)}
              <div
                style="display: flex; align-items: center; gap: 6px; font-size: 13px; color: {isAvail ? 'var(--text-primary)' : 'var(--text-muted)'};"
              >
                {#if isAvail}
                  <Check size={14} color="var(--success)" />
                {:else}
                  <AlertCircle size={14} color="var(--danger)" />
                {/if}
                <span style="text-decoration: {isAvail ? 'none' : 'line-through'}; font-weight: 500;">
                  {m}
                </span>
                {#if !isAvail}
                  <span style="font-size: 10px; color: var(--danger); font-weight: 600;">
                    (Bận)
                  </span>
                {/if}
              </div>
            {/each}
          </div>
        </div>

        {#if session.note}
          <div style="padding: 12px; background: var(--surface-card-subtle); border: 1px solid var(--border-card); border-radius: var(--radius-sm);">
            <div
              style="font-size: 12px; font-weight: 700; color: var(--text-secondary); display: flex; align-items: center; gap: 6px; margin-bottom: 4px;"
            >
              <FileText size={14} color="var(--accent)" />
              <span>Ghi chú từ bảng vote:</span>
            </div>
            <div style="font-size: 12px; color: var(--text-primary);">
              {session.note}
            </div>
          </div>
        {/if}
      </div>

      <div class="modal-footer">
        <button
          type="button"
          class="bento-btn bento-btn-danger"
          onclick={() => onDeleteSession(session.id)}
        >
          <Trash2 size={15} />
          <span>Xóa buổi này</span>
        </button>
        <button type="button" class="bento-btn bento-btn-primary" onclick={onClose}>
          Đóng
        </button>
      </div>
    </div>
  </div>
{/if}
