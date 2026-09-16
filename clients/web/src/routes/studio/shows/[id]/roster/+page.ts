import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const showId = params.id;
  if (!showId) {
    throw error(400, 'Show ID is required');
  }
  try {
    const roster = await api.shows.listRoster(showId, fetch);
    return { roster };
  } catch (err: any) {
    console.error('Failed to load show roster:', err);
    throw error(err.status || 500, err.message || 'Failed to load show roster members');
  }
};
