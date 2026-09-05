import React from 'react';
import { ScheduledSession, SongVoteData } from '../types/timetable';
import { Clock, MapPin, Users, FileText, Trash2, X, Check, AlertCircle } from 'lucide-react';

interface EventDetailModalProps {
  session: ScheduledSession | null;
  songData?: SongVoteData;
  onClose: () => void;
  onDeleteSession: (sessionId: string) => void;
}

export const EventDetailModal: React.FC<EventDetailModalProps> = ({
  session,
  songData,
  onClose,
  onDeleteSession,
}) => {
  if (!session) return null;

  const isPerfect = session.absentMembers.length === 0;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-dialog" onClick={e => e.stopPropagation()}>
        <div
          className="modal-header"
          style={{
            backgroundColor: session.color.bg,
            borderBottom: `2px solid ${session.color.border}`,
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
            <div
              style={{
                width: 14,
                height: 14,
                borderRadius: 3,
                backgroundColor: session.color.border,
              }}
            />
            <h3
              className="modal-header-title"
              style={{ color: session.color.text, fontWeight: 700 }}
            >
              {session.songName}
            </h3>
          </div>
          <button className="modal-close-btn" onClick={onClose}>
            <X size={18} />
          </button>
        </div>

        <div className="modal-body">
          {/* Time & Place info */}
          <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 10, fontSize: 14, color: '#374151' }}>
              <Clock size={16} color="#6b7280" />
              <span>
                <strong>{session.day}</strong>, {session.slot}
              </span>
            </div>

            <div style={{ display: 'flex', alignItems: 'center', gap: 10, fontSize: 14, color: '#374151' }}>
              <MapPin size={16} color="#6b7280" />
              <span>Phòng tập {session.room}</span>
            </div>
          </div>

          <hr style={{ border: 'none', borderTop: '1px solid #e5e7eb' }} />

          {/* Attendance Status */}
          <div>
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                marginBottom: 8,
              }}
            >
              <span
                style={{
                  fontSize: 13,
                  fontWeight: 600,
                  color: '#4b5563',
                  display: 'flex',
                  alignItems: 'center',
                  gap: 6,
                }}
              >
                <Users size={15} />
                <span>Thành viên tham gia ({session.availableMembers.length}/{session.allMembers.length})</span>
              </span>

              <span
                style={{
                  fontSize: 12,
                  fontWeight: 600,
                  padding: '2px 8px',
                  borderRadius: 12,
                  backgroundColor: isPerfect ? '#dcfce7' : '#fee2e2',
                  color: isPerfect ? '#15803d' : '#b91c1c',
                }}
              >
                {isPerfect ? 'Đầy đủ 100%' : `Vắng ${session.absentMembers.length} người`}
              </span>
            </div>

            <div
              style={{
                display: 'grid',
                gridTemplateColumns: 'repeat(2, 1fr)',
                gap: 6,
                backgroundColor: '#f9fafb',
                padding: 10,
                borderRadius: 6,
              }}
            >
              {session.allMembers.map(m => {
                const isAvail = session.availableMembers.includes(m);
                return (
                  <div
                    key={m}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      gap: 6,
                      fontSize: 13,
                      color: isAvail ? '#1f2937' : '#9ca3af',
                    }}
                  >
                    {isAvail ? (
                      <Check size={14} color="#16a34a" />
                    ) : (
                      <AlertCircle size={14} color="#dc2626" />
                    )}
                    <span style={{ textDecoration: isAvail ? 'none' : 'line-through' }}>
                      {m}
                    </span>
                    {!isAvail && (
                      <span style={{ fontSize: 10, color: '#dc2626', fontWeight: 500 }}>
                        (Bận)
                      </span>
                    )}
                  </div>
                );
              })}
            </div>
          </div>

          {/* Notes if present */}
          {session.note && (
            <div>
              <div
                style={{
                  fontSize: 13,
                  fontWeight: 600,
                  color: '#4b5563',
                  display: 'flex',
                  alignItems: 'center',
                  gap: 6,
                  marginBottom: 6,
                }}
              >
                <FileText size={15} />
                <span>Ghi chú từ bảng vote:</span>
              </div>
              <div
                style={{
                  fontSize: 13,
                  backgroundColor: '#fefce8',
                  border: '1px solid #fef08a',
                  padding: '8px 12px',
                  borderRadius: 6,
                  color: '#854d0e',
                }}
              >
                {session.note}
              </div>
            </div>
          )}
        </div>

        <div className="modal-footer">
          <button
            className="btn-gcal-secondary"
            onClick={() => onDeleteSession(session.id)}
            style={{
              color: '#dc2626',
              borderColor: '#fca5a5',
              display: 'flex',
              alignItems: 'center',
              gap: 6,
            }}
          >
            <Trash2 size={15} />
            <span>Xóa buổi này</span>
          </button>
          <button className="btn-gcal-primary" onClick={onClose}>
            Đóng
          </button>
        </div>
      </div>
    </div>
  );
};
