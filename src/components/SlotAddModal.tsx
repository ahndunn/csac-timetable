import React from 'react';
import { SongVoteData, ScheduledSession, DayOfWeek } from '../types/timetable';
import { getSlotAttendance } from '../services/scheduler';
import { Plus, X, Check, AlertCircle } from 'lucide-react';

interface SlotAddModalProps {
  day: DayOfWeek | null;
  slot: string | null;
  songs: SongVoteData[];
  schedule: ScheduledSession[];
  onClose: () => void;
  onAssignSong: (song: SongVoteData, day: DayOfWeek, slot: string) => void;
}

export const SlotAddModal: React.FC<SlotAddModalProps> = ({
  day,
  slot,
  songs,
  schedule,
  onClose,
  onAssignSong,
}) => {
  if (!day || !slot) return null;

  // Find other songs in this slot
  const existingInSlot = schedule.filter(s => s.day === day && s.slot === slot);

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-dialog" onClick={e => e.stopPropagation()}>
        <div className="modal-header">
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <Plus size={18} color="#1a73e8" />
            <h3 className="modal-header-title">
              Thêm bài tập vào {day} ({slot})
            </h3>
          </div>
          <button className="modal-close-btn" onClick={onClose}>
            <X size={18} />
          </button>
        </div>

        <div className="modal-body">
          <div style={{ fontSize: 13, color: '#4b5563' }}>
            Chọn bài hát bạn muốn phân bổ vào khung giờ này. Danh sách hiển thị tỷ lệ thành viên rảnh và cảnh báo trùng lịch:
          </div>

          <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
            {songs.map(song => {
              const attendance = getSlotAttendance(song, day, slot);

              // Check if any member of this song is already in an existing song in this slot
              const doubleBookedMembers: string[] = [];
              for (const existing of existingInSlot) {
                for (const m of song.members) {
                  if (existing.allMembers.includes(m)) {
                    doubleBookedMembers.push(`${m} (đã ở bài ${existing.songName})`);
                  }
                }
              }

              const isAlreadyInSlot = existingInSlot.some(s => s.songId === song.id);

              return (
                <div
                  key={song.id}
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    padding: '10px 12px',
                    border: '1px solid #e5e7eb',
                    borderRadius: 8,
                    backgroundColor: isAlreadyInSlot ? '#f3f4f6' : '#ffffff',
                  }}
                >
                  <div style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                      <div
                        style={{
                          width: 12,
                          height: 12,
                          borderRadius: 3,
                          backgroundColor: song.color.border,
                        }}
                      />
                      <strong style={{ fontSize: 14, color: '#1f2937' }}>
                        {song.name}
                      </strong>
                      <span
                        style={{
                          fontSize: 11,
                          padding: '1px 6px',
                          borderRadius: 4,
                          backgroundColor: attendance.is100Percent ? '#dcfce7' : '#fee2e2',
                          color: attendance.is100Percent ? '#15803d' : '#b91c1c',
                          fontWeight: 600,
                        }}
                      >
                        {attendance.availableMembers.length}/{song.members.length} thành viên rảnh
                      </span>
                    </div>

                    {doubleBookedMembers.length > 0 && (
                      <div style={{ fontSize: 11, color: '#dc2626', display: 'flex', alignItems: 'center', gap: 4 }}>
                        <AlertCircle size={12} />
                        <span>Trùng thành viên: {doubleBookedMembers.join(', ')}</span>
                      </div>
                    )}

                    {attendance.absentMembers.length > 0 && doubleBookedMembers.length === 0 && (
                      <div style={{ fontSize: 11, color: '#d97706' }}>
                        Vắng theo vote: {attendance.absentMembers.join(', ')}
                      </div>
                    )}
                  </div>

                  <button
                    className="btn-gcal-secondary"
                    disabled={isAlreadyInSlot}
                    onClick={() => {
                      onAssignSong(song, day, slot);
                      onClose();
                    }}
                    style={{
                      fontSize: 12,
                      padding: '5px 12px',
                      backgroundColor: isAlreadyInSlot ? '#e5e7eb' : '#ffffff',
                    }}
                  >
                    {isAlreadyInSlot ? 'Đã xếp ở ô này' : 'Xếp vào ô này'}
                  </button>
                </div>
              );
            })}
          </div>
        </div>

        <div className="modal-footer">
          <button className="btn-gcal-secondary" onClick={onClose}>
            Đóng
          </button>
        </div>
      </div>
    </div>
  );
};
