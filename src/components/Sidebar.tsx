import React, { useState } from 'react';
import { SongVoteData, SolverSettings, ScheduledSession } from '../types/timetable';
import { Music, Users, Sliders, Eye } from 'lucide-react';
import { getMonthMatrix, isSameWeek, formatWeekRange, getMonday } from '../utils/dateUtils';

interface SidebarProps {
  songs: SongVoteData[];
  schedule: ScheduledSession[];
  selectedMember: string | null;
  onSelectMember: (member: string | null) => void;
  onUpdateSongSessions: (songId: string, newTarget: number) => void;
  onViewSongVotes: (song: SongVoteData) => void;
  onDeleteSong: (songId: string) => void;
  settings: SolverSettings;
  onUpdateSettings: (newSettings: SolverSettings) => void;
  selectedWeekStart: Date;
  onSelectWeek: (monday: Date) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({
  songs,
  schedule,
  selectedMember,
  onSelectMember,
  onUpdateSongSessions,
  onViewSongVotes,
  onDeleteSong,
  settings,
  onUpdateSettings,
  selectedWeekStart,
  onSelectWeek,
}) => {
  // Calendar viewing month/year state
  const [viewYear, setViewYear] = useState(() => selectedWeekStart.getFullYear());
  const [viewMonth, setViewMonth] = useState(() => selectedWeekStart.getMonth());

  // Extract all unique members from songs
  const allMembers = React.useMemo(() => {
    const set = new Set<string>();
    songs.forEach(s => s.members.forEach(m => set.add(m)));
    return Array.from(set).sort();
  }, [songs]);

  // Count scheduled sessions per song
  const getScheduledCount = (songId: string) => {
    return schedule.filter(s => s.songId === songId).length;
  };

  const handlePrevMonth = () => {
    if (viewMonth === 0) {
      setViewMonth(11);
      setViewYear(y => y - 1);
    } else {
      setViewMonth(m => m - 1);
    }
  };

  const handleNextMonth = () => {
    if (viewMonth === 11) {
      setViewMonth(0);
      setViewYear(y => y + 1);
    } else {
      setViewMonth(m => m + 1);
    }
  };

  const monthWeeks = React.useMemo(() => {
    return getMonthMatrix(viewYear, viewMonth);
  }, [viewYear, viewMonth]);

  return (
    <aside className="sidebar">
      {/* Interactive Week Picker Mini Calendar */}
      <div className="mini-calendar">
        <div className="mini-cal-header">
          <span>Tháng {viewMonth + 1}, {viewYear}</span>
          <div style={{ display: 'flex', gap: 2 }}>
            <button
              className="mini-cal-nav-btn"
              onClick={handlePrevMonth}
              title="Tháng trước"
            >
              ‹
            </button>
            <button
              className="mini-cal-nav-btn"
              onClick={handleNextMonth}
              title="Tháng sau"
            >
              ›
            </button>
          </div>
        </div>

        <div className="mini-cal-week-labels">
          <div className="mini-cal-day-label">T2</div>
          <div className="mini-cal-day-label">T3</div>
          <div className="mini-cal-day-label">T4</div>
          <div className="mini-cal-day-label">T5</div>
          <div className="mini-cal-day-label">T6</div>
          <div className="mini-cal-day-label">T7</div>
          <div className="mini-cal-day-label">CN</div>
        </div>

        <div className="mini-cal-weeks-container">
          {monthWeeks.map((week, wIdx) => {
            const mondayDate = getMonday(week[0].date);
            const isSelectedWeek = isSameWeek(mondayDate, selectedWeekStart);
            const weekRangeLabel = formatWeekRange(mondayDate);

            return (
              <div
                key={wIdx}
                className={`mini-cal-week-row ${isSelectedWeek ? 'selected' : ''}`}
                onClick={() => onSelectWeek(mondayDate)}
                title={`Bấm để chọn xếp lịch cho tuần: ${weekRangeLabel}`}
              >
                {week.map((cell, dIdx) => (
                  <div
                    key={dIdx}
                    className={`mini-cal-day-cell ${!cell.isCurrentMonth ? 'other-month' : ''} ${
                      cell.isToday ? 'today-marker' : ''
                    } ${isSelectedWeek && dIdx === 0 ? 'monday-active' : ''}`}
                  >
                    {cell.dayNumber}
                  </div>
                ))}
              </div>
            );
          })}
        </div>
      </div>

      {/* Song List & Target Sessions Section */}
      <div>
        <div className="sidebar-section-title">
          <span style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
            <Music size={14} />
            <span>Bài hát ({songs.length})</span>
          </span>
          <span style={{ fontSize: 11, fontWeight: 'normal', textTransform: 'none' }}>
            Số buổi/tuần
          </span>
        </div>

        {songs.length === 0 ? (
          <div style={{ fontSize: 12, color: 'var(--text-muted)', padding: '8px 0' }}>
            Chưa có bài hát. Hãy tải file Excel hoặc bấm "Dữ liệu mẫu".
          </div>
        ) : (
          <div className="song-list-container">
            {songs.map(song => {
              const scheduledCount = getScheduledCount(song.id);
              const isFulfilled = scheduledCount >= song.targetSessions;

              return (
                <div key={song.id} className="song-item-card">
                  <div className="song-item-top">
                    <div
                      className="song-color-dot"
                      style={{ backgroundColor: song.color.border }}
                    />
                    <span className="song-title-text" title={song.name}>
                      {song.name}
                    </span>
                    <button
                      onClick={() => onViewSongVotes(song)}
                      style={{ background: 'none', border: 'none', cursor: 'pointer', color: '#6b7280' }}
                      title="Xem bảng vote chi tiết của bài này"
                    >
                      <Eye size={14} />
                    </button>
                    <button
                      onClick={() => onDeleteSong(song.id)}
                      style={{ background: 'none', border: 'none', cursor: 'pointer', color: '#9ca3af', fontSize: 13 }}
                      title="Xóa bài này"
                    >
                      ×
                    </button>
                  </div>

                  <div style={{ fontSize: 11, color: '#6b7280', marginTop: 4 }}>
                    {song.members.length} thành viên ({song.members.slice(0, 3).join(', ')}
                    {song.members.length > 3 ? '...' : ''})
                  </div>

                  {/* Configurable target sessions per song */}
                  <div className="song-session-config">
                    <span style={{ fontSize: 11 }}>Cần tập trong tuần:</span>
                    <div className="stepper-controls">
                      <button
                        className="stepper-btn"
                        onClick={() => onUpdateSongSessions(song.id, Math.max(1, song.targetSessions - 1))}
                        title="Giảm số buổi"
                      >
                        -
                      </button>
                      <span className="stepper-value">{song.targetSessions}</span>
                      <button
                        className="stepper-btn"
                        onClick={() => onUpdateSongSessions(song.id, song.targetSessions + 1)}
                        title="Tăng số buổi"
                      >
                        +
                      </button>
                      <span
                        style={{
                          fontSize: 10,
                          padding: '1px 5px',
                          borderRadius: 3,
                          backgroundColor: isFulfilled ? '#ecfdf5' : '#fff7ed',
                          color: isFulfilled ? '#047857' : '#c2410c',
                          fontWeight: 500,
                          marginLeft: 4,
                        }}
                      >
                        {scheduledCount}/{song.targetSessions} buổi
                      </span>
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>

      {/* Members Directory */}
      <div>
        <div className="sidebar-section-title">
          <span style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
            <Users size={14} />
            <span>Thành viên ({allMembers.length})</span>
          </span>
          {selectedMember && (
            <button
              onClick={() => onSelectMember(null)}
              style={{
                background: 'none',
                border: 'none',
                fontSize: 11,
                color: 'var(--primary-blue)',
                cursor: 'pointer',
              }}
            >
              Xem tất cả
            </button>
          )}
        </div>

        <div className="members-chip-cloud">
          {allMembers.map(member => {
            const isSelected = selectedMember === member;
            return (
              <button
                key={member}
                className={`member-chip ${isSelected ? 'selected' : ''}`}
                onClick={() => onSelectMember(isSelected ? null : member)}
                title={`Bấm để xem lịch tập riêng của ${member}`}
              >
                {member}
              </button>
            );
          })}
        </div>
      </div>

      {/* Solver Settings */}
      <div>
        <div className="sidebar-section-title">
          <span style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
            <Sliders size={14} />
            <span>Cài đặt xếp lịch</span>
          </span>
        </div>

        <div className="settings-box">
          <div className="setting-row">
            <span className="setting-label">Số phòng tập cùng lúc:</span>
            <select
              className="setting-select"
              value={settings.maxRooms}
              onChange={e =>
                onUpdateSettings({ ...settings, maxRooms: parseInt(e.target.value, 10) })
              }
            >
              <option value={1}>1 Phòng (Tiêu chuẩn)</option>
              <option value={2}>2 Phòng đồng thời</option>
              <option value={3}>3 Phòng đồng thời</option>
            </select>
          </div>

          <div className="setting-row">
            <span className="setting-label">Linh hoạt vắng 1 người:</span>
            <input
              type="checkbox"
              checked={settings.allowPartialAttendance}
              onChange={e =>
                onUpdateSettings({ ...settings, allowPartialAttendance: e.target.checked })
              }
            />
          </div>

          <div className="setting-row">
            <span className="setting-label">Rải đều các ngày:</span>
            <input
              type="checkbox"
              checked={settings.spreadDays}
              onChange={e =>
                onUpdateSettings({ ...settings, spreadDays: e.target.checked })
              }
            />
          </div>
        </div>
      </div>
    </aside>
  );
};
