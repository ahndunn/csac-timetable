import React from 'react';
import { ScheduledSession, SongVoteData, DayOfWeek, ConflictItem, UnresolvedSong } from '../types/timetable';
import { DAYS_OF_WEEK, DAY_SHORT_LABELS, DEFAULT_TIME_SLOTS } from '../constants/timetableDefaults';
import { AlertTriangle, CheckCircle, Plus, Info, Upload, Sparkles, FileSpreadsheet } from 'lucide-react';
import { getWeekDays, isSameDay } from '../utils/dateUtils';

interface CalendarGridProps {
  schedule: ScheduledSession[];
  songs: SongVoteData[];
  conflicts: ConflictItem[];
  unresolved: UnresolvedSong[];
  selectedMember: string | null;
  onSelectSession: (session: ScheduledSession) => void;
  onOpenSlotAdd: (day: DayOfWeek, slot: string) => void;
  onOpenConflictResolver: () => void;
  onLoadSample?: () => void;
  onOpenUpload?: () => void;
  onDownloadTemplate?: () => void;
  selectedWeekStart: Date;
  days?: DayOfWeek[];
  slots?: string[];
}

export const CalendarGrid: React.FC<CalendarGridProps> = ({
  schedule,
  songs,
  conflicts,
  unresolved,
  selectedMember,
  onSelectSession,
  onOpenSlotAdd,
  onOpenConflictResolver,
  onLoadSample,
  onOpenUpload,
  onDownloadTemplate,
  selectedWeekStart,
  days = DAYS_OF_WEEK,
  slots = DEFAULT_TIME_SLOTS,
}) => {
  // Compute dynamic dates for the selected week
  const weekDays = React.useMemo(() => getWeekDays(selectedWeekStart), [selectedWeekStart]);

  // Check if a slot has a conflict
  const getSlotConflicts = (day: DayOfWeek, slot: string) => {
    return conflicts.filter(c => c.day === day && c.slot === slot);
  };

  const totalSessionsRequested = songs.reduce((acc, s) => acc + s.targetSessions, 0);
  const totalSessionsScheduled = schedule.length;
  const hasUnresolved = unresolved.length > 0;
  const hasMemberConflict = conflicts.some(c => c.type === 'member_double_booked');

  return (
    <main className="calendar-main">
      {/* Top Status & Conflict Alert Banner */}
      <div
        className={`status-banner ${
          hasUnresolved || hasMemberConflict
            ? 'has-conflict'
            : totalSessionsScheduled > 0
            ? 'all-good'
            : ''
        }`}
      >
        <div className="status-left">
          {hasUnresolved || hasMemberConflict ? (
            <AlertTriangle size={18} color="#b45309" />
          ) : totalSessionsScheduled > 0 ? (
            <CheckCircle size={18} color="#16a34a" />
          ) : (
            <Info size={18} color="#4b5563" />
          )}

          <span>
            {songs.length === 0 ? (
              'Chưa có dữ liệu. Vui lòng bấm "Dữ liệu mẫu (5 bài)" hoặc "Tải file Excel" để bắt đầu.'
            ) : totalSessionsScheduled === 0 ? (
              `Đã tải ${songs.length} bài hát (yêu cầu tổng cộng ${totalSessionsRequested} buổi tập). Hãy bấm "Tự động xếp lịch".`
            ) : hasUnresolved || hasMemberConflict ? (
              <>
                Đã xếp <strong>{totalSessionsScheduled}/{totalSessionsRequested}</strong> buổi.
                {hasUnresolved && ` Còn ${unresolved.length} bài chưa xếp đủ số buổi do xung đột.`}
                {hasMemberConflict && ` Có xung đột trùng giờ thành viên!`}
              </>
            ) : (
              <>
                Hoàn thành hoàn hảo! Đã xếp đủ <strong>{totalSessionsScheduled}/{totalSessionsRequested}</strong> buổi tập cho {songs.length} bài hát không trùng giờ bất kỳ thành viên nào.
              </>
            )}
          </span>
        </div>

        {(hasUnresolved || hasMemberConflict) && (
          <button className="status-btn-fix" onClick={onOpenConflictResolver}>
            Xem chi tiết & Xử lý xung đột ({unresolved.length + conflicts.length})
          </button>
        )}
      </div>

      {/* Empty State Banner when no songs loaded */}
      {songs.length === 0 && (
        <div
          style={{
            padding: '24px 20px',
            backgroundColor: '#f8fafd',
            borderBottom: '1px solid #e2e8f0',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            justifyContent: 'center',
            gap: 12,
            textAlign: 'center',
          }}
        >
          <div style={{ fontSize: 16, fontWeight: 600, color: '#1e293b' }}>
            Chưa có bài hát nào được nạp vào hệ thống
          </div>
          <div style={{ fontSize: 13, color: '#64748b', maxWidth: 620, lineHeight: 1.5 }}>
            Bạn có thể tải lên các file Excel vote lịch của nhóm, tải về file Excel dữ liệu mẫu của 5 bài hát để xem thử, hoặc bấm "Nạp dữ liệu mẫu" để trải nghiệm xếp lịch ngay.
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: 10, flexWrap: 'wrap', justifyContent: 'center', marginTop: 4 }}>
            {onOpenUpload && (
              <button className="btn-gcal-primary" onClick={onOpenUpload}>
                <Upload size={15} />
                <span>Tải file Excel lên</span>
              </button>
            )}
            {onLoadSample && (
              <button className="btn-gcal-sample" onClick={onLoadSample}>
                <Sparkles size={15} />
                <span>Nạp dữ liệu mẫu (5 bài)</span>
              </button>
            )}
            {onDownloadTemplate && (
              <button className="btn-gcal-secondary" onClick={onDownloadTemplate}>
                <FileSpreadsheet size={15} />
                <span>Tải về file Excel dữ liệu mẫu (.xlsx)</span>
              </button>
            )}
          </div>
        </div>
      )}

      {/* Google Calendar Viewport */}
      <div className="calendar-viewport">
        <table className="gcal-table">
          <thead className="gcal-thead">
            <tr>
              <th className="gcal-time-col-header">GMT+7</th>
              {days.map((day, idx) => {
                const dayDate = weekDays[idx];
                const dateNum = dayDate ? dayDate.getDate() : idx + 1;
                const isToday = dayDate ? isSameDay(dayDate, new Date()) : false;

                return (
                  <th
                    key={day}
                    className={`gcal-day-header ${isToday ? 'today' : ''}`}
                  >
                    <div className="day-header-title">{DAY_SHORT_LABELS[day]}</div>
                    <div className="day-header-number">{dateNum}</div>
                  </th>
                );
              })}
            </tr>
          </thead>

          <tbody>
            {slots.map(slot => (
              <tr key={slot}>
                {/* Time Gutter */}
                <td className="gcal-time-cell">{slot}</td>

                {/* Day Columns */}
                {days.map(day => {
                  const sessionsInSlot = schedule.filter(
                    s => s.day === day && s.slot === slot
                  );
                  const slotConflicts = getSlotConflicts(day, slot);
                  const containsSelectedMember = selectedMember
                    ? sessionsInSlot.some(s => s.allMembers.includes(selectedMember))
                    : false;

                  return (
                    <td
                      key={`${day}__${slot}`}
                      className={`gcal-slot-cell ${
                        containsSelectedMember ? 'highlight-member' : ''
                      }`}
                    >
                      <div className="slot-events-container">
                        {/* Conflict notification in cell if any */}
                        {slotConflicts.length > 0 && (
                          <div
                            className="event-conflict-tag"
                            title={slotConflicts.map(c => c.message).join('\n')}
                          >
                            ⚠ Trùng lịch thành viên!
                          </div>
                        )}

                        {/* Scheduled Event Cards in Pastel Style */}
                        {sessionsInSlot.map(sess => {
                          const isMemberInThisSong = selectedMember
                            ? sess.allMembers.includes(selectedMember)
                            : true;
                          const isDimmed = selectedMember && !isMemberInThisSong;
                          const isPerfect = sess.absentMembers.length === 0;

                          return (
                            <div
                              key={sess.id}
                              className="event-card"
                              style={{
                                backgroundColor: sess.color.bg,
                                borderLeftColor: sess.color.border,
                                opacity: isDimmed ? 0.35 : 1,
                              }}
                              onClick={() => onSelectSession(sess)}
                              title="Bấm để xem chi tiết buổi tập"
                            >
                              <div className="event-card-top">
                                <span
                                  className="event-song-name"
                                  style={{ color: sess.color.text }}
                                >
                                  {sess.songName}
                                </span>
                                <span className="event-room-pill">P.{sess.room}</span>
                              </div>

                              <div
                                className={`event-attendance-badge ${
                                  isPerfect ? 'perfect' : 'partial'
                                }`}
                              >
                                {isPerfect ? (
                                  <>✓ {sess.allMembers.length}/{sess.allMembers.length} có mặt</>
                                ) : (
                                  <>⚠ Vắng: {sess.absentMembers.join(', ')}</>
                                )}
                              </div>

                              <div className="event-members-list">
                                {sess.allMembers.join(', ')}
                              </div>
                            </div>
                          );
                        })}

                        {/* Quick add button on hover */}
                        <button
                          className="slot-empty-add-btn"
                          onClick={() => onOpenSlotAdd(day, slot)}
                          title="Thêm bài tập vào khung giờ này"
                        >
                          <Plus size={14} />
                          <span>Thêm bài</span>
                        </button>
                      </div>
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </main>
  );
};
