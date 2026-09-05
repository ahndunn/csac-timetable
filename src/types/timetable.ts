export type DayOfWeek = 
  | 'THỨ HAI' 
  | 'THỨ BA' 
  | 'THỨ TƯ' 
  | 'THỨ NĂM' 
  | 'THỨ SÁU' 
  | 'THỨ BẢY' 
  | 'CHỦ NHẬT';

export interface PastelColor {
  id: string;
  name: string;
  bg: string;
  border: string;
  text: string;
  chipBg: string;
}

export const PASTEL_PALETTE: PastelColor[] = [
  { id: 'sky', name: 'Xanh Da Trời', bg: '#e0f2fe', border: '#7dd3fc', text: '#0369a1', chipBg: '#bae6fd' },
  { id: 'mint', name: 'Bạc Hà', bg: '#dcfce7', border: '#86efac', text: '#15803d', chipBg: '#bbf7d0' },
  { id: 'lavender', name: 'Oải Hương', bg: '#ede9fe', border: '#c4b5fd', text: '#6d28d9', chipBg: '#ddd6fe' },
  { id: 'peach', name: 'Hồng Đào', bg: '#ffedd5', border: '#fdba74', text: '#c2410c', chipBg: '#fed7aa' },
  { id: 'rose', name: 'Hoa Hồng', bg: '#ffe4e6', border: '#fda4af', text: '#be123c', chipBg: '#fecdd3' },
  { id: 'lemon', name: 'Vàng Nhạt', bg: '#fef9c3', border: '#fde047', text: '#a16207', chipBg: '#fef08a' },
  { id: 'lilac', name: 'Tím Phấn', bg: '#f3e8ff', border: '#d8b4fe', text: '#7e22ce', chipBg: '#e9d5ff' },
  { id: 'teal', name: 'Xanh Ngọc Nhạt', bg: '#ccfbf1', border: '#5eead4', text: '#0f766e', chipBg: '#99f6e4' },
  { id: 'coral', name: 'Hồng Phấn', bg: '#fae8ff', border: '#f0abfc', text: '#a21caf', chipBg: '#f5d0fe' },
  { id: 'sand', name: 'Cát Mộc', bg: '#f5f5f4', border: '#d6d3d1', text: '#44403c', chipBg: '#e7e5e4' },
];

export interface SongVoteData {
  id: string;
  name: string;
  weekTitle: string;
  members: string[];
  // key: `${day}__${slot}__${member}` -> boolean
  availability: Record<string, boolean>;
  // key: `${day}__${slot}` -> note string
  notes: Record<string, string>;
  color: PastelColor;
  targetSessions: number; // e.g. 1, 2, 3 times a week (user requested configurable)
  sourceFileName?: string;
}

export interface ScheduledSession {
  id: string;
  songId: string;
  songName: string;
  day: DayOfWeek;
  slot: string; // e.g. '17h - 18h'
  room: number; // 1, 2, etc.
  allMembers: string[];
  availableMembers: string[];
  absentMembers: string[];
  color: PastelColor;
  note?: string;
  isManual?: boolean;
}

export interface SolverSettings {
  maxRooms: number; // default 1 room, configurable to 2+
  allowPartialAttendance: boolean; // allow scheduling if 1 member is absent when 100% is impossible
  spreadDays: boolean; // if a song has multiple sessions, don't schedule on the same day
}

export interface CandidateSlot {
  day: DayOfWeek;
  slot: string;
  availableCount: number;
  totalCount: number;
  availableMembers: string[];
  absentMembers: string[];
  conflictingSongs: string[];
  conflictingMembers: string[];
  score: number;
}

export interface UnresolvedSong {
  songId: string;
  songName: string;
  targetSessions: number;
  assignedSessions: number;
  reasons: string[];
  candidates: CandidateSlot[];
}

export interface ConflictItem {
  id: string;
  type: 'member_double_booked' | 'member_absent' | 'room_overflow';
  day: DayOfWeek;
  slot: string;
  message: string;
  songNames: string[];
  memberNames: string[];
}

export interface SolverResult {
  schedule: ScheduledSession[];
  unresolved: UnresolvedSong[];
  conflicts: ConflictItem[];
  stats: {
    totalRequested: number;
    totalScheduled: number;
    perfectAttendanceCount: number;
    partialAttendanceCount: number;
  };
}
