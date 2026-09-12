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
  targetSessions: number;
  sourceFileName?: string;
}

export interface ScheduledSession {
  id: string;
  songId: string;
  songName: string;
  day: DayOfWeek;
  slot: string;
  room: number;
  allMembers: string[];
  availableMembers: string[];
  absentMembers: string[];
  color: PastelColor;
  note?: string;
  isManual?: boolean;
}

export interface SolverSettings {
  maxRooms: number;
  allowPartialAttendance: boolean;
  spreadDays: boolean;
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

// Show Management & Resource Allocation Types
export type MusicNumberStatus = 
  | 'draft' 
  | 'in_practice' 
  | 'ready_for_qc' 
  | 'qc_approved' 
  | 'stage_ready';

export type BandRole = 
  | 'vocal_lead' 
  | 'vocal_harmony' 
  | 'guitar_lead' 
  | 'guitar_rhythm' 
  | 'bass' 
  | 'keys' 
  | 'drums' 
  | 'percussion' 
  | 'sound_tech';

export interface QCVerdict {
  reviewedBy: string;
  reviewedAt: string;
  decision: 'passed' | 'revision_requested';
  feedbackNotes: string;
  actionItems?: string[];
}

export type ShowRole = 'DM' | 'PM' | 'QC' | 'Performer';

// Scoped Role Assignments attached by key
export interface UserShowScope {
  userId: string;
  showId: string;
  isDM: boolean;
}

export interface UserNumberScope {
  userId: string;
  showId: string;
  numberId: string;
  role: 'pm' | 'qc' | 'performer';
  instrumentRole?: BandRole;
}

export interface ShowRosterMember {
  id: string;
  userId: string;
  fullName: string;
  email: string;
  phone?: string;
  showRole: ShowRole;
  isDM?: boolean;
  pmSongTitles?: string[];
  qcSongTitles?: string[];
  primaryInstrument: BandRole;
  secondaryInstruments: BandRole[];
  assignedSongCount: number;
  assignedSongTitles: string[];
  totalPracticeHours: number;
  workloadStatus: 'optimal' | 'moderate' | 'fatigued';
  attendanceRate: number; // e.g. 95
  joinedAt: string;
}

export interface RosterMember {
  userId: string;
  fullName: string;
  email: string;
  primaryRole: BandRole;
  assignedNumberIds: string[];
  totalPracticeHours: number;
  workloadStatus: 'optimal' | 'moderate' | 'fatigued';
}

export interface MusicNumber {
  id: string;
  showId: string;
  title: string;
  originalArtist: string;
  status: MusicNumberStatus;
  rolesRequired: BandRole[];
  assignedMembers: Record<string, string>;
  qcHistory: QCVerdict[];
  notes?: string;
}

// User Roles & Hierarchy
export type UserRole = 'admin' | 'moderator' | 'dm' | 'pm' | 'qc' | 'member';
export type UserStatus = 'active' | 'pending_activation' | 'suspended';

export interface UserSession {
  id: string;
  email: string;
  fullName: string;
  role: UserRole;
  status?: UserStatus;
  authEpoch?: number;
}

export interface UserAccount {
  id: string;
  email: string;
  full_name: string;
  role: 'admin' | 'moderator' | 'member';
  status: UserStatus;
  created_at: string;
  auth_epoch?: number;
  phone?: string;
}

export interface UserInvitationPayload {
  email: string;
  full_name?: string;
  role?: 'admin' | 'moderator' | 'member';
  show_id?: string;
  phone?: string;
}

export interface ActivationVerificationPayload {
  token?: string;
  email?: string;
  otp: string;
}

export interface ActivationVerificationResponse {
  activation_session_id: string;
  email: string;
  prefilled_data: {
    full_name?: string;
    phone?: string;
  };
}

export interface ActivationCompletionPayload {
  activation_session_id: string;
  password: string;
  full_name: string;
  phone?: string;
}

export interface EffectiveShowPermissions {
  showId: string;
  isDM: boolean;
  pmSongIds: string[];
  qcSongIds: string[];
  castSongIds: string[];
}

// Audit & Compute History Interfaces
export interface AvailabilityHistoryItem {
  id: string;
  sprintId: string;
  userId: string;
  userName: string;
  actorId: string;
  actorName: string;
  action: 'ADD' | 'UPDATE' | 'DELETE';
  dayOfWeek: string;
  slotLabel: string;
  isAvailable: boolean;
  createdAt: string;
}

export interface ScheduleRunHistoryItem {
  id: string;
  sprintId: string;
  triggeredBy: string;
  triggeredByName: string;
  status: 'queued' | 'processing' | 'completed' | 'failed';
  durationMs: number;
  score: number;
  conflictCount?: number;
  error?: string;
  createdAt: string;
  completedAt?: string;
}

// Gear & Instrument Fleet Domain Types
export type GearCategory = 'strings' | 'keys' | 'drums' | 'amps_cabs' | 'pedals_fx' | 'audio_di' | 'cables_accessories';
export type GearOwnershipType = 'club_property' | 'member_owned';
export type GearLendingPolicy = 'open_to_all' | 'approval_required' | 'show_only' | 'locked_private';
export type GearAvailabilityStatus = 'free_to_borrow' | 'in_use' | 'unavailable' | 'in_maintenance';

export type GearShowStatus = 
  | 'allocated_main'       // Assigned to a specific music number
  | 'allocated_backup'     // Backup / spare
  | 'checked_in_venue'     // Confirmed present at venue
  | 'active_stage'         // On stage / plugged in
  | 'retrieved_owner'      // Safely retrieved by owner
  | 'retrieved_proxy'      // Retrieved by someone else on behalf
  | 'orphan'               // Left behind / unclaimed
  | 'adopted';             // Temporary custody adopted by peer

export interface GearItem {
  id: string;
  name: string;
  code?: string;
  category: GearCategory;
  ownership: GearOwnershipType;
  ownerId?: string;
  ownerName?: string;
  custodianName: string;
  custodianId?: string;
  locationNote: string;
  status: GearAvailabilityStatus;
  lendingPolicy: GearLendingPolicy;
  isRevoked?: boolean;
  estimatedValueVND?: number;
  serialNumber?: string;
  notes?: string;
}

export interface ShowGearAllocation {
  id: string;
  showId: string;
  gearId: string;
  gearName: string;
  category: GearCategory;
  ownership: GearOwnershipType;
  ownerName?: string;
  allocatedFor: 'music_number' | 'backline_common' | 'emergency_backup' | 'sound_desk';
  musicNumberTitle?: string;
  primaryPerformerName?: string;
  status: GearShowStatus;
  checkInTimestamp?: string;
  retrievalTimestamp?: string;
  retrievedByName?: string;
  isOnBehalfRetrieval: boolean;
  retrievalNote?: string;
  adoption?: {
    adopterName: string;
    adopterPhone: string;
    adoptedAt: string;
    targetReturnDate: string;
    ownerNotified: boolean;
    pickupLocationNote?: string;
  };
}


