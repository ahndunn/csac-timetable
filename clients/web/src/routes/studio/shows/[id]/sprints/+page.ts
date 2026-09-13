import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const showId = params.id || 'e0000000-0000-0000-0000-000000000001';
  try {
    const [sprintData, historyData] = await Promise.all([
      api.shows.getActiveSprint(showId, fetch),
      api.shows.getSprintHistory(showId, 'b0000000-0000-0000-0000-000000000001', fetch).catch(() => null),
    ]);

    return {
      sprintData,
      historyData,
    };
  } catch (err: any) {
    console.error('Failed to load show sprint data:', err);
    throw error(err.status || 500, err.message || 'Failed to load sprint timetable data');
  }
};
