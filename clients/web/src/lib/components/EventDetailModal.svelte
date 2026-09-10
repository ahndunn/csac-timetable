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
  <div class="modal-overlay" onclick={onClose} role="presentation">
    <div class="modal-dialog" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true">
      <div
        class="modal-header"
        style="background-color: {session.color.bg}; border-bottom: 2px solid {session.color.border};"
      >
        <div style="display: flex; align-items: center; gap: 10px;">
          <div
            style="width: 14px; height: 14px; border-radius: 3px; background-color: {session.color.border};"
          ></div>
          <h3
            class="modal-header-title"
            style="color: {session.color.text}; font-weight: 700;"
          >
            {session.songName}
          </h3>
        </div>
        <button type="button" class="modal-close-btn" onclick={onClose} aria-label="Đóng">
          <X size={18} />
        </button>
      </div>

      <div class="modal-body">
        <div style="display: flex; flex-direction: column; gap: 10px;">
          <div style="display: flex; align-items: center; gap: 10px; font-size: 14px; color: #374151;">
            <Clock size={16} color="#6b7280" />
            <span>
              <strong>{session.day}</strong>, {session.slot}
            </span>
          </div>

          <div style="display: flex; align-items: center; gap: 10px; font-size: 14px; color: #374151;">
            <MapPin size={16} color="#6b7280" />
            <span>Phòng tập {session.room}</span>
          </div>
        </div>

        <hr style="border: none; border-top: 1px solid #e5e7eb;" />

        <div>
          <div
            style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px;"
          >
            <span
              style="font-size: 13px; font-weight: 600; color: #4b5563; display: flex; align-items: center; gap: 6px;"
            >
              <Users size={15} />
              <span>Thành viên ({session.availableMembers.length}/{session.allMembers.length})</span>
            </span>

            <span
              style="font-size: 12px; font-weight: 600; padding: 2px 8px; border-radius: 12px; background-color: {isPerfect ? '#dcfce7' : '#fee2e2'}; color: {isPerfect ? '#15803d' : '#b91c1c'};"
            >
              {isPerfect ? 'Đầy đủ 100%' : `Vắng ${session.absentMembers.length} người`}
            </span>
          </div>

          <div
            style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 6px; background-color: #f9fafb; padding: 10px; border-radius: 6px;"
          >
            {#each session.allMembers as m}
              {@const isAvail = session.availableMembers.includes(m)}
              <div
                style="display: flex; align-items: center; gap: 6px; font-size: 13px; color: {isAvail ? '#1f2937' : '#9ca3af'};"
              >
                {#if isAvail}
                  <Check size={14} color="#16a34a" />
                {:else}
                  <AlertCircle size={14} color="#dc2626" />
                {/if}
                <span style="text-decoration: {isAvail ? 'none' : 'line-through'};">
                  {m}
                </span>
                {#if !isAvail}
                  <span style="font-size: 10px; color: #dc2626; font-weight: 500;">
                    (Bận)
                  </span>
                {/if}
              </div>
            {/each}
          </div>
        </div>

        {#if session.note}
          <div>
            <div
              style="font-size: 13px; font-weight: 600; color: #4b5563; display: flex; align-items: center; gap: 6px; margin-bottom: 6px;"
            >
              <FileText size={15} />
              <span>Ghi chú từ bảng vote:</span>
            </div>
            <div
              style="font-size: 13px; background-color: #fefce8; border: 1px solid #fef08a; padding: 8px 12px; border-radius: 6px; color: #854d0e;"
            >
              {session.note}
            </div>
          </div>
        {/if}
      </div>

      <div class="modal-footer">
        <button
          type="button"
          class="btn-gcal-secondary"
          onclick={() => onDeleteSession(session.id)}
          style="color: #dc2626; border-color: #fca5a5; display: flex; align-items: center; gap: 6px;"
        >
          <Trash2 size={15} />
          <span>Xóa buổi này</span>
        </button>
        <button type="button" class="btn-gcal-primary" onclick={onClose}>
          Đóng
        </button>
      </div>
    </div>
  </div>
{/if}
