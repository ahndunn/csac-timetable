import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const showId = params.id;
  if (!showId) {
    throw error(400, 'Show ID is required');
  }
  try {
    // Load sprint data first so we can use the real sprint UUID for history fetch.
    // Previously used a hardcoded seeded UUID which would fail for different environments.
    const sprintData = await api.shows.getActiveSprint(showId, fetch);
    const sprintId: string | undefined = sprintData?.sprint?.id;

    const historyData = sprintId
      ? await api.shows.getSprintHistory(showId, sprintId, fetch).catch(() => null)
      : null;

    return {
      sprintData,
      historyData,
    };
  } catch (err: any) {
    console.error('Failed to load show sprint data:', err);
    throw error(err.status || 500, err.message || 'Failed to load sprint timetable data');
  }
};
