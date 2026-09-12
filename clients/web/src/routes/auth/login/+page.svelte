<script lang="ts">
  import { goto } from '$app/navigation';
  import { api, ApiError } from '$lib/api/client';
  import { auth } from '$lib/stores/auth.svelte';
  import { tStore, t } from '$lib/i18n';
  import { Lock, Mail, ArrowRight, CircleAlert, Sparkles, LoaderCircle } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';

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

<div class="flex min-h-screen items-center justify-center bg-background p-6">
  <Card class="w-full max-w-md rounded-2xl border border-border bg-card p-8 shadow-lg">
    <div class="flex flex-col items-center text-center mb-6">
      <div class="flex h-12 w-12 items-center justify-center rounded-2xl bg-primary/10 text-primary mb-3">
        <Sparkles class="w-6 h-6 text-primary" />
      </div>
      <h1 class="text-xl font-extrabold text-foreground">
        {$tStore('auth.brand_title')}
      </h1>
      <p class="text-xs text-muted-foreground mt-1">
        {$tStore('auth.brand_subtitle')}
      </p>
    </div>

    {#if errorMessage}
      <div class="flex items-center gap-2 rounded-lg border border-destructive/20 bg-destructive/10 p-3 text-xs font-semibold text-destructive mb-4">
        <CircleAlert class="w-4 h-4 text-destructive" />
        <span>{errorMessage}</span>
      </div>
    {/if}

    <form onsubmit={handleLogin} class="flex flex-col gap-4">
      <div class="flex flex-col gap-1.5">
        <Label for="email" class="text-xs font-semibold">
          {$tStore('auth.email_label')}
        </Label>
        <div class="relative">
          <Mail class="pointer-events-none absolute left-3 top-2.5 w-4 h-4 text-muted-foreground" />
          <Input
            id="email"
            type="email"
            placeholder={$tStore('auth.email_placeholder')}
            bind:value={email}
            required
            autocomplete="email"
            class="h-9 pl-9 text-xs"
          />
        </div>
      </div>

      <div class="flex flex-col gap-1.5">
        <Label for="password" class="text-xs font-semibold">
          {$tStore('auth.password_label')}
        </Label>
        <div class="relative">
          <Lock class="pointer-events-none absolute left-3 top-2.5 w-4 h-4 text-muted-foreground" />
          <Input
            id="password"
            type="password"
            placeholder={$tStore('auth.password_placeholder')}
            bind:value={password}
            required
            autocomplete="current-password"
            class="h-9 pl-9 text-xs"
          />
        </div>
      </div>

      <Button type="submit" variant="default" size="lg" disabled={isLoading} class="w-full mt-2 font-bold gap-2">
        {#if isLoading}
          <LoaderCircle class="w-4 h-4 animate-spin" />
          <span>{$tStore('auth.authenticating')}</span>
        {:else}
          <span>{$tStore('auth.sign_in_btn')}</span>
          <ArrowRight class="w-4 h-4" />
        {/if}
      </Button>
    </form>

    <div class="border-t border-border pt-4 mt-6 text-center">
      <a href="/studio" class="text-xs font-medium text-muted-foreground hover:text-primary transition-colors">
        {$tStore('auth.back_to_timetable')}
      </a>
    </div>
  </Card>
</div>
