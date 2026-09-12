import type { UserRole, UserSession } from './types/timetable';

export const ROLE_RANKS: Record<UserRole, number> = {
  member: 1,
  qc: 2,
  pm: 3,
  dm: 4,
  moderator: 5,
  admin: 6,
};

/**
 * Checks if the user's role satisfies or exceeds the required role level in the superset hierarchy.
 */
export function hasRole(userRole: UserRole | undefined | null, requiredRole: UserRole): boolean {
  if (!userRole) return false;
  const userRank = ROLE_RANKS[userRole] ?? 0;
  const requiredRank = ROLE_RANKS[requiredRole] ?? 0;
  return userRank >= requiredRank;
}

export function canTriggerScheduler(role: UserRole | undefined | null): boolean {
  return hasRole(role, 'dm');
}

export function canManageNumbers(role: UserRole | undefined | null): boolean {
  return hasRole(role, 'pm');
}

export function canReviewQC(role: UserRole | undefined | null): boolean {
  return hasRole(role, 'qc');
}

export function canAccessAdmin(role: UserRole | undefined | null): boolean {
  return hasRole(role, 'moderator');
}

export function canViewHistory(role: UserRole | undefined | null): boolean {
  return hasRole(role, 'pm');
}

export function canManageShowRoster(role: UserRole | undefined | null): boolean {
  return hasRole(role, 'dm');
}

export function canEditPerformerProfile(role: UserRole | undefined | null): boolean {
  return hasRole(role, 'pm');
}

/**
 * Resolves the effective role for a user in a given show/song context.
 * High-level roles (admin, moderator) default across all shows/numbers.
 * DM is attached to (user, show).
 * PM / QC / Performer are attached to (user, show, music_number).
 */
export function getEffectiveRole(
  globalRole: UserRole | undefined | null,
  isDMForShow: boolean = false,
  isPMForSong: boolean = false,
  isQCForSong: boolean = false
): UserRole {
  if (globalRole === 'admin' || globalRole === 'moderator') {
    return globalRole;
  }
  if (isDMForShow || globalRole === 'dm') {
    return 'dm';
  }
  if (isPMForSong || globalRole === 'pm') {
    return 'pm';
  }
  if (isQCForSong || globalRole === 'qc') {
    return 'qc';
  }
  return 'member';
}

/**
 * Checks if user can manage the show (Admin, Moderator, or Show DM).
 */
export function canManageShowScoped(
  globalRole: UserRole | undefined | null,
  isDMForShow: boolean = false
): boolean {
  if (hasRole(globalRole, 'moderator')) return true;
  return isDMForShow || globalRole === 'dm';
}

/**
 * Checks if user can manage a specific song's lineup/progress (Admin, Mod, Show DM, or this Song's PM).
 */
export function canManageSongScoped(
  globalRole: UserRole | undefined | null,
  isDMForShow: boolean = false,
  isPMForSong: boolean = false
): boolean {
  if (canManageShowScoped(globalRole, isDMForShow)) return true;
  return isPMForSong || globalRole === 'pm';
}

/**
 * Checks if user can review QC for a specific song (Admin, Mod, Show DM, or this Song's assigned QC).
 */
export function canAuditSongScoped(
  globalRole: UserRole | undefined | null,
  isDMForShow: boolean = false,
  isQCForSong: boolean = false
): boolean {
  if (canManageShowScoped(globalRole, isDMForShow)) return true;
  return isQCForSong || globalRole === 'qc';
}

export const DEFAULT_USER: UserSession = {
  id: 'user-001',
  email: 'admin@csac.local',
  fullName: 'Administrator',
  role: 'admin',
};
