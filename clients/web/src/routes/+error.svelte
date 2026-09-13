<script lang="ts">
  import { page } from '$app/state';
  import { tStore } from '$lib/i18n';
  import { AlertTriangle, RefreshCw, ArrowLeft, Home, ShieldAlert } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import Navbar from '$lib/components/Navbar.svelte';

  const status = $derived(Number(page.status) || 500);
  const errorMsg = $derived(
    page.error?.message || 'An unexpected error occurred while communicating with the service.'
  );

  function formatFriendlyMessage(msg: string, statusCode: number): string {
    if (statusCode === 404) {
      return 'The requested concert workspace or studio resource could not be found. It may have been moved or archived.';
    }
    if (statusCode === 401 || statusCode === 403) {
      return 'You do not have the required permissions to access this studio workspace. Please sign in with an authorized role.';
    }
    if (msg.includes('filterSerializedResponseHeaders') || msg.includes('header') || msg.includes('fetch') || msg.includes('Gateway') || statusCode >= 500) {
      return 'The Studio service is currently refreshing or temporarily unavailable. Please try again in a moment.';
    }
    return msg;
  }

  const friendlyMessage = $derived(formatFriendlyMessage(errorMsg, status));

  function handleRetry() {
    if (typeof window !== 'undefined') {
      window.location.reload();
    }
  }
</script>

<svelte:head>
  <title>Error {status} — CSAC Studio</title>
</svelte:head>

<Navbar />

<div class="flex min-h-[calc(100vh-3.5rem)] items-center justify-center p-6 bg-background">
  <Card class="relative w-full max-w-lg overflow-hidden rounded-3xl border border-border bg-card p-8 shadow-xl">
    <!-- Ambient top glow -->
    <div class="absolute -right-12 -top-12 h-36 w-36 rounded-full bg-destructive/10 blur-2xl pointer-events-none"></div>
    <div class="absolute -left-12 -bottom-12 h-36 w-36 rounded-full bg-primary/10 blur-2xl pointer-events-none"></div>

    <div class="relative z-10 flex flex-col items-center text-center gap-4">
      <div class="flex h-16 w-16 items-center justify-center rounded-2xl bg-destructive/15 text-destructive">
        {#if status === 404}
          <ShieldAlert class="w-8 h-8" />
        {:else}
          <AlertTriangle class="w-8 h-8" />
        {/if}
      </div>

      <Badge variant="outline" class="border-destructive/30 bg-destructive/10 text-destructive font-black px-3 py-1 text-xs">
        HTTP {status} {status === 404 ? 'NOT FOUND' : status === 401 ? 'UNAUTHORIZED' : 'SERVICE ERROR'}
      </Badge>

      <h1 class="text-2xl font-black tracking-tight text-foreground sm:text-3xl">
        {#if status === 404}
          Resource Not Found
        {:else if status === 401 || status === 403}
          Access Restricted
        {:else}
          Unable to Load Studio Data
        {/if}
      </h1>

      <p class="text-xs sm:text-sm text-muted-foreground leading-relaxed max-w-sm">
        {friendlyMessage}
      </p>

      <div class="w-full rounded-2xl bg-muted/40 border border-border p-3.5 text-left text-xs font-mono text-muted-foreground space-y-1">
        <div class="text-[11px] font-bold text-foreground">Diagnostic Signal:</div>
        <div class="truncate text-[11px]">URL: {page.url.pathname}</div>
        <div class="text-[11px]">Status Code: {status}</div>
      </div>

      <div class="flex flex-wrap items-center justify-center gap-3 pt-2 w-full">
        <Button variant="default" class="gap-2 font-bold shadow-sm" onclick={handleRetry}>
          <RefreshCw class="w-4 h-4" />
          <span>Retry Request</span>
        </Button>
        <Button href="/studio" variant="outline" class="gap-2 font-bold">
          <Home class="w-4 h-4" />
          <span>Return to Studio</span>
        </Button>
      </div>
    </div>
  </Card>
</div>
