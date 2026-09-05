import React, { useState, useEffect, useCallback } from 'react';
import {
  SongVoteData,
  ScheduledSession,
  SolverSettings,
  ConflictItem,
  UnresolvedSong,
  DayOfWeek,
} from './types/timetable';
import { DEFAULT_WEEK_TITLE, DAYS_OF_WEEK, DEFAULT_TIME_SLOTS } from './constants/timetableDefaults';
import { generateSampleSongs } from './services/sampleData';
import { solveTimetable, detectConflicts, getSlotAttendance } from './services/scheduler';
import { exportTimetableToExcel, downloadExcelTemplate } from './services/excelExporter';
import { formatWeekRange } from './utils/dateUtils';

import { Navbar } from './components/Navbar';
import { Sidebar } from './components/Sidebar';
import { CalendarGrid } from './components/CalendarGrid';
import { EventDetailModal } from './components/EventDetailModal';
import { ConflictResolverModal } from './components/ConflictResolverModal';
import { UploadModal } from './components/UploadModal';
import { RawVoteModal } from './components/RawVoteModal';
import { SlotAddModal } from './components/SlotAddModal';

export const App: React.FC = () => {
  // State: Selected week Monday date (Default: Monday 07/09/2026)
  const [selectedWeekStart, setSelectedWeekStart] = useState<Date>(() => new Date(2026, 8, 7));

  // State: Songs (Initially empty as requested by user; only loaded when user clicks Load Sample)
  const [songs, setSongs] = useState<SongVoteData[]>([]);
  const [weekTitle, setWeekTitle] = useState<string>(DEFAULT_WEEK_TITLE);

  // State: Timetable Schedule
  const [schedule, setSchedule] = useState<ScheduledSession[]>([]);
  const [conflicts, setConflicts] = useState<ConflictItem[]>([]);
  const [unresolved, setUnresolved] = useState<UnresolvedSong[]>([]);
  const [isSolving, setIsSolving] = useState(false);

  // State: Settings
  const [settings, setSettings] = useState<SolverSettings>({
    maxRooms: 1, // Standard 1 practice room default
    allowPartialAttendance: false, // Strict 100% attendance preferred
    spreadDays: true, // Spread multiple sessions of a song on distinct days
  });

  // State: UI filters & selections
  const [selectedMember, setSelectedMember] = useState<string | null>(null);

  // State: Modals
  const [isUploadOpen, setIsUploadOpen] = useState(false);
  const [activeSessionDetail, setActiveSessionDetail] = useState<ScheduledSession | null>(null);
  const [activeVoteDetailSong, setActiveVoteDetailSong] = useState<SongVoteData | null>(null);
  const [isConflictResolverOpen, setIsConflictResolverOpen] = useState(false);
  const [slotAddCoord, setSlotAddCoord] = useState<{ day: DayOfWeek; slot: string } | null>(null);

  // Auto-solve scheduler function
  const runScheduler = useCallback(
    (songsToSolve: SongVoteData[] = songs, currentSettings: SolverSettings = settings) => {
      if (songsToSolve.length === 0) {
        setSchedule([]);
        setUnresolved([]);
        setConflicts([]);
        return;
      }
      setIsSolving(true);
      setTimeout(() => {
        const result = solveTimetable(songsToSolve, currentSettings);
        setSchedule(result.schedule);
        setUnresolved(result.unresolved);
        setConflicts(result.conflicts);
        setIsSolving(false);

        // Auto-open conflict resolver if unresolved songs exist
        if (result.unresolved.length > 0) {
          setIsConflictResolverOpen(true);
        }
      }, 100);
    },
    [songs, settings]
  );

  // Update song's target frequency
  const handleUpdateSongSessions = (songId: string, newTarget: number) => {
    setSongs(prev =>
      prev.map(s => (s.id === songId ? { ...s, targetSessions: newTarget } : s))
    );
  };

  // Delete song
  const handleDeleteSong = (songId: string) => {
    setSongs(prev => prev.filter(s => s.id !== songId));
    setSchedule(prev => prev.filter(s => s.songId !== songId));
  };

  // Delete individual session from calendar
  const handleDeleteSession = (sessionId: string) => {
    setSchedule(prev => {
      const updated = prev.filter(s => s.id !== sessionId);
      const songsMap = new Map(songs.map(s => [s.id, s]));
      setConflicts(detectConflicts(updated, songsMap, settings.maxRooms));
      return updated;
    });
    setActiveSessionDetail(null);
  };

  // Add new songs from uploaded Excel
  const handleAddUploadedSongs = (newSongs: SongVoteData[]) => {
    setSongs(prev => [...prev, ...newSongs]);
    if (newSongs[0]?.weekTitle) {
      setWeekTitle(newSongs[0].weekTitle);
    }
    // Auto re-solve with all songs
    runScheduler([...songs, ...newSongs], settings);
  };

  // Load sample dataset
  const handleLoadSample = () => {
    const samples = generateSampleSongs();
    setSongs(samples);
    setWeekTitle(DEFAULT_WEEK_TITLE);
    runScheduler(samples, settings);
  };

  // Clear schedule
  const handleResetSchedule = () => {
    setSchedule([]);
    setConflicts([]);
    setUnresolved([]);
  };

  // Export to Excel
  const handleExportExcel = async () => {
    if (schedule.length === 0) {
      alert('Chưa có buổi tập nào trên lịch để xuất! Hãy bấm "Tự động xếp lịch" trước.');
      return;
    }
    await exportTimetableToExcel(schedule, songs, weekTitle);
  };

  // Manual assignment from candidate slots or empty slot
  const handleManualAssign = (songId: string, day: DayOfWeek, slot: string) => {
    const song = songs.find(s => s.id === songId);
    if (!song) return;

    const key = `${day}__${slot}`;
    const currentInSlot = schedule.filter(s => s.day === day && s.slot === slot);
    const room = currentInSlot.length + 1;
    const attendance = getSlotAttendance(song, day, slot);

    const newSession: ScheduledSession = {
      id: `sess-${song.id}-${Date.now()}`,
      songId: song.id,
      songName: song.name,
      day,
      slot,
      room,
      allMembers: song.members,
      availableMembers: attendance.availableMembers,
      absentMembers: attendance.absentMembers,
      color: song.color,
      note: song.notes[key] || '',
      isManual: true,
    };

    const newSchedule = [...schedule, newSession];
    setSchedule(newSchedule);

    // Re-evaluate conflicts
    const songsMap = new Map(songs.map(s => [s.id, s]));
    setConflicts(detectConflicts(newSchedule, songsMap, settings.maxRooms));

    // Update unresolved list
    setUnresolved(prev =>
      prev
        .map(u => {
          if (u.songId === songId) {
            const newCount = u.assignedSessions + 1;
            return { ...u, assignedSessions: newCount };
          }
          return u;
        })
        .filter(u => u.assignedSessions < u.targetSessions)
    );
  };

  // Toggle vote in Raw Vote Matrix
  const handleToggleVote = (songId: string, day: DayOfWeek, slot: string, member: string) => {
    setSongs(prev =>
      prev.map(s => {
        if (s.id !== songId) return s;
        const key = `${day}__${slot}__${member}`;
        const current = !!s.availability[key];
        return {
          ...s,
          availability: {
            ...s.availability,
            [key]: !current,
          },
        };
      })
    );
  };

  // Download Excel sample files matching the 5 sample songs
  const handleDownloadTemplate = () => {
    downloadExcelTemplate();
  };

  // Select week from mini calendar
  const handleSelectWeek = (monday: Date) => {
    setSelectedWeekStart(monday);
    const newRange = formatWeekRange(monday);
    setWeekTitle(prev => {
      // If title contains date range like (07/09/2026 - 12/09/2026), update it
      if (/\(\d{2}\/\d{2}\/\d{4}\s*-\s*\d{2}\/\d{2}\/\d{4}\)/.test(prev)) {
        return prev.replace(/\(\d{2}\/\d{2}\/\d{4}\s*-\s*\d{2}\/\d{2}\/\d{4}\)/, `(${newRange})`);
      } else if (/\(\d{2}\/\d{2}\s*-\s*\d{2}\/\d{2}\/\d{4}\)/.test(prev)) {
        return prev.replace(/\(\d{2}\/\d{2}\s*-\s*\d{2}\/\d{2}\/\d{4}\)/, `(${newRange})`);
      } else {
        return `VOTE LỊCH TẬP TUẦN (${newRange})`;
      }
    });
  };

  return (
    <>
      <Navbar
        weekTitle={weekTitle}
        onUpdateWeekTitle={setWeekTitle}
        onOpenUpload={() => setIsUploadOpen(true)}
        onRunScheduler={() => runScheduler(songs, settings)}
        onExportExcel={handleExportExcel}
        onLoadSample={handleLoadSample}
        onResetSchedule={handleResetSchedule}
        onDownloadTemplate={handleDownloadTemplate}
        isSolving={isSolving}
      />

      <div className="app-container">
        <Sidebar
          songs={songs}
          schedule={schedule}
          selectedMember={selectedMember}
          onSelectMember={setSelectedMember}
          onUpdateSongSessions={handleUpdateSongSessions}
          onViewSongVotes={setActiveVoteDetailSong}
          onDeleteSong={handleDeleteSong}
          settings={settings}
          onUpdateSettings={newSettings => {
            setSettings(newSettings);
            runScheduler(songs, newSettings);
          }}
          selectedWeekStart={selectedWeekStart}
          onSelectWeek={handleSelectWeek}
        />

        <CalendarGrid
          schedule={schedule}
          songs={songs}
          conflicts={conflicts}
          unresolved={unresolved}
          selectedMember={selectedMember}
          onSelectSession={setActiveSessionDetail}
          onOpenSlotAdd={(day, slot) => setSlotAddCoord({ day, slot })}
          onOpenConflictResolver={() => setIsConflictResolverOpen(true)}
          onLoadSample={handleLoadSample}
          onOpenUpload={() => setIsUploadOpen(true)}
          onDownloadTemplate={handleDownloadTemplate}
          selectedWeekStart={selectedWeekStart}
        />
      </div>

      {/* Modals */}
      {isUploadOpen && (
        <UploadModal
          onClose={() => setIsUploadOpen(false)}
          onAddSongs={handleAddUploadedSongs}
          onDownloadTemplate={handleDownloadTemplate}
          existingCount={songs.length}
        />
      )}

      {activeSessionDetail && (
        <EventDetailModal
          session={activeSessionDetail}
          songData={songs.find(s => s.id === activeSessionDetail.songId)}
          onClose={() => setActiveSessionDetail(null)}
          onDeleteSession={handleDeleteSession}
        />
      )}

      {isConflictResolverOpen && (
        <ConflictResolverModal
          unresolved={unresolved}
          conflicts={conflicts}
          songs={songs}
          onClose={() => setIsConflictResolverOpen(false)}
          onManualAssign={handleManualAssign}
        />
      )}

      {activeVoteDetailSong && (
        <RawVoteModal
          song={activeVoteDetailSong}
          onClose={() => setActiveVoteDetailSong(null)}
          onToggleVote={handleToggleVote}
        />
      )}

      {slotAddCoord && (
        <SlotAddModal
          day={slotAddCoord.day}
          slot={slotAddCoord.slot}
          songs={songs}
          schedule={schedule}
          onClose={() => setSlotAddCoord(null)}
          onAssignSong={(song, day, slot) => handleManualAssign(song.id, day, slot)}
        />
      )}
    </>
  );
};

export default App;
