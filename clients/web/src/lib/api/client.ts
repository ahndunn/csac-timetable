let authModule: any = null;
try {
  // @ts-ignore
  authModule = await import('$lib/stores/auth.svelte').catch(() => null);
} catch {
  // Non-SvelteKit runtime fallback
}

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

async function request<T>(path: string, options: RequestInit = {}, customFetch?: typeof fetch): Promise<T> {
  const headers = new Headers(options.headers || {});
  if (!headers.has('Content-Type') && !(options.body instanceof FormData)) {
    headers.set('Content-Type', 'application/json');
  }

  const token = authModule?.auth?.token;
  if (token && !headers.has('Authorization')) {
    headers.set('Authorization', `Bearer ${token}`);
  }

  const fetchFn = customFetch || fetch;
  const res = await fetchFn(`${API_BASE}${path}`, {
    ...options,
    headers,
  });

  let isJson = false;
  try {
    const contentType = res.headers?.get?.('content-type');
    isJson = typeof contentType === 'string' && contentType.includes('application/json');
  } catch {
    // If header inspection fails due to environment restrictions
    isJson = false;
  }
  const data = isJson ? await res.json() : await res.text();

  if (!res.ok) {
    const errorMsg =
      (typeof data === 'object' && data !== null && 'error' in data
        ? typeof data.error === 'object' && data.error !== null && 'message' in data.error
          ? data.error.message
          : data.error
        : null) ||
      (typeof data === 'string' ? data : 'API Request Failed');
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
    verifyActivationOtp: (payload: { token?: string; email?: string; otp: string }) =>
      request<{ activation_session_id: string; email: string; prefilled_data: { full_name?: string; phone?: string } }>(
        '/auth/verify-activation-otp',
        {
          method: 'POST',
          body: JSON.stringify(payload),
        }
      ),
    completeActivation: (payload: { activation_session_id: string; password: string; full_name: string; phone?: string }) =>
      request<{ token: string; user: any }>('/auth/complete-activation', {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    refresh: () =>
      request<{ token: string; user: any }>('/auth/refresh', {
        method: 'POST',
      }),
    me: () => request<any>('/auth/me'),
  },
  admin: {
    listUsers: (customFetch?: typeof fetch) => request<any[]>('/admin/users', {}, customFetch),
    inviteUser: (payload: { email: string; full_name?: string; role?: string; show_id?: string; phone?: string }) =>
      request<{ user: any; message: string }>('/admin/users/invite', {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    resendInvite: (userId: string) =>
      request<{ message: string }>(`/admin/users/${userId}/resend-invite`, {
        method: 'POST',
      }),
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
    updateStatus: (userId: string, status: string) =>
      request<any>(`/admin/users/${userId}/status`, {
        method: 'PUT',
        body: JSON.stringify({ status }),
      }),
    createDowngradeProposal: (userId: string, payload: { target_role: string; reason: string }) =>
      request<any>(`/admin/users/${userId}/downgrade-proposal`, {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    listProposals: (customFetch?: typeof fetch) => request<any[]>('/admin/approve/proposals', {}, customFetch),
    requestOtp: (proposalId: string) =>
      request<{ message: string; ttl_seconds: number }>(`/admin/approve/${proposalId}/request-otp`, {
        method: 'POST',
      }),
    voteProposal: (proposalId: string, payload: { otp_code?: string; otp?: string; decision: 'approve' | 'reject' }) =>
      request<any>(`/admin/approve/${proposalId}/vote`, {
        method: 'POST',
        body: JSON.stringify({
          otp: payload.otp || payload.otp_code || '',
          decision: payload.decision,
        }),
      }),
  },
  users: {
    list: async (customFetch?: typeof fetch) => {
      const users = await request<any[]>('/admin/users', {}, customFetch);
      return { users };
    },
    invite: (payload: { email: string; full_name?: string; role?: string; show_id?: string; phone?: string }) =>
      request<{ user: any; message: string }>('/admin/users/invite', {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    resendInvite: (userId: string) =>
      request<{ message: string }>(`/admin/users/${userId}/resend-invite`, {
        method: 'POST',
      }),
    create: (payload: { email: string; full_name: string; password?: string; role: string }) =>
      request<{ user: any; initial_password?: string }>('/admin/users', {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    updateStatus: (userId: string, status: string) =>
      request<any>(`/admin/users/${userId}/status`, {
        method: 'PUT',
        body: JSON.stringify({ status }),
      }),
  },
  governance: {
    listProposals: async (customFetch?: typeof fetch) => {
      const proposals = await request<any[]>('/admin/approve/proposals', {}, customFetch);
      return { proposals };
    },
    requestOtp: (proposalId: string) =>
      request<{ message: string; ttl_seconds: number }>(`/admin/approve/${proposalId}/request-otp`, {
        method: 'POST',
      }),
    submitVote: (proposalId: string, payload: { decision: 'approve' | 'reject'; otp_code: string }) =>
      request<any>(`/admin/approve/${proposalId}/vote`, {
        method: 'POST',
        body: JSON.stringify({
          otp: payload.otp_code,
          decision: payload.decision,
        }),
      }),
    proposeDemotion: (payload: { target_user_id: string; target_role: string; reason: string }) =>
      request<any>(`/admin/users/${payload.target_user_id}/downgrade-proposal`, {
        method: 'POST',
        body: JSON.stringify({
          target_role: payload.target_role,
          reason: payload.reason,
        }),
      }),
  },
  events: {
    list: (customFetch?: typeof fetch) => request<any[]>('/events', {}, customFetch),
    get: (eventId: string, customFetch?: typeof fetch) => request<{ event: any; time_slots: any[] }>(`/events/${eventId}`, {}, customFetch),
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
    listNumbers: (customFetch?: typeof fetch) => request<any[]>('/music/numbers', {}, customFetch),
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
    listInstruments: (customFetch?: typeof fetch) => request<{ instruments: any[]; reservations: any[] }>('/music/instruments', {}, customFetch),
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
  shows: {
    getOverview: (showId: string, customFetch?: typeof fetch) =>
      (customFetch
        ? customFetch(`/api/v1/shows/${showId}/overview`).then((r) => r.json())
        : request<any>(`/shows/${showId}/overview`)),
    listNumbers: (showId: string, customFetch?: typeof fetch) =>
      (customFetch
        ? customFetch(`/api/v1/shows/${showId}/numbers`).then((r) => r.json())
        : request<any[]>(`/shows/${showId}/numbers`)),
    createNumber: (
      showId: string,
      payload: { title: string; genre?: string; pm_name?: string; qc_reviewer?: string }
    ) =>
      request<any>(`/shows/${showId}/numbers`, {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    updateStage: (showId: string, numberId: string, stage: string) =>
      request<any>(`/shows/${showId}/numbers/${numberId}/stage`, {
        method: 'PUT',
        body: JSON.stringify({ stage }),
      }),
    updateLineup: (
      showId: string,
      numberId: string,
      lineup: {
        vocalLead?: string;
        guitarLead?: string;
        bass?: string;
        drums?: string;
        keys?: string;
      }
    ) =>
      request<any>(`/shows/${showId}/numbers/${numberId}/lineup`, {
        method: 'PUT',
        body: JSON.stringify(lineup),
      }),
    submitQc: (
      showId: string,
      numberId: string,
      payload: { verdict: 'pass' | 'revision'; notes?: string }
    ) =>
      request<any>(`/shows/${showId}/numbers/${numberId}/qc`, {
        method: 'POST',
        body: JSON.stringify(payload),
      }),
    listRoster: (showId: string, customFetch?: typeof fetch) =>
      (customFetch
        ? customFetch(`/api/v1/shows/${showId}/roster`).then((r) => r.json())
        : request<any[]>(`/shows/${showId}/roster`)),
    saveRosterMember: (
      showId: string,
      payload: {
        id?: string;
        fullName: string;
        email: string;
        phone?: string;
        showRole: string;
        primaryInstrument: string;
        secondaryInstruments?: string[];
        practiceHours?: number;
      }
    ) => {
      if (payload.id) {
        return request<any>(`/shows/${showId}/roster/${payload.id}`, {
          method: 'PUT',
          body: JSON.stringify(payload),
        });
      }
      return request<any>(`/shows/${showId}/roster`, {
        method: 'POST',
        body: JSON.stringify(payload),
      });
    },
    deleteRosterMember: (showId: string, memberId: string) =>
      request<any>(`/shows/${showId}/roster/${memberId}`, {
        method: 'DELETE',
      }),
    getActiveSprint: (showId: string, customFetch?: typeof fetch) =>
      (customFetch
        ? customFetch(`/api/v1/shows/${showId}/sprints/active`).then((r) => r.json())
        : request<any>(`/shows/${showId}/sprints/active`)),
    saveSprintAvailability: (
      showId: string,
      sprintId: string,
      slots: Array<{ day_of_week: string; slot_label: string; is_available: boolean }>
    ) =>
      request<any>(`/shows/${showId}/sprints/${sprintId}/availability`, {
        method: 'POST',
        body: JSON.stringify({ slots }),
      }),
    getSprintHistory: (showId: string, sprintId: string, customFetch?: typeof fetch) =>
      (customFetch
        ? customFetch(`/api/v1/shows/${showId}/sprints/${sprintId}/history`).then((r) => r.json())
        : request<any>(`/shows/${showId}/sprints/${sprintId}/history`)),
  },
};

