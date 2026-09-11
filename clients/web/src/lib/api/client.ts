import { auth } from '$lib/stores/auth.svelte';

const API_BASE = '/api/v1';

export class ApiError extends Error {
  status: number;
  data: any;
  constructor(status: number, message: string, data?: any) {
    super(message);
    this.status = status;
    this.data = data;
  }
}

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
  const headers = new Headers(options.headers || {});
  headers.set('Content-Type', 'application/json');

  if (auth.token) {
    headers.set('Authorization', `Bearer ${auth.token}`);
  }

  const res = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers,
  });

  const isJson = res.headers.get('content-type')?.includes('application/json');
  const data = isJson ? await res.json() : await res.text();

  if (!res.ok) {
    const errorMsg = data?.error || (typeof data === 'string' ? data : 'API Request Failed');
    throw new ApiError(res.status, errorMsg, data);
  }

  return data as T;
}

export const api = {
  auth: {
    login: (payload: { email: string; password: string }) =>
      request<{ token: string; user: any }>('/auth/login', {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    me: () => request<any>('/auth/me'),
  },
  admin: {
    listUsers: () => request<any[]>('/admin/users'),
    createUser: (payload: { email: string; full_name: string; role: string }) =>
      request<{ user: any; initial_password?: string }>('/admin/users', {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    updateRole: (userId: string, newRole: string) =>
      request<any>(`/admin/users/${userId}/role`, {
        method: 'PUT',
        body: JSON.stringify({ new_role: newRole }),
      }),
    createDowngradeProposal: (userId: string, payload: { target_role: string; reason: string }) =>
      request<any>(`/admin/users/${userId}/downgrade-proposal`, {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    listProposals: () => request<any[]>('/admin/approve/proposals'),
    requestOtp: (proposalId: string) =>
      request<{ message: string; ttl_seconds: number }>(`/admin/approve/${proposalId}/request-otp`, {
        method: 'POST',
      }),
    voteProposal: (proposalId: string, payload: { otp: string; decision: 'approve' | 'reject' }) =>
      request<any>(`/admin/approve/${proposalId}/vote`, {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
  },
  events: {
    list: () => request<any[]>('/events'),
    get: (eventId: string) => request<{ event: any; time_slots: any[] }>(`/events/${eventId}`),
    create: (payload: {
      title: string;
      description?: string;
      start_date: string;
      end_date: string;
      time_slots: Array<{ day_of_week: string; slot_label: string; sort_order?: number }>;
    }) =>
      request<any>('/events', {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    close: (eventId: string) =>
      request<any>(`/events/${eventId}/close`, {
        method: 'PUT',
      }),
    submitVote: (eventId: string, payload: { slot_id: string; is_available: boolean; note?: string }) =>
      request<any>(`/events/${eventId}/vote`, {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
  },
};
