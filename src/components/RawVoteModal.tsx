import React from 'react';
import { SongVoteData, DayOfWeek } from '../types/timetable';
import { DAYS_OF_WEEK, DEFAULT_TIME_SLOTS } from '../constants/timetableDefaults';
import { X, CheckSquare, Square } from 'lucide-react';

interface RawVoteModalProps {
  song: SongVoteData | null;
  onClose: () => void;
  onToggleVote: (songId: string, day: DayOfWeek, slot: string, member: string) => void;
}

export const RawVoteModal: React.FC<RawVoteModalProps> = ({
  song,
  onClose,
  onToggleVote,
}) => {
  if (!song) return null;

  const days = DAYS_OF_WEEK; // Thứ Hai -> Chủ Nhật
  const slots = DEFAULT_TIME_SLOTS;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div
        className="modal-dialog"
        style={{ maxWidth: 880 }}
        onClick={e => e.stopPropagation()}
      >
        <div
          className="modal-header"
          style={{
            backgroundColor: song.color.bg,
            borderBottom: `2px solid ${song.color.border}`,
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <div
              style={{
                width: 14,
                height: 14,
                borderRadius: 3,
                backgroundColor: song.color.border,
              }}
            />
            <h3 className="modal-header-title" style={{ color: song.color.text }}>
              Bảng Vote Chi Tiết: {song.name}
            </h3>
          </div>
          <button className="modal-close-btn" onClick={onClose}>
            <X size={18} />
          </button>
        </div>

        <div className="modal-body" style={{ overflowX: 'auto', padding: 12 }}>
          <div style={{ fontSize: 12, color: '#6b7280', marginBottom: 8 }}>
            Bạn có thể bấm vào các ô checkbox để bật/tắt trạng thái rảnh của từng thành viên trực tiếp:
          </div>

          <table
            style={{
              width: '100%',
              borderCollapse: 'collapse',
              fontSize: 12,
              textAlign: 'center',
            }}
          >
            <thead>
              {/* Sheet Title Row */}
              <tr>
                <th
                  colSpan={3 + song.members.length}
                  style={{
                    padding: '8px',
                    backgroundColor: '#fef3c7',
                    border: '1px solid #d1d5db',
                    fontSize: 13,
                    fontWeight: 700,
                    color: '#1f2937',
                  }}
                >
                  {song.weekTitle || 'VOTE LỊCH TẬP TUẦN'}
                </th>
              </tr>

              {/* Song name row */}
              <tr>
                <th
                  colSpan={2}
                  style={{
                    padding: '6px',
                    backgroundColor: '#ffffff',
                    border: '1px solid #d1d5db',
                    fontWeight: 700,
                    color: '#374151',
                  }}
                >
                  BÀI HÁT
                </th>
                <th
                  colSpan={song.members.length + 1}
                  style={{
                    padding: '6px',
                    backgroundColor: '#ffffff',
                    border: '1px solid #d1d5db',
                    fontWeight: 700,
                    color: song.color.text,
                    fontSize: 13,
                  }}
                >
                  {song.name}
                </th>
              </tr>

              {/* Header row 1 */}
              <tr>
                <th
                  rowSpan={2}
                  style={{
                    padding: '6px',
                    backgroundColor: '#f9fafb',
                    border: '1px solid #d1d5db',
                    width: 90,
                  }}
                >
                  THỨ
                </th>
                <th
                  rowSpan={2}
                  style={{
                    padding: '6px',
                    backgroundColor: '#f9fafb',
                    border: '1px solid #d1d5db',
                    width: 100,
                  }}
                >
                  KHUNG GIỜ
                </th>
                <th
                  colSpan={song.members.length}
                  style={{
                    padding: '6px',
                    backgroundColor: '#f9fafb',
                    border: '1px solid #d1d5db',
                  }}
                >
                  TÊN THÀNH VIÊN
                </th>
                <th
                  rowSpan={2}
                  style={{
                    padding: '6px',
                    backgroundColor: '#f9fafb',
                    border: '1px solid #d1d5db',
                    width: 130,
                  }}
                >
                  GHI CHÚ
                </th>
              </tr>

              {/* Header row 2: Member names */}
              <tr>
                {song.members.map(m => (
                  <th
                    key={m}
                    style={{
                      padding: '6px 8px',
                      backgroundColor: '#f9fafb',
                      border: '1px solid #d1d5db',
                      fontWeight: 600,
                      color: '#1f2937',
                    }}
                  >
                    {m}
                  </th>
                ))}
              </tr>
            </thead>

            <tbody>
              {days.map((day, dayIdx) => {
                const isEvenDay = dayIdx % 2 === 0;
                const rowBg = isEvenDay ? '#ffedd5' : '#ffffff'; // Match user reference screenshot tint

                return slots.map((slot, slotIdx) => {
                  const noteKey = `${day}__${slot}`;
                  const note = song.notes[noteKey] || '';

                  return (
                    <tr key={`${day}_${slot}`} style={{ backgroundColor: rowBg }}>
                      {/* Only render Day in first slot of the day (rowSpan) */}
                      {slotIdx === 0 && (
                        <td
                          rowSpan={slots.length}
                          style={{
                            border: '1px solid #d1d5db',
                            fontWeight: 700,
                            color: '#374151',
                            verticalAlign: 'middle',
                            backgroundColor: rowBg,
                          }}
                        >
                          {day}
                        </td>
                      )}

                      <td
                        style={{
                          border: '1px solid #d1d5db',
                          padding: '4px',
                          color: '#4b5563',
                        }}
                      >
                        {slot}
                      </td>

                      {/* Member vote checkboxes */}
                      {song.members.map(member => {
                        const key = `${day}__${slot}__${member}`;
                        const isChecked = !!song.availability[key];

                        return (
                          <td
                            key={member}
                            style={{
                              border: '1px solid #d1d5db',
                              padding: '6px',
                              cursor: 'pointer',
                              backgroundColor: isChecked ? '#bbf7d0' : 'transparent',
                            }}
                            onClick={() => onToggleVote(song.id, day, slot, member)}
                            title={`Bấm để chuyển trạng thái rảnh của ${member}`}
                          >
                            {isChecked ? (
                              <CheckSquare size={16} color="#15803d" />
                            ) : (
                              <Square size={16} color="#9ca3af" />
                            )}
                          </td>
                        );
                      })}

                      <td
                        style={{
                          border: '1px solid #d1d5db',
                          padding: '4px 6px',
                          fontSize: 11,
                          color: '#6b7280',
                          textAlign: 'left',
                        }}
                      >
                        {note}
                      </td>
                    </tr>
                  );
                });
              })}
            </tbody>
          </table>
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
