<script lang="ts">
  import type { SongVoteData, SolverSettings, ScheduledSession } from '../types/timetable';
  import { Music, Users, Sliders, Eye, X, ChevronLeft, ChevronRight } from '@lucide/svelte';
  import { getMonthMatrix, isSameWeek, formatWeekRange, getMonday } from '../utils/dateUtils';
  import { tStore, currentLocale } from '$lib/i18n';

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

  const ENGLISH_MONTHS = [
    'January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December'
  ];

  let displayMonthYear = $derived.by(() => {
    if ($currentLocale === 'en') {
      return `${ENGLISH_MONTHS[viewMonth]} ${viewYear}`;
    }
    return `Tháng ${viewMonth + 1}, ${viewYear}`;
  });

  let miniCalDayLabels = $derived.by(() => {
    if ($currentLocale === 'en') {
      return ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
    }
    return ['T2', 'T3', 'T4', 'T5', 'T6', 'T7', 'CN'];
  });

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
      <span>{$tStore('sidebar.options_and_data')}</span>
    </div>
    {#if onCloseMobile}
      <button
        type="button"
        class="bento-icon-btn"
        onclick={onCloseMobile}
        aria-label={$tStore('sidebar.close_menu')}
      >
        <X size={18} />
      </button>
    {/if}
  </div>

  <div class="sidebar-scroll-area">
    <!-- Interactive Week Picker Mini Calendar -->
    <div class="mini-calendar">
      <div class="mini-cal-header">
        <span>{displayMonthYear}</span>
        <div style="display: flex; gap: 4px;">
          <button
            type="button"
            class="mini-cal-nav-btn"
            onclick={handlePrevMonth}
            title={$tStore('sidebar.prev_month')}
            aria-label={$tStore('sidebar.prev_month')}
          >
            <ChevronLeft size={14} />
          </button>
          <button
            type="button"
            class="mini-cal-nav-btn"
            onclick={handleNextMonth}
            title={$tStore('sidebar.next_month')}
            aria-label={$tStore('sidebar.next_month')}
          >
            <ChevronRight size={14} />
          </button>
        </div>
      </div>

      <div class="mini-cal-grid" style="margin-bottom: 4px;">
        {#each miniCalDayLabels as dLabel}
          <div class="mini-cal-day-label">{dLabel}</div>
        {/each}
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
            title={$tStore('sidebar.select_week_tooltip', { range: weekRangeLabel })}
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
          <span>{$tStore('sidebar.songs_header', { count: songs.length })}</span>
        </span>
        <span style="font-size: 11px; font-weight: 500; color: var(--text-muted);">
          {$tStore('sidebar.col_sessions')}
        </span>
      </div>

      {#if songs.length === 0}
        <div style="font-size: 12px; color: var(--text-muted); padding: 8px 4px; text-align: center;">
          {$tStore('sidebar.empty_songs')}
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
                    title={$tStore('sidebar.view_votes_tooltip')}
                    aria-label={$tStore('sidebar.view_votes_tooltip')}
                  >
                    <Eye size={13} />
                  </button>
                  <button
                    type="button"
                    class="bento-icon-btn"
                    style="width: 26px; height: 26px; color: var(--danger);"
                    onclick={() => onDeleteSong(song.id)}
                    title={$tStore('sidebar.delete_song_tooltip')}
                    aria-label={$tStore('sidebar.delete_song_tooltip')}
                  >
                    <X size={13} />
                  </button>
                </div>
              </div>

              <div class="sidebar-song-meta">
                {song.members.length} {$tStore('sidebar.members_count', { count: song.members.length })} ({song.members.slice(0, 3).join(', ')}{song.members.length > 3 ? '...' : ''})
              </div>

              <div class="sidebar-song-controls">
                <span>{$tStore('sidebar.weekly_need')}</span>
                <div style="display: flex; align-items: center; gap: 4px;">
                  <button
                    type="button"
                    class="bento-btn"
                    style="padding: 2px 8px; font-size: 11px; min-width: 22px; height: 24px;"
                    onclick={() => onUpdateSongSessions(song.id, Math.max(1, song.targetSessions - 1))}
                    title={$tStore('sidebar.decrease_sessions')}
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
                    title={$tStore('sidebar.increase_sessions')}
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
          <span>{$tStore('sidebar.members_header', { count: allMembers.length })}</span>
        </span>
        {#if selectedMember}
          <button
            type="button"
            class="bento-pill is-active"
            onclick={() => onSelectMember(null)}
            style="font-size: 10px; padding: 2px 8px;"
          >
            {$tStore('sidebar.clear_filter')}
          </button>
        {/if}
      </div>

      {#if allMembers.length === 0}
        <div style="font-size: 12px; color: var(--text-muted); padding: 4px; text-align: center;">
          {$tStore('sidebar.empty_members')}
        </div>
      {:else}
        <div class="member-chips-container">
          {#each allMembers as member (member)}
            <button
              type="button"
              class="member-chip {selectedMember === member ? 'is-active' : ''}"
              onclick={() => onSelectMember(selectedMember === member ? null : member)}
              title={$tStore('sidebar.filter_member_tooltip', { member })}
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
          <span>{$tStore('sidebar.settings_title')}</span>
        </span>
      </div>

      <div class="settings-box">
        <div class="setting-row">
          <div>
            <div>{$tStore('sidebar.max_rooms')}</div>
            <div class="setting-subtext">{$tStore('sidebar.max_rooms_desc')}</div>
          </div>
          <select
            class="bento-input"
            style="width: auto; padding: 4px 8px; font-weight: 700;"
            value={settings.maxRooms}
            onchange={(e) => onUpdateSettings({ ...settings, maxRooms: Number((e.target as HTMLSelectElement).value) })}
          >
            <option value={1}>{$currentLocale === 'en' ? '1 Room' : '1 Phòng'}</option>
            <option value={2}>{$currentLocale === 'en' ? '2 Rooms' : '2 Phòng'}</option>
            <option value={3}>{$currentLocale === 'en' ? '3 Rooms' : '3 Phòng'}</option>
          </select>
        </div>

        <div class="setting-row">
          <div>
            <div>{$tStore('sidebar.allow_partial')}</div>
            <div class="setting-subtext">{$tStore('sidebar.allow_partial_desc')}</div>
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
            <div>{$tStore('sidebar.spread_days')}</div>
            <div class="setting-subtext">{$tStore('sidebar.spread_days_desc')}</div>
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
