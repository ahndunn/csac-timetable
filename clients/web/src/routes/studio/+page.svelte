<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore } from '$lib/i18n';
  import {
    Music,
    Calendar,
    ArrowRight,
    Sparkles,
    CircleCheck,
    FileSpreadsheet,
  } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';

  interface ShowSummary {
    id: string;
    title: string;
    description: string;
    venue: string;
    startDate: string;
    endDate: string;
    numbersCount: number;
    qcPassRate: number;
  }

  let activeShows = $state<ShowSummary[]>([
    {
      id: 'show-2026-annual',
      title: 'CSAC Annual Concert 2026',
      description: 'Main annual cultural concert featuring 12 band numbers and orchestral arrangements.',
      venue: 'CSAC Main Auditorium',
      startDate: '2026-10-01',
      endDate: '2026-10-15',
      numbersCount: 12,
      qcPassRate: 75,
    },
    {
      id: 'show-acoustic-vol4',
      title: 'Acoustic Night Vol. 4',
      description: 'Intimate acoustic unplugged session with vocal harmonies & classical guitars.',
      venue: 'Studio Lounge B',
      startDate: '2026-11-05',
      endDate: '2026-11-12',
      numbersCount: 6,
      qcPassRate: 40,
    },
  ]);
</script>

<svelte:head>
  <title>{$tStore('studio.page_title')}</title>
</svelte:head>

<Navbar />

<div class="mx-auto flex max-w-7xl flex-col gap-8 p-6">
  <!-- Studio Hero Section with Vivid Orange Glow Accent -->
  <Card class="relative overflow-hidden rounded-3xl border-2 border-primary/30 bg-gradient-to-br from-card via-card to-primary/10 p-8 sm:p-10 shadow-lg shadow-primary/5">
    <!-- Ambient Radial Glow Accent -->
    <div class="pointer-events-none absolute -right-16 -top-16 h-64 w-64 rounded-full bg-primary/20 blur-3xl"></div>
    <div class="pointer-events-none absolute -left-16 -bottom-16 h-48 w-48 rounded-full bg-primary/10 blur-2xl"></div>

    <div class="relative z-10 flex flex-col gap-4 max-w-3xl">
      <Badge variant="outline" class="w-fit bg-primary/15 text-primary border-primary/30 gap-1.5 font-bold shadow-xs py-1 px-3">
        <Sparkles class="w-4 h-4 text-primary animate-pulse" />
        <span>{$tStore('studio.tag')}</span>
      </Badge>

      <h1 class="text-3xl font-black tracking-tight text-foreground sm:text-5xl">
        <span class="text-primary">CSAC</span> {$tStore('studio.hero_title').replace('CSAC ', '')}
      </h1>
      <p class="text-sm sm:text-base leading-relaxed text-muted-foreground font-medium max-w-2xl">
        {$tStore('studio.hero_description')}
      </p>

      <div class="flex flex-wrap items-center gap-3 pt-3">
        <Button href="/studio/shows/show-2026-annual/overview" variant="default" size="lg" class="shadow-md shadow-primary/30 font-bold gap-2 px-5 text-sm h-10">
          <Music class="w-4 h-4" />
          <span>Enter Annual Concert 2026</span>
          <ArrowRight class="w-4 h-4" />
        </Button>
        <Button href="/studio/gear" variant="outline" size="lg" class="gap-2 px-5 text-sm h-10 hover:border-primary/40 hover:text-primary transition-all">
          <FileSpreadsheet class="w-4 h-4 text-primary" />
          <span>Instrument Fleet</span>
        </Button>
      </div>
    </div>
  </Card>

  <!-- Active Shows Grid -->
  <div class="flex flex-col gap-4">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2.5">
        <div class="flex h-7 w-7 items-center justify-center rounded-lg bg-primary/15 text-primary">
          <Calendar class="w-4 h-4 text-primary" />
        </div>
        <h2 class="text-xl font-extrabold text-foreground tracking-tight">{$tStore('studio.active_shows_title')}</h2>
      </div>
    </div>

    <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
      {#each activeShows as show}
        <Card class="group relative flex flex-col justify-between rounded-2xl border border-border bg-card p-6 shadow-xs transition-all duration-300 hover:-translate-y-1 hover:shadow-lg hover:shadow-primary/5 hover:border-primary/50">
          <!-- Subtle top color strip -->
          <div class="absolute inset-x-0 top-0 h-1 bg-gradient-to-r from-primary/80 via-primary to-primary/40 rounded-t-2xl opacity-80 group-hover:opacity-100 transition-opacity"></div>

          <div class="flex flex-col gap-3.5">
            <div class="flex items-start justify-between gap-2">
              <h3 class="text-lg font-bold text-foreground group-hover:text-primary transition-colors">{show.title}</h3>
              <Badge variant="outline" class="bg-primary/10 text-primary border-primary/20 text-xs font-bold">
                {show.numbersCount} Numbers
              </Badge>
            </div>

            <p class="text-xs leading-relaxed text-muted-foreground">
              {show.description}
            </p>

            <div class="grid grid-cols-2 gap-3 rounded-xl border border-border/80 bg-muted/40 p-3 text-xs text-muted-foreground">
              <div class="flex items-center gap-2">
                <Calendar class="w-4 h-4 text-primary" />
                <span class="font-medium">{show.startDate} &rarr; {show.endDate}</span>
              </div>
              <div class="flex items-center gap-2">
                <CircleCheck class="w-4 h-4 text-emerald-600" />
                <span>QC Pass: <strong class="text-foreground">{show.qcPassRate}%</strong></span>
              </div>
            </div>
          </div>

          <div class="flex items-center justify-between border-t border-border pt-4 mt-6">
            <span class="text-xs font-semibold text-muted-foreground">{show.venue}</span>
            <Button href="/studio/shows/{show.id}/overview" variant="default" size="sm" class="gap-1.5 font-bold shadow-xs">
              <span>Open Studio</span>
              <ArrowRight class="w-3.5 h-3.5 transition-transform group-hover:translate-x-0.5" />
            </Button>
          </div>
        </Card>
      {/each}
    </div>
  </div>
</div>
