<script lang="ts">
  import { goto } from '$app/navigation';
  import { api, ApiError } from '$lib/api/client';
  import { auth } from '$lib/stores/auth.svelte';
  import { tStore, t } from '$lib/i18n';
  import { Lock, Mail, ArrowRight, AlertCircle, Sparkles } from '@lucide/svelte';

  let email = $state('');
  let password = $state('');
  let isLoading = $state(false);
  let errorMessage = $state<string | null>(null);

  async function handleLogin(e: SubmitEvent) {
    e.preventDefault();
    errorMessage = null;
    isLoading = true;

    try {
      const response = await api.auth.login({ email, password });
      auth.setSession(response.token, response.user);
      if (response.user.role === 'admin') {
        goto('/admin/users');
      } else if (response.user.role === 'moderator') {
        goto('/admin/events');
      } else {
        goto('/');
      }
    } catch (err) {
      if (err instanceof ApiError) {
        errorMessage = err.message;
      } else {
        errorMessage = t('auth.error_gateway');
      }
    } finally {
      isLoading = false;
    }
  }
</script>

<svelte:head>
  <title>{$tStore('auth.page_title')}</title>
</svelte:head>

<div class="login-wrapper">
  <div class="bento-login-card">
    <div class="brand-header">
      <div class="logo-icon">
        <Sparkles size={24} class="text-orange" />
      </div>
      <h1 class="brand-title">{$tStore('auth.brand_title')}</h1>
      <p class="brand-subtitle">{$tStore('auth.brand_subtitle')}</p>
    </div>

    {#if errorMessage}
      <div class="error-banner">
        <AlertCircle size={18} />
        <span>{errorMessage}</span>
      </div>
    {/if}

    <form onsubmit={handleLogin} class="login-form">
      <div class="input-group">
        <label for="email">{$tStore('auth.email_label')}</label>
        <div class="input-wrapper">
          <Mail size={18} class="input-icon" />
          <input
            id="email"
            type="email"
            placeholder={$tStore('auth.email_placeholder')}
            bind:value={email}
            required
            autocomplete="email"
          />
        </div>
      </div>

      <div class="input-group">
        <label for="password">{$tStore('auth.password_label')}</label>
        <div class="input-wrapper">
          <Lock size={18} class="input-icon" />
          <input
            id="password"
            type="password"
            placeholder={$tStore('auth.password_placeholder')}
            bind:value={password}
            required
            autocomplete="current-password"
          />
        </div>
      </div>

      <button type="submit" class="submit-btn" disabled={isLoading}>
        {#if isLoading}
          <div class="spinner"></div>
          <span>{$tStore('auth.authenticating')}</span>
        {:else}
          <span>{$tStore('auth.sign_in_btn')}</span>
          <ArrowRight size={18} />
        {/if}
      </button>
    </form>

    <div class="card-footer">
      <a href="/utils/timetable" class="back-link">
        {$tStore('auth.back_to_timetable')}
      </a>
    </div>
  </div>
</div>

<style>
  .login-wrapper {
    min-height: calc(100vh - 72px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem 1.5rem;
    background-color: var(--color-bg-base, #f8fafc);
  }

  .bento-login-card {
    width: 100%;
    max-width: 440px;
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 24px;
    padding: 2.5rem 2rem;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.04);
  }

  .brand-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .logo-icon {
    width: 52px;
    height: 52px;
    border-radius: 16px;
    background: rgba(255, 107, 0, 0.1);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 1rem;
  }

  .text-orange { color: #ff6b00; }

  .brand-title {
    font-size: 1.45rem;
    font-weight: 700;
    color: #0f172a;
    margin: 0 0 0.5rem 0;
  }

  .brand-subtitle {
    font-size: 0.875rem;
    color: #64748b;
    margin: 0;
  }

  .error-banner {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    background: #fef2f2;
    border: 1px solid #fee2e2;
    color: #ef4444;
    padding: 0.75rem 1rem;
    border-radius: 12px;
    font-size: 0.875rem;
    margin-bottom: 1.5rem;
  }

  .login-form {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .input-group label {
    font-size: 0.825rem;
    font-weight: 600;
    color: #334155;
  }

  .input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  :global(.input-icon) {
    position: absolute;
    left: 1rem;
    color: #94a3b8;
    pointer-events: none;
  }

  .input-wrapper input {
    width: 100%;
    padding: 0.75rem 1rem 0.75rem 2.75rem;
    border: 1px solid #e2e8f0;
    border-radius: 12px;
    font-size: 0.95rem;
    background: #f8fafc;
    color: #0f172a;
    transition: all 0.2s;
  }

  .input-wrapper input:focus {
    outline: none;
    border-color: #ff6b00;
    background: #ffffff;
    box-shadow: 0 0 0 3px rgba(255, 107, 0, 0.15);
  }

  .submit-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    width: 100%;
    padding: 0.85rem;
    border-radius: 14px;
    background: #ff6b00;
    color: #ffffff;
    border: none;
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    margin-top: 0.5rem;
  }

  .submit-btn:hover:not(:disabled) {
    background: #e65c00;
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(255, 107, 0, 0.25);
  }

  .submit-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .spinner {
    width: 18px;
    height: 18px;
    border: 2px solid #ffffff;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .card-footer {
    text-align: center;
    margin-top: 1.75rem;
    padding-top: 1.25rem;
    border-top: 1px solid #f1f5f9;
  }

  .back-link {
    font-size: 0.85rem;
    font-weight: 500;
    color: #64748b;
    text-decoration: none;
    transition: color 0.2s;
  }

  .back-link:hover {
    color: #ff6b00;
  }
</style>
