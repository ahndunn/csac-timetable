<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { page } from '$app/state';
  import { tStore } from '$lib/i18n';
  import {
    Music,
    LayoutDashboard,
    Calendar,
    Users,
    CircleCheck,
  } from '@lucide/svelte';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';

  let { data, children } = $props();

  const showId = $derived(page.params.id);
  const show = $derived(data?.showOverview || page.data?.showOverview || {
    id: showId,
    title: 'Music Show Workspace',
    venue: 'CSAC Main Auditorium',
    dates: 'Oct 1 - Oct 15, 2026',
    readiness_percent: 0
  });

  const activeTab = $derived(
    page.url.pathname.includes('/numbers')
      ? 'numbers'
      : page.url.pathname.includes('/sprints')
      ? 'sprints'
      : page.url.pathname.includes('/roster')
      ? 'roster'
      : 'overview'
  );
</script>

<svelte:head>
  <title>{show.title || 'CSAC Show Studio'} — {showId}</title>
</svelte:head>

<Navbar />

<div class="max-w-[1280px] mx-auto p-6 flex flex-col gap-6">
  <!-- Show Banner & Sub-Nav Header -->
  <Card class="relative overflow-hidden p-6 pb-0 flex flex-col gap-4 shadow-sm border border-border/90">
    <!-- Top Vivid Orange Border Strip -->
    <div class="absolute inset-x-0 top-0 h-1 bg-gradient-to-r from-primary via-primary/80 to-primary/40"></div>

    <div>
      <Badge variant="outline" class="bg-primary/15 text-primary border-primary/30 gap-1.5 font-bold px-2.5 py-0.5">
        <Music class="w-3.5 h-3.5 text-primary" />
        <span>Active Music Show Workspace</span>
      </Badge>

      <div class="flex flex-col sm:flex-row justify-between sm:items-end gap-3 mt-2.5">
        <div>
          <h1 class="text-2xl sm:text-3xl font-black text-foreground tracking-tight m-0 mb-1">
            {show.title}
          </h1>
          <p class="text-xs text-muted-foreground m-0">Venue: {show.venue || 'CSAC Main Auditorium'} • Production Dates: {show.dates || 'TBD'}</p>
        </div>

        <div class="inline-flex items-center gap-1.5 bg-emerald-500/10 text-emerald-600 px-3 py-1.5 rounded-full font-bold text-xs border border-emerald-500/20">
          <CircleCheck class="w-4 h-4 text-emerald-600" />
          <span>{show.readiness_percent}% Stage Ready</span>
        </div>
      </div>
    </div>

    <!-- Sub-Page Navigation Tabs -->
    <nav class="flex gap-1.5 border-t border-border pt-2.5 -mb-px">
      <a
        href="/studio/shows/{showId}/overview"
        class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-xs font-bold transition-all {activeTab === 'overview' ? 'text-primary bg-primary/15 shadow-xs font-extrabold' : 'text-muted-foreground hover:text-foreground hover:bg-muted/60'}"
      >
        <LayoutDashboard class="w-4 h-4" />
        <span>{$tStore('studio_shows.tab_overview')}</span>
      </a>
      <a
        href="/studio/shows/{showId}/numbers"
        class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-xs font-bold transition-all {activeTab === 'numbers' ? 'text-primary bg-primary/15 shadow-xs font-extrabold' : 'text-muted-foreground hover:text-foreground hover:bg-muted/60'}"
      >
        <Music class="w-4 h-4" />
        <span>{$tStore('studio_shows.tab_numbers')}</span>
      </a>
      <a
        href="/studio/shows/{showId}/sprints"
        class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-xs font-bold transition-all {activeTab === 'sprints' ? 'text-primary bg-primary/15 shadow-xs font-extrabold' : 'text-muted-foreground hover:text-foreground hover:bg-muted/60'}"
      >
        <Calendar class="w-4 h-4" />
        <span>{$tStore('studio_shows.tab_sprints')}</span>
      </a>
      <a
        href="/studio/shows/{showId}/roster"
        class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-xs font-bold transition-all {activeTab === 'roster' ? 'text-primary bg-primary/15 shadow-xs font-extrabold' : 'text-muted-foreground hover:text-foreground hover:bg-muted/60'}"
      >
        <Users class="w-4 h-4" />
        <span>{$tStore('studio_shows.tab_roster')}</span>
      </a>
    </nav>
  </Card>

  <!-- Sub-Page Main Content View -->
  <main class="min-h-[400px]">
    {#key page.url.pathname}
      {@render children()}
    {/key}
  </main>
</div>
