import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api, ApiError } from '$lib/api/client';

export interface ShowSummary {
  id: string;
  title: string;
  description: string;
  venue: string;
  startDate: string;
  endDate: string;
  numbersCount: number;
  qcPassRate: number;
}

export const load: PageLoad = async ({ fetch }) => {
  try {
    const res = await api.events.list(fetch);
    const shows: ShowSummary[] = (res || []).map((event: any) => {
      if (!event.id || !event.title) {
        throw new Error('Invalid show data received from server');
      }
      return {
        id: event.id,
        title: event.title,
        description: event.description || '',
        venue: event.venue || '',
        startDate: event.start_date || '',
        endDate: event.end_date || '',
        numbersCount: Number(event.numbers_count) || 0,
        qcPassRate: Number(event.qc_pass_rate) || 0,
      };
    });

    return { shows };
  } catch (err: any) {
    console.error('Failed to load shows for studio page:', err);
    throw error(err.status || 500, err.message || 'Failed to load active shows');
  }
};
