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

export const DEFAULT_USER: UserSession = {
  id: 'user-001',
  email: 'admin@csac.local',
  fullName: 'Administrator',
  role: 'admin',
};
