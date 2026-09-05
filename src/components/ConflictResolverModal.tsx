import React from 'react';
import { UnresolvedSong, ConflictItem, DayOfWeek, ScheduledSession, SongVoteData } from '../types/timetable';
import { AlertTriangle, Check, X, ArrowRight, UserX, Calendar } from 'lucide-react';

interface ConflictResolverModalProps {
  unresolved: UnresolvedSong[];
  conflicts: ConflictItem[];
  songs: SongVoteData[];
  onClose: () => void;
  onManualAssign: (songId: string, day: DayOfWeek, slot: string) => void;
  onDismissConflict?: (conflictId: string) => void;
}

export const ConflictResolverModal: React.FC<ConflictResolverModalProps> = ({
  unresolved,
  conflicts,
  songs,
  onClose,
  onManualAssign,
}) => {
  const songsMap = React.useMemo(() => new Map(songs.map(s => [s.id, s])), [songs]);

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div
        className="modal-dialog"
        style={{ maxWidth: 680 }}
        onClick={e => e.stopPropagation()}
      >
        <div className="modal-header">
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <AlertTriangle size={20} color="#d97706" />
            <h3 className="modal-header-title">Xử lý xung đột & Cấu hình thủ công</h3>
          </div>
          <button className="modal-close-btn" onClick={onClose}>
            <X size={18} />
          </button>
        </div>

        <div className="modal-body">
          <div style={{ fontSize: 13, color: '#4b5563', lineHeight: 1.5 }}>
            Hệ thống phát hiện một số bài hát chưa thể xếp đủ số buổi tự động vì các thành viên bị trùng lịch ở nhiều bài khác nhau hoặc không có khung giờ 100% rảnh. Dưới đây là các phương án khả thi nhất để bạn chọn thủ công:
          </div>

          {/* Unresolved Songs Section */}
          {unresolved.map(item => {
            const song = songsMap.get(item.songId);
            const color = song?.color;

            return (
              <div key={item.songId} className="conflict-card">
                <div className="conflict-card-header">
                  <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                    <div
                      style={{
                        width: 12,
                        height: 12,
                        borderRadius: 3,
                        backgroundColor: color?.border || '#f59e0b',
                      }}
                    />
                    <strong style={{ fontSize: 15, color: '#1f2937' }}>
                      {item.songName}
                    </strong>
                    <span
                      style={{
                        fontSize: 11,
                        padding: '2px 8px',
                        borderRadius: 12,
                        backgroundColor: '#fee2e2',
                        color: '#b91c1c',
                        fontWeight: 600,
                      }}
                    >
                      Mới xếp {item.assignedSessions}/{item.targetSessions} buổi
                    </span>
                  </div>
                </div>

                <div style={{ fontSize: 12, color: '#b45309' }}>
                  {item.reasons.map((r, i) => (
                    <div key={i}>• {r}</div>
                  ))}
                </div>

                <div style={{ fontSize: 12, fontWeight: 600, color: '#374151', marginTop: 4 }}>
                  Gợi ý khung giờ tốt nhất để xếp bổ sung:
                </div>

                <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
                  {item.candidates.slice(0, 5).map((cand, cIdx) => {
                    const hasDoubleBook = cand.conflictingMembers.length > 0;
                    const isAllFree = cand.absentMembers.length === 0;

                    return (
                      <div key={cIdx} className="candidate-slot-item">
                        <div style={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                          <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                            <strong style={{ color: '#1f2937' }}>{cand.day}</strong>
                            <span>({cand.slot})</span>
                            <span
                              style={{
                                fontSize: 10,
                                padding: '1px 5px',
                                borderRadius: 3,
                                backgroundColor: isAllFree ? '#dcfce7' : '#fef3c7',
                                color: isAllFree ? '#15803d' : '#92400e',
                                fontWeight: 600,
                              }}
                            >
                              {cand.availableCount}/{cand.totalCount} thành viên rảnh
                            </span>
                          </div>

                          <div style={{ fontSize: 11, color: '#6b7280' }}>
                            {hasDoubleBook ? (
                              <span style={{ color: '#b91c1c' }}>
                                ⚠ Trùng {cand.conflictingMembers.join(', ')} với bài {cand.conflictingSongs.join(', ')}
                              </span>
                            ) : cand.absentMembers.length > 0 ? (
                              <span style={{ color: '#d97706' }}>
                                Vắng: {cand.absentMembers.join(', ')}
                              </span>
                            ) : (
                              <span style={{ color: '#15803d' }}>
                                ✓ Khung giờ hoàn hảo (không trùng bài nào khác)
                              </span>
                            )}
                          </div>
                        </div>

                        <button
                          className="btn-gcal-secondary"
                          style={{
                            fontSize: 12,
                            padding: '4px 10px',
                            backgroundColor: '#ffffff',
                            borderColor: hasDoubleBook ? '#fca5a5' : '#86efac',
                            color: hasDoubleBook ? '#b91c1c' : '#15803d',
                          }}
                          onClick={() => onManualAssign(item.songId, cand.day, cand.slot)}
                        >
                          Xếp vào khung này
                        </button>
                      </div>
                    );
                  })}
                </div>
              </div>
            );
          })}

          {/* Active Conflicts List */}
          {conflicts.length > 0 && (
            <div style={{ marginTop: 10 }}>
              <div style={{ fontSize: 14, fontWeight: 600, color: '#1f2937', marginBottom: 8 }}>
                Chi tiết xung đột hiện tại trên lịch ({conflicts.length}):
              </div>

              <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
                {conflicts.map(c => (
                  <div
                    key={c.id}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      gap: 8,
                      padding: '8px 12px',
                      backgroundColor: '#fff1f2',
                      border: '1px solid #fecdd3',
                      borderRadius: 6,
                      fontSize: 12,
                      color: '#9f1239',
                    }}
                  >
                    <AlertTriangle size={14} color="#e11d48" style={{ flexShrink: 0 }} />
                    <span style={{ flex: 1 }}>{c.message}</span>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>

        <div className="modal-footer">
          <button className="btn-gcal-primary" onClick={onClose}>
            Đóng
          </button>
        </div>
      </div>
    </div>
  );
};
