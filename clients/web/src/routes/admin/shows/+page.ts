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
    const shows: ShowItem[] = (events || []).map((e: any) => {
      if (!e.id || !e.title) {
        throw new Error('Invalid show record received from server');
      }
      return {
        id: e.id,
        title: e.title,
        description: e.description || '',
        venue: e.venue || '',
        startDate: e.start_date || '',
        endDate: e.end_date || '',
        targetNumbers: Number(e.target_numbers) || 0,
        activeSprints: Number(e.active_sprints) || 0,
        qcPassRate: Number(e.qc_pass_rate) || 0,
        rehearsalHours: Number(e.rehearsal_hours) || 0,
      };
    });

    return { shows };
  } catch (err: any) {
    console.error('Failed to load shows in admin:', err);
    throw error(err.status || 500, err.message || 'Failed to load shows');
  }
};
