<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { page } from '$app/state';
  import {
    Music,
    Clock,
    CircleCheck,
    Activity,
    ArrowRight,
    ChevronRight,
    Shield,
    Calendar,
    Users,
  } from '@lucide/svelte';
  import { Card, CardHeader, CardTitle, CardContent } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';

  let { data } = $props();
  const showId = $derived(page.params.id || '');
  const overview = $derived(data?.overview || {
    id: showId,
    title: '',
    venue: '',
    dates: '',
    readiness_percent: 0,
    total_numbers: 0,
    total_hours: 0,
    qc_approved_count: 0,
    highlights: [],
    milestones: [],
  });
</script>

<div class="flex flex-col gap-5">
  <!-- Key Metrics Bento Grid with 1-Click Deep Links -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
    <a href="/studio/shows/{showId}/numbers?stage=stage_ready" class="block group no-underline text-inherit">
      <Card class="p-5 transition-all duration-300 hover:-translate-y-1 hover:shadow-lg hover:shadow-primary/5 hover:border-primary/50">
        <div class="flex items-center justify-between mb-3">
          <span class="text-xs font-bold text-muted-foreground uppercase tracking-wider">{$tStore('studio_shows.readiness')}</span>
          <div class="flex h-8 w-8 items-center justify-center rounded-xl bg-primary/15 text-primary group-hover:scale-110 transition-transform">
            <Activity class="w-4 h-4 text-primary" />
          </div>
        </div>
        <div class="text-3xl font-black text-primary tracking-tight">{overview.readiness_percent}%</div>
        <div class="h-2 w-full bg-muted rounded-full my-2.5 overflow-hidden">
          <div class="h-full bg-gradient-to-r from-primary to-primary/80 rounded-full transition-all" style="width: {overview.readiness_percent}%;"></div>
        </div>
        <div class="flex items-center justify-between mt-1">
          <span class="text-xs text-muted-foreground">{overview.qc_approved_count} of {overview.total_numbers} numbers ready for stage</span>
          <ChevronRight class="w-4 h-4 text-muted-foreground transition-transform group-hover:translate-x-1 group-hover:text-primary" />
        </div>
      </Card>
    </a>

    <a href="/studio/shows/{showId}/numbers" class="block group no-underline text-inherit">
      <Card class="p-5 transition-all duration-300 hover:-translate-y-1 hover:shadow-lg hover:shadow-primary/5 hover:border-primary/50">
        <div class="flex items-center justify-between mb-3">
          <span class="text-xs font-bold text-muted-foreground uppercase tracking-wider">{$tStore('studio_shows.total_numbers')}</span>
          <div class="flex h-8 w-8 items-center justify-center rounded-xl bg-muted text-muted-foreground group-hover:scale-110 transition-transform">
            <Music class="w-4 h-4" />
          </div>
        </div>
        <div class="text-3xl font-black text-foreground tracking-tight">{overview.total_numbers}</div>
        <div class="flex items-center justify-between mt-6">
          <span class="text-xs text-muted-foreground">Across 4 performance categories</span>
          <ChevronRight class="w-4 h-4 text-muted-foreground transition-transform group-hover:translate-x-1 group-hover:text-primary" />
        </div>
      </Card>
    </a>

    <a href="/studio/shows/{showId}/sprints" class="block group no-underline text-inherit">
      <Card class="p-5 transition-all duration-300 hover:-translate-y-1 hover:shadow-lg hover:shadow-primary/5 hover:border-primary/50">
        <div class="flex items-center justify-between mb-3">
          <span class="text-xs font-bold text-muted-foreground uppercase tracking-wider">{$tStore('studio_shows.total_hours')}</span>
          <div class="flex h-8 w-8 items-center justify-center rounded-xl bg-muted text-muted-foreground group-hover:scale-110 transition-transform">
            <Clock class="w-4 h-4" />
          </div>
        </div>
        <div class="text-3xl font-black text-foreground tracking-tight">{overview.total_hours} <span class="text-xl font-bold text-muted-foreground">hrs</span></div>
        <div class="flex items-center justify-between mt-6">
          <span class="text-xs text-muted-foreground">3 practice sprints completed</span>
          <ChevronRight class="w-4 h-4 text-muted-foreground transition-transform group-hover:translate-x-1 group-hover:text-primary" />
        </div>
      </Card>
    </a>

    <a href="/studio/shows/{showId}/numbers?stage=qc_approved" class="block group no-underline text-inherit">
      <Card class="p-5 transition-all duration-300 hover:-translate-y-1 hover:shadow-lg hover:shadow-primary/5 hover:border-primary/50">
        <div class="flex items-center justify-between mb-3">
          <span class="text-xs font-bold text-muted-foreground uppercase tracking-wider">{$tStore('studio_shows.qc_approved')}</span>
          <div class="flex items-center gap-1.5 text-xs text-emerald-600 font-bold">
            <CircleCheck class="w-4 h-4 text-emerald-600" />
            <span>Target: 100%</span>
          </div>
        </div>
        <div class="text-3xl font-black text-emerald-600 tracking-tight">{overview.qc_approved_count} <span class="text-xl font-bold text-foreground">/ {overview.total_numbers}</span></div>
        <div class="flex items-center justify-between mt-6">
          <span class="text-xs text-muted-foreground">Reviewed by designated QC team</span>
          <ChevronRight class="w-4 h-4 text-primary transition-transform group-hover:translate-x-1" />
        </div>
      </Card>
    </a>
  </div>

  <!-- Quick Action Jump Bar (Cross-Screen Synergy) -->
  <Card class="p-4 flex flex-wrap items-center gap-4 border border-border/90 bg-gradient-to-r from-card via-card to-primary/5 shadow-xs">
    <div class="flex items-center gap-2 text-xs font-extrabold text-foreground uppercase tracking-wider">
      <div class="flex h-6 w-6 items-center justify-center rounded-md bg-primary/15 text-primary">
        <Shield class="w-3.5 h-3.5 text-primary" />
      </div>
      <span>Quick Workflow Jump:</span>
    </div>
    <div class="flex flex-wrap gap-2.5">
      <a
        href="/studio/shows/{showId}/numbers?stage=ready_for_qc"
        class="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full text-xs font-bold bg-primary text-primary-foreground shadow-xs hover:bg-primary/90 transition-all hover:scale-[1.02] active:scale-[0.98]"
      >
        <Activity class="w-3.5 h-3.5" />
        <span>3 Numbers Ready for QC</span>
        <ArrowRight class="w-3.5 h-3.5" />
      </a>
      <a
        href="/studio/shows/{showId}/sprints"
        class="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full text-xs font-bold bg-card text-foreground border border-border hover:border-primary/50 hover:bg-primary/10 hover:text-primary transition-all hover:-translate-y-0.5 shadow-2xs"
      >
        <Calendar class="w-3.5 h-3.5 text-primary" />
        <span>Manage 15m Sprint Timetable</span>
        <ArrowRight class="w-3 h-3 text-muted-foreground" />
      </a>
      <a
        href="/studio/shows/{showId}/roster"
        class="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full text-xs font-bold bg-card text-foreground border border-border hover:border-primary/50 hover:bg-primary/10 hover:text-primary transition-all hover:-translate-y-0.5 shadow-2xs"
      >
        <Users class="w-3.5 h-3.5 text-primary" />
        <span>Inspect Show Leadership & Roster</span>
        <ArrowRight class="w-3 h-3 text-muted-foreground" />
      </a>
    </div>
  </Card>

  <!-- Lineup Summary & Milestones -->
  <div class="grid grid-cols-1 lg:grid-cols-5 gap-5">
    <Card class="lg:col-span-3 p-5 flex flex-col gap-3.5 border border-border/90">
      <div class="flex items-center justify-between mb-1">
        <div class="flex items-center gap-2">
          <div class="flex h-6 w-6 items-center justify-center rounded-md bg-primary/15 text-primary">
            <Music class="w-3.5 h-3.5 text-primary" />
          </div>
          <h3 class="text-base font-extrabold text-foreground m-0">Show Lineup Highlights</h3>
        </div>
        <a href="/studio/shows/{showId}/numbers" class="inline-flex items-center gap-1 text-xs font-bold text-primary hover:underline">
          <span>View All {overview.total_numbers} Numbers</span>
          <ChevronRight class="w-3.5 h-3.5" />
        </a>
      </div>
      <div class="flex flex-col gap-2.5">
        {#each overview.highlights as highlight}
          <a href="/studio/shows/{showId}/numbers?q={encodeURIComponent(highlight.title)}" class="flex items-center gap-3 p-3 bg-muted/40 hover:bg-primary/5 rounded-xl transition-all hover:translate-x-1 border border-border/50 hover:border-primary/40">
            <div class="w-9 h-9 rounded-lg bg-primary/15 flex items-center justify-center shrink-0">
              <Music class="w-4 h-4 text-primary" />
            </div>
            <div class="flex-1 min-w-0">
              <div class="text-sm font-bold text-foreground truncate">{highlight.title}</div>
              <div class="text-xs text-muted-foreground truncate">{highlight.meta}</div>
            </div>
            <Badge class={highlight.stage === 'stage_ready' ? 'bg-emerald-500/15 text-emerald-600 border-0 font-bold text-xs shrink-0' : highlight.stage === 'qc_approved' ? 'bg-primary/15 text-primary border border-primary/30 font-bold text-xs shrink-0' : 'bg-primary/10 text-primary border-0 font-bold text-xs shrink-0'}>
              {highlight.badge}
            </Badge>
          </a>
        {:else}
          <div class="text-center py-6 text-xs text-muted-foreground">No highlights available</div>
        {/each}
      </div>
    </Card>

    <Card class="lg:col-span-2 p-5 flex flex-col gap-3.5 border border-border/90">
      <div class="flex items-center justify-between mb-1">
        <div class="flex items-center gap-2">
          <div class="flex h-6 w-6 items-center justify-center rounded-md bg-primary/15 text-primary">
            <Clock class="w-3.5 h-3.5 text-primary" />
          </div>
          <h3 class="text-base font-extrabold text-foreground m-0">Production Milestones</h3>
        </div>
        <a href="/studio/shows/{showId}/sprints" class="inline-flex items-center gap-1 text-xs font-bold text-primary hover:underline">
          <span>View Sprints</span>
          <ChevronRight class="w-3.5 h-3.5" />
        </a>
      </div>
      <div class="flex flex-col gap-4 mt-1">
        {#each overview.milestones as milestone}
          <div class="flex items-start gap-3 p-2.5 rounded-xl {milestone.status === 'active' ? 'bg-primary/10 border border-primary/30 shadow-xs' : 'bg-muted/30 border border-border/50'}">
            <div class="w-7 h-7 rounded-full {milestone.status === 'active' ? 'bg-primary text-primary-foreground shadow-xs shadow-primary/30' : 'bg-emerald-500/15 text-emerald-600'} flex items-center justify-center shrink-0 mt-0.5 font-bold">
              {#if milestone.status === 'active'}
                <Clock class="w-4 h-4" />
              {:else}
                <CircleCheck class="w-4 h-4" />
              {/if}
            </div>
            <div>
              <div class="text-xs font-bold text-foreground">{milestone.title}</div>
              <div class="text-[11px] {milestone.status === 'active' ? 'text-primary font-bold' : 'text-muted-foreground font-medium'}">{milestone.date}</div>
            </div>
          </div>
        {:else}
          <div class="text-center py-6 text-xs text-muted-foreground">No milestones tracked</div>
        {/each}
      </div>
    </Card>
  </div>
</div>
