import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api/client';
import type { UserAccount } from '$lib/types/timetable';

export const load: PageLoad = async ({ fetch }) => {
  try {
    const res = await api.users.list(fetch);
    const users: UserAccount[] = res?.users || [];
    return { users };
  } catch (err: any) {
    console.error('Failed to load users in admin:', err);
    throw error(err.status || 500, err.message || 'Failed to load user accounts');
  }
};
