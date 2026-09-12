<script lang="ts">
  import type { SongVoteData, SolverSettings, ScheduledSession } from '../types/timetable';
  import { Music, Users, SlidersVertical, Eye, X, ChevronLeft, ChevronRight } from '@lucide/svelte';
  import { getMonthMatrix, isSameWeek, formatWeekRange, getMonday } from '../utils/dateUtils';
  import { tStore, currentLocale } from '$lib/i18n';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';
  import { cn } from '$lib/utils';

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

  function prevMonth() {
    if (viewMonth === 0) {
      viewMonth = 11;
      viewYear -= 1;
    } else {
      viewMonth -= 1;
    }
  }

  function nextMonth() {
    if (viewMonth === 11) {
      viewMonth = 0;
      viewYear += 1;
    } else {
      viewMonth += 1;
    }
  }

  function getMemberSessionCount(member: string) {
    return schedule.filter(s => s.allMembers.includes(member)).length;
  }
</script>

<aside class={cn(
  "w-80 flex-shrink-0 flex flex-col gap-4 border-r border-slate-200 bg-white p-4 overflow-y-auto z-30 transition-transform duration-300 dark:border-slate-800 dark:bg-card md:static md:translate-x-0",
  isOpenMobile ? "fixed inset-y-0 left-0 shadow-2xl translate-x-0" : "fixed -translate-x-full md:translate-x-0"
)}>
  {#if isOpenMobile}
    <div class="flex items-center justify-between md:hidden pb-2 border-b border-slate-100 dark:border-slate-800">
      <span class="text-xs font-bold text-slate-800 dark:text-slate-100">Controls</span>
      <Button variant="ghost" size="icon-xs" onclick={onCloseMobile}>
        <X size={16} />
      </Button>
    </div>
  {/if}

  <!-- Mini Calendar Picker -->
  <Card class="p-3 border border-slate-200 bg-slate-50/50 dark:border-slate-800 dark:bg-slate-900/30">
    <div class="flex items-center justify-between mb-2">
      <strong class="text-xs font-bold text-slate-800 dark:text-slate-100">{displayMonthYear}</strong>
      <div class="flex items-center gap-1">
        <Button variant="ghost" size="icon-xs" onclick={prevMonth}>
          <ChevronLeft size={13} />
        </Button>
        <Button variant="ghost" size="icon-xs" onclick={nextMonth}>
          <ChevronRight size={13} />
        </Button>
      </div>
    </div>

    <!-- Mini calendar table -->
    <table class="w-full text-center text-[10px]">
      <thead>
        <tr class="text-slate-400 font-semibold">
          {#each miniCalDayLabels as dl}
            <th class="p-1">{dl}</th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each monthWeeks as week}
          {@const isThisWeekSelected = isSameWeek(week[0].date, selectedWeekStart)}
          <tr
            class={cn(
              "cursor-pointer rounded-md transition-colors hover:bg-primary/10",
              isThisWeekSelected && "bg-primary/20"
            )}
            onclick={() => onSelectWeek(week[0].date)}
          >
            {#each week as dayObj}
              <td class={cn(
                "p-1.5 font-medium",
                !dayObj.isCurrentMonth && "text-muted-foreground/40",
                dayObj.isCurrentMonth && "text-foreground",
                dayObj.isToday && "text-primary font-bold"
              )}>
                {dayObj.dayNumber}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </Card>

  <!-- Repertoire / Songs Section -->
  <div class="flex flex-col gap-2">
    <div class="flex items-center justify-between text-xs font-bold text-foreground">
      <div class="flex items-center gap-1.5">
        <Music size={14} class="text-primary" />
        <span>{$tStore('sidebar.songs_title')} ({songs.length})</span>
      </div>
    </div>

    <div class="flex flex-col gap-1.5 max-h-56 overflow-y-auto pr-1">
      {#each songs as song (song.id)}
        <Card class="flex items-center justify-between p-2.5 border border-border bg-card">
          <div class="flex items-center gap-2 min-w-0">
            <div
              class="h-2.5 w-2.5 rounded-full shrink-0"
              style="background-color: {song.color.border};"
            ></div>
            <div class="flex flex-col min-w-0">
              <span class="text-xs font-semibold text-foreground truncate">{song.name}</span>
              <span class="text-[10px] text-muted-foreground">
                {$tStore('sidebar.song_members_count', { count: song.members.length })}
              </span>
            </div>
          </div>

          <div class="flex items-center gap-1">
            <Button
              variant="ghost"
              size="icon-xs"
              onclick={() => onViewSongVotes(song)}
              title={$tStore('sidebar.view_votes')}
            >
              <Eye size={13} class="text-muted-foreground" />
            </Button>
            <Button
              variant="ghost"
              size="icon-xs"
              onclick={() => onDeleteSong(song.id)}
              class="text-destructive hover:text-destructive/80"
              title={$tStore('sidebar.delete_song')}
            >
              <X size={13} />
            </Button>
          </div>
        </Card>
      {/each}
    </div>
  </div>

  <!-- Roster / Members Filter -->
  <div class="flex flex-col gap-2">
    <div class="flex items-center justify-between text-xs font-bold text-foreground">
      <div class="flex items-center gap-1.5">
        <Users size={14} class="text-primary" />
        <span>{$tStore('sidebar.members_title')} ({allMembers.length})</span>
      </div>
      {#if selectedMember}
        <Button variant="ghost" size="xs" onclick={() => onSelectMember(null)}>
          {$tStore('sidebar.clear_filter')}
        </Button>
      {/if}
    </div>

    <div class="flex flex-wrap gap-1.5 max-h-40 overflow-y-auto">
      {#each allMembers as member}
        {@const count = getMemberSessionCount(member)}
        {@const isSelected = selectedMember === member}
        <Button
          variant={isSelected ? 'default' : 'outline'}
          size="xs"
          onclick={() => onSelectMember(isSelected ? null : member)}
          class="text-[11px] gap-1"
        >
          <span>{member}</span>
          <Badge variant={isSelected ? 'secondary' : 'default'} class="text-[9px] px-1 py-0 h-4">
            {count}
          </Badge>
        </Button>
      {/each}
    </div>
  </div>

  <!-- Solver Settings -->
  <div class="flex flex-col gap-2 pt-2 border-t border-border">
    <div class="flex items-center gap-1.5 font-bold text-xs uppercase tracking-wider text-muted-foreground">
      <SlidersVertical size={14} class="text-primary" />
      <span>{$tStore('sidebar.solver_settings')}</span>
    </div>

    <div class="flex flex-col gap-2 text-xs">
      <label class="flex items-center justify-between cursor-pointer">
        <span class="text-muted-foreground">{$tStore('sidebar.setting_max_rooms')}</span>
        <select
          class="rounded-md border border-border bg-muted/40 px-2 py-1 text-xs text-foreground"
          value={settings.maxRooms}
          onchange={(e) => onUpdateSettings({ ...settings, maxRooms: parseInt((e.target as HTMLSelectElement).value) })}
        >
          <option value="1">1 {$tStore('sidebar.room_single')}</option>
          <option value="2">2 {$tStore('sidebar.rooms_multi')}</option>
          <option value="3">3 {$tStore('sidebar.rooms_multi')}</option>
        </select>
      </label>

      <label class="flex items-center justify-between cursor-pointer">
        <span class="text-muted-foreground">{$tStore('sidebar.setting_spread')}</span>
        <input
          type="checkbox"
          class="rounded text-primary focus:ring-primary accent-primary"
          checked={settings.spreadDays}
          onchange={(e) => onUpdateSettings({ ...settings, spreadDays: (e.target as HTMLInputElement).checked })}
        />
      </label>
    </div>
  </div>
</aside>
