<script lang="ts">
  import type { SongVoteData, SolverSettings, ScheduledSession } from '../types/timetable';
  import { Music, Users, Sliders, Eye, X, ChevronLeft, ChevronRight } from '@lucide/svelte';
  import { getMonthMatrix, isSameWeek, formatWeekRange, getMonday } from '../utils/dateUtils';

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

  let viewYear = $state(2026);
  let viewMonth = $state(8);

  $effect(() => {
    viewYear = selectedWeekStart.getFullYear();
    viewMonth = selectedWeekStart.getMonth();
  });

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
  <!-- Mobile Drawer Header -->
  <div class="sidebar-mobile-header">
    <div class="sidebar-mobile-title">
      <Sliders size={18} color="var(--accent)" />
      <span>Tùy chọn & Dữ liệu</span>
    </div>
    {#if onCloseMobile}
      <button
        type="button"
        class="bento-icon-btn"
        onclick={onCloseMobile}
        aria-label="Đóng menu"
      >
        <X size={18} />
      </button>
    {/if}
  </div>

  <div class="sidebar-scroll-area">
    <!-- Interactive Week Picker Mini Calendar -->
    <div class="mini-calendar">
      <div class="mini-cal-header">
        <span>Tháng {viewMonth + 1}, {viewYear}</span>
        <div style="display: flex; gap: 4px;">
          <button
            type="button"
            class="mini-cal-nav-btn"
            onclick={handlePrevMonth}
            title="Tháng trước"
            aria-label="Tháng trước"
          >
            <ChevronLeft size={14} />
          </button>
          <button
            type="button"
            class="mini-cal-nav-btn"
            onclick={handleNextMonth}
            title="Tháng sau"
            aria-label="Tháng sau"
          >
            <ChevronRight size={14} />
          </button>
        </div>
      </div>

      <div class="mini-cal-grid" style="margin-bottom: 4px;">
        <div class="mini-cal-day-label">T2</div>
        <div class="mini-cal-day-label">T3</div>
        <div class="mini-cal-day-label">T4</div>
        <div class="mini-cal-day-label">T5</div>
        <div class="mini-cal-day-label">T6</div>
        <div class="mini-cal-day-label">T7</div>
        <div class="mini-cal-day-label">CN</div>
      </div>

      <div style="display: flex; flex-direction: column; gap: 2px;">
        {#each monthWeeks as week, wIdx (wIdx)}
          {@const mondayDate = getMonday(week[0].date)}
          {@const isSelectedWeek = isSameWeek(mondayDate, selectedWeekStart)}
          {@const weekRangeLabel = formatWeekRange(mondayDate)}

          <div
            class="mini-cal-grid"
            style="cursor: pointer; border-radius: var(--radius-xs); padding: 1px 0;"
            onclick={() => onSelectWeek(mondayDate)}
            onkeydown={(e) => { if (e.key === 'Enter') onSelectWeek(mondayDate); }}
            role="button"
            tabindex="0"
            title="Chọn tuần: {weekRangeLabel}"
          >
            {#each week as cell, dIdx (dIdx)}
              <div
                class="mini-cal-day-cell {!cell.isCurrentMonth ? 'is-other-month' : ''} {cell.isToday ? 'is-today' : ''} {isSelectedWeek ? 'is-selected-week' : ''}"
              >
                {cell.dayNumber}
              </div>
            {/each}
          </div>
        {/each}
      </div>
    </div>

    <!-- Song List & Target Sessions Section -->
    <div>
      <div class="sidebar-section-title">
        <span style="display: flex; align-items: center; gap: 6px;">
          <Music size={14} color="var(--accent)" />
          <span>Bài hát ({songs.length})</span>
        </span>
        <span style="font-size: 11px; font-weight: 500; color: var(--text-muted);">
          Số buổi
        </span>
      </div>

      {#if songs.length === 0}
        <div style="font-size: 12px; color: var(--text-muted); padding: 8px 4px; text-align: center;">
          Chưa có bài hát. Hãy nạp file Excel hoặc chọn Dữ liệu mẫu.
        </div>
      {:else}
        <div>
          {#each songs as song (song.id)}
            {@const scheduledCount = getScheduledCount(song.id)}
            {@const isFulfilled = scheduledCount >= song.targetSessions}

            <div class="sidebar-song-card">
              <div class="sidebar-song-top">
                <div class="sidebar-song-title-wrap">
                  <div
                    class="sidebar-song-dot"
                    style="background-color: {song.color.border};"
                  ></div>
                  <span class="sidebar-song-name" title={song.name}>
                    {song.name}
                  </span>
                </div>

                <div class="sidebar-song-actions">
                  <button
                    type="button"
                    class="bento-icon-btn"
                    style="width: 26px; height: 26px;"
                    onclick={() => onViewSongVotes(song)}
                    title="Xem bảng vote chi tiết của bài này"
                    aria-label="Xem vote"
                  >
                    <Eye size={13} />
                  </button>
                  <button
                    type="button"
                    class="bento-icon-btn"
                    style="width: 26px; height: 26px; color: var(--danger);"
                    onclick={() => onDeleteSong(song.id)}
                    title="Xóa bài này"
                    aria-label="Xóa bài"
                  >
                    <X size={13} />
                  </button>
                </div>
              </div>

              <div class="sidebar-song-meta">
                {song.members.length} thành viên ({song.members.slice(0, 3).join(', ')}{song.members.length > 3 ? '...' : ''})
              </div>

              <div class="sidebar-song-controls">
                <span>Cần tập trong tuần:</span>
                <div style="display: flex; align-items: center; gap: 4px;">
                  <button
                    type="button"
                    class="bento-btn"
                    style="padding: 2px 8px; font-size: 11px; min-width: 22px; height: 24px;"
                    onclick={() => onUpdateSongSessions(song.id, Math.max(1, song.targetSessions - 1))}
                    title="Giảm số buổi"
                  >
                    -
                  </button>
                  <span class="sidebar-freq-input" style="display: flex; align-items: center; justify-content: center;">
                    {song.targetSessions}
                  </span>
                  <button
                    type="button"
                    class="bento-btn"
                    style="padding: 2px 8px; font-size: 11px; min-width: 22px; height: 24px;"
                    onclick={() => onUpdateSongSessions(song.id, song.targetSessions + 1)}
                    title="Tăng số buổi"
                  >
                    +
                  </button>
                  <span
                    class="bento-pill {isFulfilled ? 'is-accent' : ''}"
                    style="font-size: 10px; padding: 2px 6px; {isFulfilled ? '' : 'color: var(--warning-text); background: var(--warning-light);'}"
                  >
                    {scheduledCount}/{song.targetSessions}
                  </span>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Members Filter Directory -->
    <div>
      <div class="sidebar-section-title">
        <span style="display: flex; align-items: center; gap: 6px;">
          <Users size={14} color="var(--accent)" />
          <span>Thành viên ({allMembers.length})</span>
        </span>
        {#if selectedMember}
          <button
            type="button"
            class="bento-pill is-active"
            onclick={() => onSelectMember(null)}
            style="font-size: 10px; padding: 2px 8px;"
          >
            Bỏ lọc
          </button>
        {/if}
      </div>

      {#if allMembers.length === 0}
        <div style="font-size: 12px; color: var(--text-muted); padding: 4px; text-align: center;">
          Chưa có danh sách thành viên.
        </div>
      {:else}
        <div class="member-chips-container">
          {#each allMembers as member (member)}
            <button
              type="button"
              class="member-chip {selectedMember === member ? 'is-active' : ''}"
              onclick={() => onSelectMember(selectedMember === member ? null : member)}
              title="Lọc xem lịch riêng của {member}"
            >
              {member}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Solver Settings Panel -->
    <div>
      <div class="sidebar-section-title">
        <span style="display: flex; align-items: center; gap: 6px;">
          <Sliders size={14} color="var(--accent)" />
          <span>Cấu hình giải thuật</span>
        </span>
      </div>

      <div class="settings-box">
        <div class="setting-row">
          <div>
            <div>Số phòng đồng thời</div>
            <div class="setting-subtext">Giới hạn số bài tập song song</div>
          </div>
          <select
            class="bento-input"
            style="width: auto; padding: 4px 8px; font-weight: 700;"
            value={settings.maxRooms}
            onchange={(e) => onUpdateSettings({ ...settings, maxRooms: Number((e.target as HTMLSelectElement).value) })}
          >
            <option value={1}>1 Phòng</option>
            <option value={2}>2 Phòng</option>
            <option value={3}>3 Phòng</option>
          </select>
        </div>

        <div class="setting-row">
          <div>
            <div>Cho phép vắng 1 người</div>
            <div class="setting-subtext">Cứu các bài kẹt lịch 100%</div>
          </div>
          <label class="bento-switch">
            <input
              type="checkbox"
              checked={settings.allowPartialAttendance}
              onchange={(e) => onUpdateSettings({ ...settings, allowPartialAttendance: (e.target as HTMLInputElement).checked })}
            />
            <span class="bento-switch-slider"></span>
          </label>
        </div>

        <div class="setting-row">
          <div>
            <div>Ưu tiên giãn ngày tập</div>
            <div class="setting-subtext">Không dồn bài vào 1 ngày</div>
          </div>
          <label class="bento-switch">
            <input
              type="checkbox"
              checked={settings.spreadDays}
              onchange={(e) => onUpdateSettings({ ...settings, spreadDays: (e.target as HTMLInputElement).checked })}
            />
            <span class="bento-switch-slider"></span>
          </label>
        </div>
      </div>
    </div>
  </div>
</aside>
