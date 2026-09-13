import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api/client';

export interface DemotionProposal {
  id: string;
  target_user_id: string;
  target_name: string;
  target_email: string;
  target_role: string;
  reason: string;
  initiator_name: string;
  required_approvals: number;
  current_approvals: number;
  status: 'pending' | 'approved' | 'rejected' | 'expired';
  expires_at: string;
}

export const load: PageLoad = async ({ fetch }) => {
  try {
    const res = await api.governance.listProposals(fetch);
    const proposals: DemotionProposal[] = (res?.proposals || []).map((p: any) => {
      if (!p.id || !p.target_user_id) {
        throw new Error('Invalid proposal record received from server');
      }
      return {
        id: p.id,
        target_user_id: p.target_user_id,
        target_name: p.target_name || p.target_email || p.target_user_id,
        target_email: p.target_email || '',
        target_role: p.target_role || 'member',
        reason: p.reason || '',
        initiator_name: p.initiator_name || '',
        required_approvals: Number(p.required_approvals) || 0,
        current_approvals: Number(p.current_approvals) || 0,
        status: p.status || 'pending',
        expires_at: p.expires_at || '',
      };
    });

    return { proposals };
  } catch (err: any) {
    console.error('Failed to load proposals in admin approve:', err);
    throw error(err.status || 500, err.message || 'Failed to load governance proposals');
  }
};
