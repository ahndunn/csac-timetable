import type { PageLoad } from './$types';
import { api } from '$lib/api/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const showId = params.id || 'show-2026-annual';
  try {
    const [sprintData, historyData] = await Promise.all([
      api.shows.getActiveSprint(showId, fetch).catch(() => null),
      api.shows.getSprintHistory(showId, 'sprint-3', fetch).catch(() => null),
    ]);

    return {
      sprintData,
      historyData,
    };
  } catch (err) {
    console.error('Failed to load show sprint data:', err);
    return {
      sprintData: null,
      historyData: null,
    };
  }
};
