<script lang="ts">
  import type { SongVoteData, SolverSettings, ScheduledSession } from '../types/timetable';
  import { Music, Users, Sliders, Eye, X, ChevronLeft, ChevronRight } from '@lucide/svelte';
  import { getMonthMatrix, isSameWeek, getMonday } from '../utils/dateUtils';

  interface Props {
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
    isOpenMobile?: boolean;
    onCloseMobile?: () => void;
  }

  let {
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
    isOpenMobile = false,
    onCloseMobile,
  }: Props = $props();

  let viewYear = $state(selectedWeekStart.getFullYear());
  let viewMonth = $state(selectedWeekStart.getMonth());

  let allMembers = $derived.by(() => {
    const set = new Set<string>();
    songs.forEach(s => s.members.forEach(m => set.add(m)));
    return Array.from(set).sort();
  });

  let monthWeeks = $derived(getMonthMatrix(viewYear, viewMonth));

  function getScheduledCount(songId: string) {
    return schedule.filter(s => s.songId === songId).length;
  }

  function handlePrevMonth() {
    if (viewMonth === 0) {
      viewMonth = 11;
      viewYear -= 1;
    } else {
      viewMonth -= 1;
    }
  }

  function handleNextMonth() {
    if (viewMonth === 11) {
      viewMonth = 0;
      viewYear += 1;
    } else {
      viewMonth += 1;
    }
  }
</script>

<aside class="sidebar {isOpenMobile ? 'open' : ''}">
  <div class="sidebar-mobile-header">
    <div class="sidebar-mobile-title">
      <Sliders size={18} color="#1a73e8" />
      <span>Tùy chọn & Dữ liệu</span>
    </div>
    {#if onCloseMobile}
      <button
        type="button"
        class="sidebar-mobile-close-btn"
        onclick={onCloseMobile}
        aria-label="Đóng menu"
      >
        <X size={18} />
      </button>
    {/if}
  </div>

  <!-- Interactive Week Picker Mini Calendar -->
  <div class="mini-calendar">
    <div class="mini-cal-header">
      <span>Tháng {viewMonth + 1}, {viewYear}</span>
      <div style="display: flex; gap: 2px;">
        <button
          type="button"
          class="mini-cal-nav-btn"
          onclick={handlePrevMonth}
          title="Tháng trước"
        >
          <ChevronLeft size={16} />
        </button>
        <button
          type="button"
          class="mini-cal-nav-btn"
          onclick={handleNextMonth}
          title="Tháng sau"
        >
          <ChevronRight size={16} />
        </button>
      </div>
    </div>

    <div class="mini-cal-grid">
      {#each ['T2', 'T3', 'T4', 'T5', 'T6', 'T7', 'CN'] as d}
        <span class="mini-cal-day-label">{d}</span>
      {/each}

      {#each monthWeeks as week}
        {#each week as cell}
          {@const isCellSelectedWeek = isSameWeek(cell.date, selectedWeekStart)}
          <button
            type="button"
            class="mini-cal-day-cell {cell.isCurrentMonth ? '' : 'outside'} {cell.isToday ? 'today' : ''} {isCellSelectedWeek ? 'selected-week' : ''}"
            onclick={() => onSelectWeek(getMonday(cell.date))}
            title="Bấm để chọn tuần {cell.date.toLocaleDateString()}"
          >
            {cell.dayNumber}
          </button>
        {/each}
      {/each}
    </div>
  </div>

  <!-- Solver Settings Controls -->
  <div class="sidebar-section">
    <div class="sidebar-section-title">
      <Sliders size={14} />
      <span>Cấu hình xếp lịch</span>
    </div>

    <div class="setting-row">
      <label for="setting-max-rooms">Số phòng tập đồng thời:</label>
      <select
        id="setting-max-rooms"
        class="select-gcal"
        value={settings.maxRooms}
        onchange={(e) => onUpdateSettings({ ...settings, maxRooms: Number((e.target as HTMLSelectElement).value) })}
      >
        <option value={1}>1 Phòng (Tiêu chuẩn)</option>
        <option value={2}>2 Phòng</option>
        <option value={3}>3 Phòng</option>
      </select>
    </div>

    <div class="setting-row-checkbox">
      <input
        type="checkbox"
        id="setting-allow-partial"
        checked={settings.allowPartialAttendance}
        onchange={(e) => onUpdateSettings({ ...settings, allowPartialAttendance: (e.target as HTMLInputElement).checked })}
      />
      <label for="setting-allow-partial">
        Cho phép vắng 1 người nếu không thể đủ 100%
      </label>
    </div>

    <div class="setting-row-checkbox">
      <input
        type="checkbox"
        id="setting-spread-days"
        checked={settings.spreadDays}
        onchange={(e) => onUpdateSettings({ ...settings, spreadDays: (e.target as HTMLInputElement).checked })}
      />
      <label for="setting-spread-days">
        Ưu tiên giãn cách các ngày trong tuần
      </label>
    </div>
  </div>

  <!-- Filter by Member Attendance -->
  <div class="sidebar-section">
    <div class="sidebar-section-title">
      <Users size={14} />
      <span>Lọc lịch theo thành viên</span>
    </div>

    <div class="member-filter-wrapper">
      <select
        class="select-gcal"
        value={selectedMember ?? ''}
        onchange={(e) => {
          const val = (e.target as HTMLSelectElement).value;
          onSelectMember(val === '' ? null : val);
        }}
      >
        <option value="">-- Xem lịch toàn bộ thành viên --</option>
        {#each allMembers as member}
          <option value={member}>{member}</option>
        {/each}
      </select>
    </div>
  </div>

  <!-- Songs & Rehearsal Frequencies -->
  <div class="sidebar-section songs-list-section">
    <div class="sidebar-section-title">
      <Music size={14} />
      <span>Danh sách bài hát ({songs.length})</span>
    </div>

    <div class="songs-scroll-list">
      {#each songs as song (song.id)}
        {@const scheduledCount = getScheduledCount(song.id)}
        {@const isFulfilled = scheduledCount >= song.targetSessions}

        <div class="song-item-card" style="border-left: 4px solid {song.color.border};">
          <div class="song-item-header">
            <span class="song-item-title" style="color: {song.color.text};">
              {song.name}
            </span>
            <div class="song-item-actions">
              <button
                type="button"
                class="song-item-btn"
                onclick={() => onViewSongVotes(song)}
                title="Xem bảng vote gốc"
              >
                <Eye size={13} />
              </button>
              <button
                type="button"
                class="song-item-btn delete"
                onclick={() => onDeleteSong(song.id)}
                title="Xóa bài hát"
              >
                <X size={13} />
              </button>
            </div>
          </div>

          <div class="song-item-meta">
            <span>👥 {song.members.length} thành viên</span>
            <span class="song-item-status {isFulfilled ? 'fulfilled' : 'pending'}">
              {scheduledCount}/{song.targetSessions} buổi
            </span>
          </div>

          <div class="song-freq-controls">
            <label for="freq-{song.id}">Số buổi/tuần:</label>
            <div class="song-freq-stepper">
              <button
                type="button"
                class="freq-btn"
                onclick={() => onUpdateSongSessions(song.id, Math.max(1, song.targetSessions - 1))}
                disabled={song.targetSessions <= 1}
              >-</button>
              <span class="freq-value">{song.targetSessions}</span>
              <button
                type="button"
                class="freq-btn"
                onclick={() => onUpdateSongSessions(song.id, song.targetSessions + 1)}
              >+</button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>
</aside>
