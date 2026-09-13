import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const showId = params.id || 'e0000000-0000-0000-0000-000000000001';
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
