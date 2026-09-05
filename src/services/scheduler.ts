import {
  SongVoteData,
  ScheduledSession,
  SolverSettings,
  SolverResult,
  UnresolvedSong,
  CandidateSlot,
  ConflictItem,
  DayOfWeek,
} from '../types/timetable';
import { DAYS_OF_WEEK, DEFAULT_TIME_SLOTS } from '../constants/timetableDefaults';

interface SlotCoord {
  day: DayOfWeek;
  slot: string;
}

// Check attendance of a song in a given slot
export function getSlotAttendance(
  song: SongVoteData,
  day: DayOfWeek,
  slot: string
): {
  availableMembers: string[];
  absentMembers: string[];
  attendanceRate: number; // 0.0 to 1.0
  is100Percent: boolean;
} {
  const availableMembers: string[] = [];
  const absentMembers: string[] = [];

  for (const m of song.members) {
    const key = `${day}__${slot}__${m}`;
    const isAvail = !!song.availability[key];
    if (isAvail) {
      availableMembers.push(m);
    } else {
      absentMembers.push(m);
    }
  }

  const rate = song.members.length > 0 ? availableMembers.length / song.members.length : 1;
  return {
    availableMembers,
    absentMembers,
    attendanceRate: rate,
    is100Percent: absentMembers.length === 0,
  };
}

// Detect any conflicts in the current schedule
export function detectConflicts(
  schedule: ScheduledSession[],
  songsMap: Map<string, SongVoteData>,
  maxRooms: number
): ConflictItem[] {
  const conflicts: ConflictItem[] = [];
  const slotGroups = new Map<string, ScheduledSession[]>();

  for (const session of schedule) {
    const key = `${session.day}__${session.slot}`;
    if (!slotGroups.has(key)) {
      slotGroups.set(key, []);
    }
    slotGroups.get(key)!.push(session);
  }

  for (const [key, sessions] of slotGroups.entries()) {
    const [day, slot] = key.split('__') as [DayOfWeek, string];

    // 1. Room overflow
    if (sessions.length > maxRooms) {
      conflicts.push({
        id: `conflict-room-${key}`,
        type: 'room_overflow',
        day,
        slot,
        message: `Khung giờ ${day} (${slot}) có ${sessions.length} bài nhưng chỉ có ${maxRooms} phòng tập.`,
        songNames: sessions.map(s => s.songName),
        memberNames: [],
      });
    }

    // 2. Member double-booking across different songs
    const memberToSongs = new Map<string, string[]>();
    for (const sess of sessions) {
      for (const m of sess.allMembers) {
        if (!memberToSongs.has(m)) {
          memberToSongs.set(m, []);
        }
        memberToSongs.get(m)!.push(sess.songName);
      }
    }

    for (const [member, songsInSlot] of memberToSongs.entries()) {
      if (songsInSlot.length > 1) {
        conflicts.push({
          id: `conflict-member-${key}-${member}`,
          type: 'member_double_booked',
          day,
          slot,
          message: `Thành viên "${member}" bị trùng lịch tập ở cả ${songsInSlot.join(' và ')} vào ${day} (${slot}).`,
          songNames: songsInSlot,
          memberNames: [member],
        });
      }
    }

    // 3. Member absent check
    for (const sess of sessions) {
      if (sess.absentMembers.length > 0) {
        conflicts.push({
          id: `conflict-absent-${sess.id}`,
          type: 'member_absent',
          day,
          slot,
          message: `Bài "${sess.songName}" có ${sess.absentMembers.length} thành viên bận (${sess.absentMembers.join(', ')}).`,
          songNames: [sess.songName],
          memberNames: sess.absentMembers,
        });
      }
    }
  }

  return conflicts;
}

// Generate candidate slots for manual resolution
export function getCandidateSlotsForSong(
  song: SongVoteData,
  currentSchedule: ScheduledSession[],
  allDays: DayOfWeek[],
  allSlots: string[],
  maxRooms: number
): CandidateSlot[] {
  const candidates: CandidateSlot[] = [];

  for (const day of allDays) {
    for (const slot of allSlots) {
      const attendance = getSlotAttendance(song, day, slot);

      // Check current sessions in this slot
      const existingInSlot = currentSchedule.filter(
        s => s.day === day && s.slot === slot && s.songId !== song.id
      );

      const conflictingSongs: string[] = [];
      const conflictingMembers: string[] = [];

      for (const sess of existingInSlot) {
        const shared = sess.allMembers.filter(m => song.members.includes(m));
        if (shared.length > 0) {
          conflictingSongs.push(sess.songName);
          conflictingMembers.push(...shared);
        }
      }

      // Check room capacity
      const isRoomFull = existingInSlot.length >= maxRooms;

      // Calculate candidate score (higher is better)
      let score = attendance.attendanceRate * 100;
      if (conflictingMembers.length > 0) score -= 50 * conflictingMembers.length;
      if (isRoomFull) score -= 30;

      // Bonus if all available
      if (attendance.is100Percent) score += 20;

      candidates.push({
        day,
        slot,
        availableCount: attendance.availableMembers.length,
        totalCount: song.members.length,
        availableMembers: attendance.availableMembers,
        absentMembers: attendance.absentMembers,
        conflictingSongs,
        conflictingMembers: Array.from(new Set(conflictingMembers)),
        score,
      });
    }
  }

  // Sort candidates by score descending
  return candidates.sort((a, b) => b.score - a.score);
}

// Main Intelligent Scheduling Algorithm
export function solveTimetable(
  songs: SongVoteData[],
  settings: SolverSettings,
  days: DayOfWeek[] = DAYS_OF_WEEK,
  timeSlots: string[] = DEFAULT_TIME_SLOTS,
  manualFixedSessions: ScheduledSession[] = []
): SolverResult {
  const songsMap = new Map(songs.map(s => [s.id, s]));

  // Build all possible (day, slot) coordinates
  const allCoords: SlotCoord[] = [];
  for (const day of days) {
    for (const slot of timeSlots) {
      allCoords.push({ day, slot });
    }
  }

  // State: slotKey -> ScheduledSession[]
  const slotOccupancy = new Map<string, ScheduledSession[]>();
  for (const coord of allCoords) {
    slotOccupancy.set(`${coord.day}__${coord.slot}`, []);
  }

  // Keep manual fixed sessions
  const finalSchedule: ScheduledSession[] = [...manualFixedSessions];
  for (const sess of manualFixedSessions) {
    const key = `${sess.day}__${sess.slot}`;
    if (slotOccupancy.has(key)) {
      slotOccupancy.get(key)!.push(sess);
    }
  }

  // Count how many sessions each song still needs
  const neededSessions: { song: SongVoteData; sessionIndex: number }[] = [];
  for (const song of songs) {
    const fixedCount = manualFixedSessions.filter(s => s.songId === song.id).length;
    const remainingNeeded = Math.max(0, song.targetSessions - fixedCount);
    for (let i = 0; i < remainingNeeded; i++) {
      neededSessions.push({ song, sessionIndex: fixedCount + i + 1 });
    }
  }

  // Helper: check if a song can be placed in (day, slot)
  const canPlaceSong = (
    song: SongVoteData,
    day: DayOfWeek,
    slot: string,
    requireFullAttendance: boolean,
    alreadyAssignedDays: Set<DayOfWeek>
  ): { ok: boolean; attendance: ReturnType<typeof getSlotAttendance> } => {
    const key = `${day}__${slot}`;
    const currentInSlot = slotOccupancy.get(key) || [];

    // 1. Room limit check
    if (currentInSlot.length >= settings.maxRooms) {
      return { ok: false, attendance: getSlotAttendance(song, day, slot) };
    }

    // 2. Member conflict check with already placed songs in this slot
    for (const sess of currentInSlot) {
      const hasOverlap = sess.allMembers.some(m => song.members.includes(m));
      if (hasOverlap) {
        return { ok: false, attendance: getSlotAttendance(song, day, slot) };
      }
    }

    // 3. Spread days check (prefer distinct days for multiple sessions of the same song)
    if (settings.spreadDays && alreadyAssignedDays.has(day)) {
      return { ok: false, attendance: getSlotAttendance(song, day, slot) };
    }

    // 4. Attendance check
    const attendance = getSlotAttendance(song, day, slot);
    if (requireFullAttendance) {
      if (!attendance.is100Percent) return { ok: false, attendance };
    } else {
      // Partial attendance: allow at most 1 missing member
      if (attendance.absentMembers.length > 1) return { ok: false, attendance };
    }

    return { ok: true, attendance };
  };

  // Pre-calculate candidate count per song (MRV heuristic: Most Constrained Variable First)
  const songCandidateCount = new Map<string, number>();
  for (const song of songs) {
    let perfectCount = 0;
    for (const coord of allCoords) {
      const att = getSlotAttendance(song, coord.day, coord.slot);
      if (att.is100Percent) perfectCount++;
    }
    songCandidateCount.set(song.id, perfectCount);
  }

  // Sort sessions to schedule:
  // 1. Songs with FEWEST candidate slots first (MRV)
  // 2. Songs with MORE members first (harder to satisfy)
  neededSessions.sort((a, b) => {
    const candA = songCandidateCount.get(a.song.id) || 0;
    const candB = songCandidateCount.get(b.song.id) || 0;
    if (candA !== candB) return candA - candB;
    return b.song.members.length - a.song.members.length;
  });

  // Track assigned days per song
  const songAssignedDays = new Map<string, Set<DayOfWeek>>();
  for (const sess of manualFixedSessions) {
    if (!songAssignedDays.has(sess.songId)) {
      songAssignedDays.set(sess.songId, new Set());
    }
    songAssignedDays.get(sess.songId)!.add(sess.day);
  }

  // Multi-pass scheduling algorithm:
  // Pass 1: Strict mode (100% attendance required for all songs)
  // Pass 2: If any remaining, and allowPartialAttendance is true, place with max attendance
  const unresolvedSessions: typeof neededSessions = [];

  for (const item of neededSessions) {
    const { song } = item;
    if (!songAssignedDays.has(song.id)) {
      songAssignedDays.set(song.id, new Set());
    }
    const assignedDays = songAssignedDays.get(song.id)!;

    // Find best slot for this song
    let bestCoord: SlotCoord | null = null;
    let bestAttendance: ReturnType<typeof getSlotAttendance> | null = null;
    let bestScore = -Infinity;

    for (const coord of allCoords) {
      // Pass 1: Try strict 100% attendance
      const test = canPlaceSong(song, coord.day, coord.slot, true, assignedDays);
      if (test.ok) {
        // Calculate heuristic score:
        // Prefer slots with lower overall demand from other songs (Least Constraining Value)
        let otherDemand = 0;
        for (const other of songs) {
          if (other.id !== song.id && getSlotAttendance(other, coord.day, coord.slot).is100Percent) {
            otherDemand++;
          }
        }
        const score = 100 - otherDemand;
        if (score > bestScore) {
          bestScore = score;
          bestCoord = coord;
          bestAttendance = test.attendance;
        }
      }
    }

    // Fallback relaxation if strict 100% was not found
    if (!bestCoord && settings.allowPartialAttendance) {
      for (const coord of allCoords) {
        const test = canPlaceSong(song, coord.day, coord.slot, false, assignedDays);
        if (test.ok) {
          const score = (test.attendance.availableMembers.length / song.members.length) * 80;
          if (score > bestScore) {
            bestScore = score;
            bestCoord = coord;
            bestAttendance = test.attendance;
          }
        }
      }
    }

    // If still not found and spreadDays was on, try relaxing spreadDays for this session
    if (!bestCoord) {
      for (const coord of allCoords) {
        const test = canPlaceSong(song, coord.day, coord.slot, true, new Set());
        if (test.ok) {
          bestCoord = coord;
          bestAttendance = test.attendance;
          break;
        }
      }
    }

    if (bestCoord && bestAttendance) {
      const key = `${bestCoord.day}__${bestCoord.slot}`;
      const currentInSlot = slotOccupancy.get(key) || [];
      const roomNumber = currentInSlot.length + 1;

      const newSession: ScheduledSession = {
        id: `sess-${song.id}-${Date.now()}-${Math.random().toString(36).substr(2, 5)}`,
        songId: song.id,
        songName: song.name,
        day: bestCoord.day,
        slot: bestCoord.slot,
        room: roomNumber,
        allMembers: song.members,
        availableMembers: bestAttendance.availableMembers,
        absentMembers: bestAttendance.absentMembers,
        color: song.color,
        note: song.notes[key] || '',
        isManual: false,
      };

      currentInSlot.push(newSession);
      slotOccupancy.set(key, currentInSlot);
      finalSchedule.push(newSession);
      assignedDays.add(bestCoord.day);
    } else {
      // Could not automatically resolve! Save for user manual configuration
      unresolvedSessions.push(item);
    }
  }

  // Build Unresolved Song list
  const unresolvedGrouped = new Map<string, { song: SongVoteData; count: number }>();
  for (const item of unresolvedSessions) {
    const existing = unresolvedGrouped.get(item.song.id);
    if (existing) {
      existing.count++;
    } else {
      unresolvedGrouped.set(item.song.id, { song: item.song, count: 1 });
    }
  }

  const unresolved: UnresolvedSong[] = [];
  for (const [songId, val] of unresolvedGrouped.entries()) {
    const song = val.song;
    const assignedCount = finalSchedule.filter(s => s.songId === songId).length;
    const candidates = getCandidateSlotsForSong(song, finalSchedule, days, timeSlots, settings.maxRooms);

    const reasons: string[] = [];
    if (candidates.every(c => c.conflictingMembers.length > 0)) {
      reasons.push('Tất cả khung giờ rảnh đều bị trùng thành viên với bài khác đã xếp.');
    } else if (candidates.every(c => c.absentMembers.length > 0)) {
      reasons.push('Không có khung giờ nào đạt 100% thành viên rảnh.');
    } else {
      reasons.push('Hết phòng tập hoặc bị trùng lịch với bài khác.');
    }

    unresolved.push({
      songId: song.id,
      songName: song.name,
      targetSessions: song.targetSessions,
      assignedSessions: assignedCount,
      reasons,
      candidates: candidates.slice(0, 8), // Top 8 suggestions
    });
  }

  // Detect any conflicts in the final schedule
  const conflicts = detectConflicts(finalSchedule, songsMap, settings.maxRooms);

  // Stats
  let perfectCount = 0;
  let partialCount = 0;
  for (const s of finalSchedule) {
    if (s.absentMembers.length === 0) perfectCount++;
    else partialCount++;
  }

  const totalRequested = songs.reduce((acc, s) => acc + s.targetSessions, 0);

  return {
    schedule: finalSchedule,
    unresolved,
    conflicts,
    stats: {
      totalRequested,
      totalScheduled: finalSchedule.length,
      perfectAttendanceCount: perfectCount,
      partialAttendanceCount: partialCount,
    },
  };
}
