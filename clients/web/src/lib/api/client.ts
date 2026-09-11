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
  music: {
    listNumbers: () => request<any[]>('/music/numbers'),
    createNumber: (payload: {
      title: string;
      genre?: string;
      pm_user_id?: string;
      target_sessions_per_week?: number;
      description?: string;
    }) =>
      request<any>('/music/numbers', {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    listInstruments: () => request<{ instruments: any[]; reservations: any[] }>('/music/instruments'),
    registerInstrument: (payload: {
      name: string;
      code: string;
      category: string;
      ownership_type: 'club_property' | 'member_owned';
      owner_user_id?: string;
      custody_user_id?: string;
      custody_location?: string;
      availability_status?: 'free_to_borrow' | 'in_use' | 'unavailable' | 'in_maintenance';
      notes?: string;
    }) =>
      request<any>('/music/instruments', {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    updateInstrumentStatus: (
      id: string,
      payload: {
        availability_status?: 'free_to_borrow' | 'in_use' | 'unavailable' | 'in_maintenance';
        custody_user_id?: string;
        custody_location?: string;
        notes?: string;
      }
    ) =>
      request<any>(`/music/instruments/${id}/status`, {
        method: 'PUT',
        body: JSON.stringify(payload),
      }),
    reserveInstrument: (payload: {
      instrument_id: string;
      music_number_id: string;
      day_of_week: string;
      slot_label: string;
      notes?: string;
    }) =>
      request<any>('/music/instruments/reserve', {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
  },
  sprints: {
    list: () => request<any[]>('/sprints'),
    listTasks: (sprintId: string) => request<any[]>(`/sprints/${sprintId}/tasks`),
    createTask: (
      sprintId: string,
      payload: {
        music_number_id: string;
        task_type: 'study' | 'create' | 'review_qc';
        title: string;
        description?: string;
        assigned_to?: string;
        qc_reviewer_id?: string;
      }
    ) =>
      request<any>(`/sprints/${sprintId}/tasks`, {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    reviewTask: (
      sprintId: string,
      taskId: string,
      payload: {
        status: 'passed' | 'blocked' | 'in_progress';
        qc_feedback?: string;
      }
    ) =>
      request<any>(`/sprints/${sprintId}/tasks/${taskId}/review`, {
        method: 'PUT',
        body: JSON.stringify(payload),
      }),
    submitAvailability: (
      sprintId: string,
      payload: {
        slots: Array<{ day_of_week: string; slot_label: string; is_available: boolean }>;
      }
    ) =>
      request<any>(`/sprints/${sprintId}/availability`, {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    scheduleStub: (sprintId: string) =>
      request<any>(`/sprints/${sprintId}/schedule`, {
        method: 'POST',
      }),
  },
};

