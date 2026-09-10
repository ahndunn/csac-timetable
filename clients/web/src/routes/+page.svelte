<script lang="ts">
  import { enhance } from '$app/forms';
  import type {
    SongVoteData,
    ScheduledSession,
    SolverSettings,
    ConflictItem,
    UnresolvedSong,
    DayOfWeek,
  } from '$lib/types/timetable';
  import { DEFAULT_WEEK_TITLE, DAYS_OF_WEEK, DEFAULT_TIME_SLOTS } from '$lib/constants/timetableDefaults';
  import {
    generateSingleTabSampleFiles,
    generateMultiTabSampleFile,
    generateMixedSampleFiles,
  } from '$lib/engine/sampleData';
  import {
    inspectExcelFiles,
    parseSelectedSheets,
    type FileInspection,
  } from '$lib/engine/excelParser';
  import { solveTimetable, detectConflicts } from '$lib/engine/scheduler';
  import { exportTimetableToExcel, downloadExcelTemplate } from '$lib/engine/excelExporter';

  import Navbar from '$lib/components/Navbar.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import CalendarGrid from '$lib/components/CalendarGrid.svelte';
  import EventDetailModal from '$lib/components/EventDetailModal.svelte';
  import ConflictResolverModal from '$lib/components/ConflictResolverModal.svelte';
  import UploadModal from '$lib/components/UploadModal.svelte';
  import SheetSelectionModal from '$lib/components/SheetSelectionModal.svelte';
  import RawVoteModal from '$lib/components/RawVoteModal.svelte';
  import SlotAddModal from '$lib/components/SlotAddModal.svelte';

  // Svelte 5 Runes State Management
  let selectedWeekStart = $state(new Date(2026, 8, 7));
  let songs = $state<SongVoteData[]>([]);
  let weekTitle = $state(DEFAULT_WEEK_TITLE);

  let schedule = $state<ScheduledSession[]>([]);
  let conflicts = $state<ConflictItem[]>([]);
  let unresolved = $state<UnresolvedSong[]>([]);
  let isSolving = $state(false);

  let settings = $state<SolverSettings>({
    maxRooms: 1,
    allowPartialAttendance: false,
    spreadDays: true,
  });

  let selectedMember = $state<string | null>(null);

  // Modals state
  let isUploadOpen = $state(false);
  let isMobileSidebarOpen = $state(false);
  let activeSessionDetail = $state<ScheduledSession | null>(null);
  let activeVoteDetailSong = $state<SongVoteData | null>(null);
  let isConflictResolverOpen = $state(false);
  let slotAddCoord = $state<{ day: DayOfWeek; slot: string } | null>(null);
  let sheetSelectionInspections = $state<FileInspection[] | null>(null);

  function runScheduler(songsToSolve = songs, currentSettings = settings) {
    if (songsToSolve.length === 0) {
      schedule = [];
      unresolved = [];
      conflicts = [];
      return;
    }
    isSolving = true;
    setTimeout(() => {
      const result = solveTimetable(songsToSolve, currentSettings);
      schedule = result.schedule;
      unresolved = result.unresolved;
      conflicts = result.conflicts;
      isSolving = false;

      if (result.unresolved.length > 0) {
        isConflictResolverOpen = true;
      }
    }, 100);
  }

  function handleUpdateSongSessions(songId: string, newTarget: number) {
    songs = songs.map(s => (s.id === songId ? { ...s, targetSessions: newTarget } : s));
  }

  function handleDeleteSong(songId: string) {
    songs = songs.filter(s => s.id !== songId);
    schedule = schedule.filter(s => s.songId !== songId);
  }

  function handleDeleteSession(sessionId: string) {
    schedule = schedule.filter(s => s.id !== sessionId);
    const songsMap = new Map(songs.map(s => [s.id, s]));
    conflicts = detectConflicts(schedule, songsMap, settings.maxRooms);
    activeSessionDetail = null;
  }

  function handleAddUploadedSongs(newSongs: SongVoteData[]) {
    songs = [...songs, ...newSongs];
    if (newSongs[0]?.weekTitle) {
      weekTitle = newSongs[0].weekTitle;
    }
    runScheduler([...songs], settings);
  }

  async function handleLoadSampleMultiTab() {
    const file = generateMultiTabSampleFile();
    const inspections = await inspectExcelFiles([file]);
    sheetSelectionInspections = inspections;
  }

  async function handleLoadSampleSingleTab() {
    const files = generateSingleTabSampleFiles();
    const inspections = await inspectExcelFiles(files);
    const allKeys = new Set<string>();
    for (const f of inspections) {
      for (const s of f.sheets) {
        if (s.isValid) allKeys.add(`${f.fileId}::${s.sheetName}`);
      }
    }
    const samples = parseSelectedSheets(inspections, allKeys, 0);
    songs = samples;
    weekTitle = DEFAULT_WEEK_TITLE;
    runScheduler(samples, settings);
  }

  async function handleLoadSampleMixed() {
    const files = generateMixedSampleFiles();
    const inspections = await inspectExcelFiles(files);
    sheetSelectionInspections = inspections;
  }

  function handleResetSchedule() {
    schedule = [];
    unresolved = [];
    conflicts = [];
  }

  function handleExportExcel() {
    if (schedule.length === 0) {
      alert('Chưa có lịch tập được xếp để xuất file Excel.');
      return;
    }
    exportTimetableToExcel(schedule, songs, weekTitle);
  }

  function handleManualAssign(songId: string, day: DayOfWeek, slot: string) {
    const song = songs.find(s => s.id === songId);
    if (!song) return;

    const availableMembers = song.members.filter(m => song.availability[`${day}__${slot}__${m}`]);
    const absentMembers = song.members.filter(m => !song.availability[`${day}__${slot}__${m}`]);

    const newSession: ScheduledSession = {
      id: `manual-${song.id}-${day}-${slot}-${Date.now()}`,
      songId: song.id,
      songName: song.name,
      day,
      slot,
      room: 1,
      allMembers: song.members,
      availableMembers,
      absentMembers,
      color: song.color,
      isManual: true,
    };

    schedule = [...schedule, newSession];
    unresolved = unresolved.filter(u => u.songId !== songId);
    const songsMap = new Map(songs.map(s => [s.id, s]));
    conflicts = detectConflicts(schedule, songsMap, settings.maxRooms);
    isConflictResolverOpen = false;
  }

  function handleToggleVote(songId: string, day: DayOfWeek, slot: string, member: string) {
    songs = songs.map(s => {
      if (s.id !== songId) return s;
      const key = `${day}__${slot}__${member}`;
      return {
        ...s,
        availability: {
          ...s.availability,
          [key]: !s.availability[key],
        },
      };
    });

    if (activeVoteDetailSong && activeVoteDetailSong.id === songId) {
      activeVoteDetailSong = songs.find(s => s.id === songId) || null;
    }
  }
</script>

<svelte:head>
  <title>CSAC Timetable Studio 🎵📅</title>
  <meta name="description" content="Hệ thống xếp lịch tập tự động cho CLB âm nhạc CSAC" />
</svelte:head>

<div class="app-container">
  <Navbar
    {weekTitle}
    onUpdateWeekTitle={(t) => weekTitle = t}
    onOpenUpload={() => isUploadOpen = true}
    onRunScheduler={() => runScheduler(songs, settings)}
    onExportExcel={handleExportExcel}
    onLoadSampleSingleTab={handleLoadSampleSingleTab}
    onLoadSampleMultiTab={handleLoadSampleMultiTab}
    onLoadSampleMixed={handleLoadSampleMixed}
    onResetSchedule={handleResetSchedule}
    onDownloadTemplate={downloadExcelTemplate}
    {isSolving}
    onToggleSidebar={() => isMobileSidebarOpen = !isMobileSidebarOpen}
  />

  <div class="app-layout">
    <Sidebar
      {songs}
      {schedule}
      {selectedMember}
      onSelectMember={(m) => selectedMember = m}
      onUpdateSongSessions={handleUpdateSongSessions}
      onViewSongVotes={(song) => activeVoteDetailSong = song}
      onDeleteSong={handleDeleteSong}
      {settings}
      onUpdateSettings={(newSettings) => {
        settings = newSettings;
        runScheduler(songs, newSettings);
      }}
      {selectedWeekStart}
      onSelectWeek={(d) => selectedWeekStart = d}
      isOpenMobile={isMobileSidebarOpen}
      onCloseMobile={() => isMobileSidebarOpen = false}
    />

    <CalendarGrid
      {schedule}
      {songs}
      {conflicts}
      {unresolved}
      {selectedMember}
      onSelectSession={(sess) => activeSessionDetail = sess}
      onOpenSlotAdd={(day, slot) => slotAddCoord = { day, slot }}
      onOpenConflictResolver={() => isConflictResolverOpen = true}
      onLoadSampleMultiTab={handleLoadSampleMultiTab}
      onLoadSampleSingleTab={handleLoadSampleSingleTab}
      onOpenUpload={() => isUploadOpen = true}
      {selectedWeekStart}
    />
  </div>

  <!-- Progressive Enhancement Hidden Form for SvelteKit SSR Actions -->
  <form method="POST" action="?/solve" use:enhance style="display: none;">
    <input type="hidden" name="maxRooms" value={settings.maxRooms} />
    <input type="hidden" name="allowPartialAttendance" value={String(settings.allowPartialAttendance)} />
    <input type="hidden" name="spreadDays" value={String(settings.spreadDays)} />
  </form>

  <!-- Modals -->
  {#if activeSessionDetail}
    <EventDetailModal
      session={activeSessionDetail}
      onClose={() => activeSessionDetail = null}
      onDeleteSession={handleDeleteSession}
    />
  {/if}

  {#if isConflictResolverOpen}
    <ConflictResolverModal
      {unresolved}
      {conflicts}
      {songs}
      onClose={() => isConflictResolverOpen = false}
      onManualAssign={handleManualAssign}
    />
  {/if}

  {#if isUploadOpen}
    <UploadModal
      onClose={() => isUploadOpen = false}
      onAddSongs={handleAddUploadedSongs}
      existingCount={songs.length}
    />
  {/if}

  {#if sheetSelectionInspections}
    <SheetSelectionModal
      inspections={sheetSelectionInspections}
      existingCount={songs.length}
      onClose={() => sheetSelectionInspections = null}
      onConfirm={(selected) => {
        handleAddUploadedSongs(selected);
        sheetSelectionInspections = null;
      }}
    />
  {/if}

  {#if activeVoteDetailSong}
    <RawVoteModal
      song={activeVoteDetailSong}
      onClose={() => activeVoteDetailSong = null}
      onToggleVote={handleToggleVote}
    />
  {/if}

  {#if slotAddCoord}
    <SlotAddModal
      day={slotAddCoord.day}
      slot={slotAddCoord.slot}
      {songs}
      {schedule}
      onClose={() => slotAddCoord = null}
      onAssignSong={(song, day, slot) => {
        handleManualAssign(song.id, day, slot);
        slotAddCoord = null;
      }}
    />
  {/if}
</div>

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }
  .app-layout {
    display: flex;
    flex: 1;
    overflow: hidden;
    position: relative;
  }
</style>
