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
    const shows: ShowSummary[] = await Promise.all(
      (res || []).map(async (event: any) => {
        if (!event.id || !event.title) {
          throw new Error('Invalid show data received from server');
        }

        let overviewData: any = null;
        try {
          overviewData = await api.shows.getOverview(event.id, fetch);
        } catch {
          overviewData = null;
        }

        const numbersCount = overviewData?.total_numbers ?? Number(event.numbers_count) ?? 0;
        const qcPassRate = overviewData?.readiness_percent ?? Number(event.qc_pass_rate) ?? 0;

        return {
          id: event.id,
          title: overviewData?.title || event.title,
          description: event.description || '',
          venue: overviewData?.venue || event.venue || 'CSAC Main Auditorium',
          startDate: event.start_date || '',
          endDate: event.end_date || '',
          numbersCount,
          qcPassRate,
        };
      })
    );

    return { shows };
  } catch (err: any) {
    console.error('Failed to load shows for studio page:', err);
    throw error(err.status || 500, err.message || 'Failed to load active shows');
  }
};
