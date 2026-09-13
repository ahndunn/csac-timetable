import type { LayoutServerLoad } from './$types';
import type { UserRole, UserSession } from '$lib/types/timetable';

export const load: LayoutServerLoad = async ({ cookies }) => {
  const cookieRole = cookies.get('csac_role') as UserRole | null;
  const cookieToken = cookies.get('csac_token');
  const validRoles: UserRole[] = ['member', 'qc', 'pm', 'dm', 'moderator', 'admin'];
  const activeRole: UserRole = cookieRole && validRoles.includes(cookieRole) ? cookieRole : 'member';

  const user: UserSession | null = cookieToken
    ? {
        id: 'user-001',
        email: activeRole === 'admin' ? 'admin@csac.local' : 'member@csac.local',
        fullName: activeRole === 'admin' ? 'System Administrator' : 'CSAC Performer',
        role: activeRole,
      }
    : null;

  return {
    user,
    role: activeRole,
  };
};
