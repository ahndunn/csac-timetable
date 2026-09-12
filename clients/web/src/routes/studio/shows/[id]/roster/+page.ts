import type { PageLoad } from './$types';
import { api } from '$lib/api/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const showId = params.id || 'show-2026-annual';
  try {
    const roster = await api.shows.listRoster(showId, fetch);
    return { roster };
  } catch (err) {
    console.error('Failed to load show roster:', err);
    return { roster: [] };
  }
};
