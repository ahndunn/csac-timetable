import { browser } from '$app/environment';

export interface AuthUser {
  id: string;
  email: string;
  full_name: string;
  role: 'admin' | 'moderator' | 'member';
  status: 'active' | 'suspended';
}

class AuthState {
  user = $state<AuthUser | null>(null);
  token = $state<string | null>(null);
  isLoading = $state(true);

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
    }
  }

  setSession(token: string, user: AuthUser) {
    this.token = token;
    this.user = user;
    if (browser) {
      localStorage.setItem('csac_token', token);
      localStorage.setItem('csac_user', JSON.stringify(user));
    }
  }

  logout() {
    this.token = null;
    this.user = null;
    if (browser) {
      localStorage.removeItem('csac_token');
      localStorage.removeItem('csac_user');
    }
  }

  get isAuthenticated() {
    return !!this.token && !!this.user;
  }

  get isAdmin() {
    return this.user?.role === 'admin';
  }

  get isModerator() {
    return this.user?.role === 'moderator' || this.user?.role === 'admin';
  }
}

export const auth = new AuthState();
