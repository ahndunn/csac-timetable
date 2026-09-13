import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api/client';
import type { GearItem, ShowGearAllocation, GearCategory } from '$lib/types/timetable';

function mapCategory(cat: string): GearCategory {
  const c = (cat || '').toLowerCase();
  if (c.includes('string') || c.includes('guitar') || c.includes('bass')) return 'strings';
  if (c.includes('key') || c.includes('piano') || c.includes('synth')) return 'keys';
  if (c.includes('drum') || c.includes('percussion') || c.includes('cajon')) return 'drums';
  if (c.includes('amp') || c.includes('cab')) return 'amps_cabs';
  if (c.includes('fx') || c.includes('pedal')) return 'pedals_fx';
  if (c.includes('di') || c.includes('mic') || c.includes('audio')) return 'audio_di';
  return 'cables_accessories';
}

export const load: PageLoad = async ({ fetch }) => {
  try {
    const res = await api.music.listInstruments(fetch);
    const rawInstruments = res?.instruments || [];
    const rawReservations = res?.reservations || [];

    const gearList: GearItem[] = rawInstruments.map((inst: any) => {
      if (!inst.id || !inst.name) {
        throw new Error('Invalid instrument payload from server');
      }
      return {
        id: inst.id,
        name: inst.name,
        category: mapCategory(inst.category),
        ownership: inst.ownership_type || 'club_property',
        custodianName: inst.custody_location || '',
        ownerName: inst.owner_name || (inst.ownership_type === 'member_owned' ? inst.custody_location : undefined),
        locationNote: inst.custody_location || '',
        status: inst.availability_status || 'free_to_borrow',
        lendingPolicy: inst.ownership_type === 'member_owned' ? 'show_only' : 'open_to_all',
        serialNumber: inst.code,
        notes: inst.notes || undefined,
      };
    });

    const showAllocations: ShowGearAllocation[] = rawReservations.map((r: any, idx: number) => {
      const inst = rawInstruments.find((i: any) => i.id === r.instrument_id);
      return {
        id: r.id || `res-${idx}`,
        showId: r.event_id || '',
        gearId: r.instrument_id,
        gearName: inst?.name || 'Allocated Gear',
        category: mapCategory(inst?.category || ''),
        ownership: inst?.ownership_type || 'club_property',
        allocatedFor: 'music_number',
        musicNumberTitle: `Rehearsal (${r.day_of_week || ''} ${r.slot_label || ''})`.trim(),
        primaryPerformerName: inst?.custody_location || '',
        status: 'active_stage',
        isOnBehalfRetrieval: false,
      };
    });

    return {
      gearList,
      showAllocations,
    };
  } catch (err: any) {
    console.error('Failed to load instrument fleet:', err);
    throw error(err.status || 500, err.message || 'Failed to load gear catalog');
  }
};
