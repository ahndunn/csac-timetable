import type { LayoutLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api/client';

export const load: LayoutLoad = async ({ params, fetch }) => {
  const showId = params.id;
  if (!showId) {
    throw error(400, 'Show ID is required');
  }
  try {
    const overview = await api.shows.getOverview(showId, fetch);
    return {
      showOverview: overview,
    };
  } catch (err: any) {
    console.error('Failed to load show overview for layout:', err);
    // Return a minimal fallback or throw
    return {
      showOverview: {
        id: showId,
        title: 'Music Show Workspace',
        venue: 'CSAC Main Auditorium',
        dates: '2026-10-01 -> 2026-10-15',
        readiness_percent: 0,
        total_numbers: 0,
        total_hours: 0,
        qc_approved_count: 0,
        highlights: [],
        milestones: [],
      },
    };
  }
};
