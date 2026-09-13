import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api/client';

export interface ShowItem {
  id: string;
  title: string;
  description: string;
  venue: string;
  startDate: string;
  endDate: string;
  targetNumbers: number;
  activeSprints: number;
  qcPassRate: number;
  rehearsalHours: number;
}

export const load: PageLoad = async ({ fetch }) => {
  try {
    const events = await api.events.list(fetch);
    const shows: ShowItem[] = await Promise.all(
      (events || []).map(async (e: any) => {
        if (!e.id || !e.title) {
          throw new Error('Invalid show record received from server');
        }

        // Fetch live overview statistics to ensure 100% sync across studio and admin
        let overviewData: any = null;
        try {
          overviewData = await api.shows.getOverview(e.id, fetch);
        } catch {
          overviewData = null;
        }

        const targetNumbers = overviewData?.total_numbers ?? Number(e.target_numbers) ?? Number(e.numbers_count) ?? 0;
        const activeSprints = overviewData?.milestones ? overviewData.milestones.filter((m: any) => m.status === 'active').length : (Number(e.active_sprints) || 0);
        const qcPassRate = overviewData?.readiness_percent ?? Number(e.qc_pass_rate) ?? 0;
        const rehearsalHours = overviewData?.total_hours ?? (targetNumbers * 4);

        return {
          id: e.id,
          title: overviewData?.title || e.title,
          description: e.description || '',
          venue: overviewData?.venue || e.venue || 'CSAC Main Auditorium',
          startDate: e.start_date || '',
          endDate: e.end_date || '',
          targetNumbers,
          activeSprints,
          qcPassRate,
          rehearsalHours,
        };
      })
    );

    return { shows };
  } catch (err: any) {
    console.error('Failed to load shows in admin:', err);
    throw error(err.status || 500, err.message || 'Failed to load shows');
  }
};
