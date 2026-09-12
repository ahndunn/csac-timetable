<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore } from '$lib/i18n';
  import { auth } from '$lib/stores/auth.svelte';
  import {
    Music,
    Calendar,
    Users,
    ShieldAlert,
    ArrowRight,
    Sparkles,
    Lock,
    FileSpreadsheet,
  } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
</script>

<svelte:head>
  <title>{$tStore('hub.page_title')}</title>
  <meta name="description" content={$tStore('hub.meta_description')} />
</svelte:head>

<Navbar />

<div class="mx-auto flex max-w-7xl flex-col gap-6 p-6">
  <!-- Hero Bento Banner -->
  <Card class="relative overflow-hidden rounded-2xl border border-primary/20 bg-gradient-to-br from-card via-card to-primary/5 p-8 shadow-sm">
    <div class="flex flex-col gap-3 max-w-3xl">
      <Badge variant="outline" class="w-fit bg-primary/10 text-primary border-primary/20 gap-1.5 font-bold">
        <Sparkles class="w-3.5 h-3.5 text-primary" />
        <span>{$tStore('hub.tag')}</span>
      </Badge>

      <h1 class="text-3xl font-extrabold tracking-tight text-foreground sm:text-4xl">
        {$tStore('hub.hero_title')}
      </h1>
      <p class="text-sm leading-relaxed text-muted-foreground">
        {$tStore('hub.hero_description')}
      </p>

      <div class="flex flex-wrap items-center gap-3 pt-2">
        <Button href="/studio" variant="default" size="lg" class="shadow-sm font-bold gap-2">
          <Music class="w-4 h-4" />
          <span>Launch Music Studio</span>
          <ArrowRight class="w-4 h-4" />
        </Button>
        <Button href="/studio/gear" variant="outline" size="lg" class="gap-2">
          <FileSpreadsheet class="w-4 h-4 text-blue-600" />
          <span>Instrument Fleet</span>
        </Button>
        {#if auth.isAuthenticated}
          <Button href="/admin/shows" variant="outline" size="lg" class="gap-2">
            <Calendar class="w-4 h-4 text-emerald-600" />
            <span>Admin Shows</span>
          </Button>
        {:else}
          <Button href="/auth/login" variant="outline" size="lg" class="gap-2">
            <Lock class="w-4 h-4 text-muted-foreground" />
            <span>{$tStore('hub.sign_in_admin')}</span>
          </Button>
        {/if}
      </div>
    </div>
  </Card>

  <!-- Bento Grid Cards -->
  <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
    <!-- Card 1: Music Studio Workspace -->
    <Card class="flex flex-col justify-between rounded-2xl border border-border bg-card p-6 shadow-sm transition-all duration-200 hover:-translate-y-1 hover:shadow-md hover:border-primary/40">
      <div>
        <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10 text-primary mb-4">
          <Music class="w-6 h-6" />
        </div>
        <h3 class="text-base font-bold text-foreground mb-2">Music Production Studio</h3>
        <p class="text-xs leading-relaxed text-muted-foreground mb-6">
          Show-driven music numbers, Kanban practice stages, QC verdict audits, and 1-click weekly sprint free-time registration.
        </p>
      </div>
      <div class="border-t border-border pt-4">
        <a href="/studio" class="flex items-center justify-between text-xs font-bold text-primary hover:underline">
          <span>Open Studio</span>
          <ArrowRight class="w-4 h-4" />
        </a>
      </div>
    </Card>

    <!-- Card 2: Instrument Fleet & Custody -->
    <Card class="flex flex-col justify-between rounded-2xl border border-border bg-card p-6 shadow-sm transition-all duration-200 hover:-translate-y-1 hover:shadow-md hover:border-blue-500/40">
      <div>
        <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-blue-500/10 text-blue-600 mb-4">
          <FileSpreadsheet class="w-6 h-6" />
        </div>
        <h3 class="text-base font-bold text-foreground mb-2">Instrument Fleet & Custody</h3>
        <p class="text-xs leading-relaxed text-muted-foreground mb-6">
          Centralized registry tracking club property and member-owned instruments, physical custody holders, and zero-conflict reservations.
        </p>
      </div>
      <div class="border-t border-border pt-4">
        <a href="/studio/gear" class="flex items-center justify-between text-xs font-bold text-blue-600 hover:underline">
          <span>Manage Fleet</span>
          <ArrowRight class="w-4 h-4" />
        </a>
      </div>
    </Card>

    <!-- Card 3: Admin Show Studio -->
    <Card class="flex flex-col justify-between rounded-2xl border border-border bg-card p-6 shadow-sm transition-all duration-200 hover:-translate-y-1 hover:shadow-md hover:border-emerald-500/40">
      <div>
        <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-emerald-500/10 text-emerald-600 mb-4">
          <Calendar class="w-6 h-6" />
        </div>
        <h3 class="text-base font-bold text-foreground mb-2">Admin Show Studio</h3>
        <p class="text-xs leading-relaxed text-muted-foreground mb-6">
          Design music shows, define production date windows, monitor rehearsal metrics, and track QC approval rates in real-time.
        </p>
      </div>
      <div class="border-t border-border pt-4">
        <a href="/admin/shows" class="flex items-center justify-between text-xs font-bold text-emerald-600 hover:underline">
          <span>Admin Shows</span>
          <ArrowRight class="w-4 h-4" />
        </a>
      </div>
    </Card>

    <!-- Card 4: Member Governance -->
    <Card class="flex flex-col justify-between rounded-2xl border border-border bg-card p-6 shadow-sm transition-all duration-200 hover:-translate-y-1 hover:shadow-md hover:border-purple-500/40">
      <div>
        <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-purple-500/10 text-purple-600 mb-4">
          <Users class="w-6 h-6" />
        </div>
        <h3 class="text-base font-bold text-foreground mb-2">{$tStore('hub.card_users_title')}</h3>
        <p class="text-xs leading-relaxed text-muted-foreground mb-6">
          {$tStore('hub.card_users_desc')}
        </p>
      </div>
      <div class="border-t border-border pt-4">
        <a href="/admin/users" class="flex items-center justify-between text-xs font-bold text-purple-600 hover:underline">
          <span>{$tStore('hub.card_users_link')}</span>
          <ArrowRight class="w-4 h-4" />
        </a>
      </div>
    </Card>

    <!-- Card 5: Quorum Demotion Approval -->
    <Card class="flex flex-col justify-between rounded-2xl border border-border bg-card p-6 shadow-sm transition-all duration-200 hover:-translate-y-1 hover:shadow-md hover:border-destructive/40">
      <div>
        <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-destructive/10 text-destructive mb-4">
          <ShieldAlert class="w-6 h-6" />
        </div>
        <h3 class="text-base font-bold text-foreground mb-2">{$tStore('hub.card_quorum_title')}</h3>
        <p class="text-xs leading-relaxed text-muted-foreground mb-6">
          {$tStore('hub.card_quorum_desc')}
        </p>
      </div>
      <div class="border-t border-border pt-4">
        <a href="/admin/approve" class="flex items-center justify-between text-xs font-bold text-destructive hover:underline">
          <span>{$tStore('hub.card_quorum_link')}</span>
          <ArrowRight class="w-4 h-4" />
        </a>
      </div>
    </Card>
  </div>
</div>
