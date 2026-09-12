import type { PageLoad } from './$types';
import { api } from '$lib/api/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const showId = params.id || 'show-2026-annual';
  try {
    const overview = await api.shows.getOverview(showId, fetch);
    return { overview };
  } catch (err) {
    console.error('Failed to load show overview:', err);
    return {
      overview: {
        id: showId,
        title: 'CSAC Annual Concert 2026',
        venue: 'CSAC Main Auditorium',
        dates: 'Oct 1 - Oct 15, 2026',
        readiness_percent: 75,
        total_numbers: 12,
        total_hours: 48,
        qc_approved_count: 9,
        highlights: [],
        milestones: [],
      },
    };
  }
};
