import type { LayoutServerLoad } from './$types';
import type { UserRole, UserSession } from '$lib/types/timetable';

export const load: LayoutServerLoad = async ({ url, cookies }) => {
  // Allow demo role switching via query parameter ?role= or cookie 'csac_role'
  const roleParam = url.searchParams.get('role') as UserRole | null;
  const cookieRole = cookies.get('csac_role') as UserRole | null;

  let activeRole: UserRole = 'admin';

  const validRoles: UserRole[] = ['member', 'qc', 'pm', 'dm', 'moderator', 'admin'];

  if (roleParam && validRoles.includes(roleParam)) {
    activeRole = roleParam;
    cookies.set('csac_role', roleParam, { path: '/', httpOnly: false });
  } else if (cookieRole && validRoles.includes(cookieRole)) {
    activeRole = cookieRole;
  }

  const roleLabels: Record<UserRole, string> = {
    member: 'Member (Nhạc công)',
    qc: 'QC Auditor (Kiểm định viên)',
    pm: 'Performance Manager (PM)',
    dm: 'Delivery Manager (DM)',
    moderator: 'Moderator (Điều hành viên)',
    admin: 'System Administrator (Admin)',
  };

  const user: UserSession = {
    id: 'user-001',
    email: `${activeRole}@csac.local`,
    fullName: `CSAC ${roleLabels[activeRole] || activeRole}`,
    role: activeRole,
  };

  return {
    user,
    role: activeRole,
  };
};
