import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const showId = params.id;
  if (!showId) {
    throw error(400, 'Show ID is required');
  }
  try {
    const [numbers, roster] = await Promise.all([
      api.shows.listNumbers(showId, fetch),
      api.shows.listRoster(showId, fetch).catch(() => []),
    ]);
    return { numbers, roster };
  } catch (err: any) {
    console.error('Failed to load show numbers:', err);
    throw error(err.status || 500, err.message || 'Failed to load show music numbers');
  }
};
