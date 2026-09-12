import type { PageLoad } from './$types';
import { api } from '$lib/api/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const showId = params.id || 'show-2026-annual';
  try {
    const numbers = await api.shows.listNumbers(showId, fetch);
    return { numbers };
  } catch (err) {
    console.error('Failed to load show numbers:', err);
    return { numbers: [] };
  }
};
