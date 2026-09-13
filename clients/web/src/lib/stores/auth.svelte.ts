import { browser } from '$app/environment';
import type { UserStatus } from '$lib/types/timetable';

export interface AuthUser {
  id: string;
  email: string;
  full_name: string;
  role: 'admin' | 'moderator' | 'member';
  status: UserStatus;
  auth_epoch?: number;
  phone?: string;
}

class AuthState {
  user = $state<AuthUser | null>(null);
  token = $state<string | null>(null);
  isLoading = $state(true);

  private syncChannel: BroadcastChannel | null = null;
  private sseSource: EventSource | null = null;

  constructor() {
    if (browser) {
      const savedToken = localStorage.getItem('csac_token');
      const savedUser = localStorage.getItem('csac_user');
      if (savedToken && savedUser) {
        try {
          this.token = savedToken;
          this.user = JSON.parse(savedUser);
        } catch {
          this.logout();
        }
      }
      this.isLoading = false;

      // Setup multi-tab auth synchronization
      if (typeof BroadcastChannel !== 'undefined') {
        this.syncChannel = new BroadcastChannel('csac_auth_sync');
        this.syncChannel.onmessage = (event) => {
          if (event.data?.type === 'SESSION_UPDATED') {
            this.token = event.data.token;
            this.user = event.data.user;
          } else if (event.data?.type === 'SESSION_CLEARED') {
            this.token = null;
            this.user = null;
          }
        };
      }

      this.initSseListener();
    }
  }

  setSession(token: string, user: AuthUser) {
    this.token = token;
    this.user = user;
    if (browser) {
      localStorage.setItem('csac_token', token);
      localStorage.setItem('csac_user', JSON.stringify(user));
      // Synchronize cookies for SSR
      document.cookie = `csac_role=${user.role}; Path=/; SameSite=Lax; Max-Age=2592000`;
      document.cookie = `csac_token=${token}; Path=/; SameSite=Lax; Max-Age=2592000`;
      this.syncChannel?.postMessage({ type: 'SESSION_UPDATED', token, user });
      this.initSseListener();
    }
  }

  logout() {
    this.token = null;
    this.user = null;
    if (browser) {
      localStorage.removeItem('csac_token');
      localStorage.removeItem('csac_user');
      // Clear cookies
      document.cookie = 'csac_role=; Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT; SameSite=Lax';
      document.cookie = 'csac_token=; Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT; SameSite=Lax';
      this.syncChannel?.postMessage({ type: 'SESSION_CLEARED' });
      this.closeSse();
    }
  }

  async silentRefresh() {
    if (!browser || !this.isAuthenticated) return;

    const performRefresh = async () => {
      try {
        const { api } = await import('$lib/api/client');
        const res = await api.auth.refresh();
        if (res?.token && res?.user) {
          this.setSession(res.token, res.user);
        }
      } catch (err) {
        console.warn('[CSAC Auth] Silent refresh failed or unsupported in offline mock mode:', err);
      }
    };

    if (navigator.locks) {
      await navigator.locks.request('csac_token_refresh', performRefresh);
    } else {
      await performRefresh();
    }
  }

  private initSseListener() {
    if (!browser || !this.token) return;
    this.closeSse();

    try {
      this.sseSource = new EventSource(`/api/v1/events/user?token=${encodeURIComponent(this.token)}`);
      this.sseSource.addEventListener('AUTH_INVALIDATED', (e: MessageEvent) => {
        try {
          const data = JSON.parse(e.data);
          if (data?.user_id === this.user?.id) {
            this.silentRefresh();
          }
        } catch {
          this.silentRefresh();
        }
      });
      this.sseSource.onerror = () => {
        // SSE disconnects gracefully in offline or non-SSE dev mode
        this.closeSse();
      };
    } catch {
      // In dev or test environments without SSE backend
    }
  }

  private closeSse() {
    if (this.sseSource) {
      this.sseSource.close();
      this.sseSource = null;
    }
  }

  get isAuthenticated() {
    return !!this.token && !!this.user && this.user.status !== 'suspended';
  }

  get isAdmin() {
    return this.user?.role === 'admin';
  }

  get isModerator() {
    return this.user?.role === 'moderator' || this.user?.role === 'admin';
  }
}

export const auth = new AuthState();
