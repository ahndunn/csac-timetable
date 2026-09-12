<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore, t } from '$lib/i18n';
  import {
    Calendar,
    Plus,
    Search,
    Music,
    Clock,
    CheckCircle2,
    ArrowRight,
    MapPin,
    Sparkles,
    AlertCircle,
  } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import * as Dialog from '$lib/components/ui/dialog';

  interface ShowItem {
    id: string;
    title: string;
    description: string;
    venue: string;
    startDate: string;
    endDate: string;
    targetNumbers: number;
    activeSprints: number;
    qcPassRate: number;
    rehearsalHours: number;
  }

  let shows = $state<ShowItem[]>([
    {
      id: 'show-2026-annual',
      title: 'CSAC Annual Concert 2026',
      description: 'Grand annual showcase featuring multi-genre band performances, acoustic arrangements, and orchestral medleys.',
      venue: 'CSAC Main Auditorium',
      startDate: '2026-10-01',
      endDate: '2026-10-15',
      targetNumbers: 12,
      activeSprints: 3,
      qcPassRate: 75,
      rehearsalHours: 48,
    },
    {
      id: 'show-acoustic-vol4',
      title: 'Acoustic Night Vol. 4',
      description: 'Intimate unplugged acoustic showcase emphasizing close vocal harmonies, fingerstyle guitars, and jazz fusion.',
      venue: 'Studio Lounge B',
      startDate: '2026-11-05',
      endDate: '2026-11-12',
      targetNumbers: 6,
      activeSprints: 2,
      qcPassRate: 40,
      rehearsalHours: 18,
    },
  ]);

  let search = $state('');
  let isCreateModalOpen = $state(false);
  let newTitle = $state('');
  let newDescription = $state('');
  let newVenue = $state('CSAC Main Auditorium');
  let newStartDate = $state('2026-12-01');
  let newEndDate = $state('2026-12-15');
  let newTargetNumbers = $state(8);

  let filteredShows = $derived(
    shows.filter(
      (s) =>
        s.title.toLowerCase().includes(search.toLowerCase()) ||
        s.venue.toLowerCase().includes(search.toLowerCase())
    )
  );

  function handleCreateShow(e: Event) {
    e.preventDefault();
    if (!newTitle.trim()) return;

    const newShow: ShowItem = {
      id: `show-${Date.now()}`,
      title: newTitle.trim(),
      description: newDescription.trim() || 'New music show workspace.',
      venue: newVenue.trim() || 'CSAC Studio',
      startDate: newStartDate,
      endDate: newEndDate,
      targetNumbers: Number(newTargetNumbers) || 8,
      activeSprints: 1,
      qcPassRate: 0,
      rehearsalHours: 0,
    };

    shows = [newShow, ...shows];
    isCreateModalOpen = false;

    // Reset Form
    newTitle = '';
    newDescription = '';
  }
</script>

<svelte:head>
  <title>{$tStore('admin_shows.page_title')}</title>
</svelte:head>

<Navbar />

<div class="mx-auto flex max-w-7xl flex-col gap-6 p-6">
  <!-- Header Banner -->
  <Card class="flex flex-col gap-4 rounded-2xl border border-border bg-card p-6 shadow-sm md:flex-row md:items-center md:justify-between">
    <div class="flex flex-col gap-1.5">
      <Badge variant="outline" class="w-fit bg-primary/10 text-primary border-primary/20 gap-1.5 font-bold">
        <Calendar class="w-3.5 h-3.5 text-primary" />
        <span>{$tStore('admin_shows.navbar_title')}</span>
      </Badge>
      <h1 class="text-2xl font-extrabold tracking-tight text-foreground">
        {$tStore('admin_shows.heading')}
      </h1>
      <p class="text-xs text-muted-foreground">
        {$tStore('admin_shows.subheading')}
      </p>
    </div>

    <Button variant="default" size="sm" onclick={() => (isCreateModalOpen = true)} class="font-bold gap-1.5">
      <Plus class="w-4 h-4" />
      <span>{$tStore('admin_shows.btn_create_show')}</span>
    </Button>
  </Card>

  <!-- Monitor Stats Overview -->
  <div class="grid grid-cols-2 gap-4 md:grid-cols-4">
    <Card class="flex items-center gap-3.5 rounded-2xl border border-border bg-card p-4 shadow-sm">
      <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10 text-primary">
        <Music class="w-5 h-5" />
      </div>
      <div class="flex flex-col">
        <span class="text-lg font-bold text-foreground">{shows.reduce((acc, s) => acc + s.targetNumbers, 0)}</span>
        <span class="text-xs text-muted-foreground">Total Music Numbers</span>
      </div>
    </Card>

    <Card class="flex items-center gap-3.5 rounded-2xl border border-border bg-card p-4 shadow-sm">
      <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-blue-500/10 text-blue-600">
        <Clock class="w-5 h-5" />
      </div>
      <div class="flex flex-col">
        <span class="text-lg font-bold text-foreground">{shows.reduce((acc, s) => acc + s.rehearsalHours, 0)} hrs</span>
        <span class="text-xs text-muted-foreground">Scheduled Practice</span>
      </div>
    </Card>

    <Card class="flex items-center gap-3.5 rounded-2xl border border-border bg-card p-4 shadow-sm">
      <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-emerald-500/10 text-emerald-600">
        <CheckCircle2 class="w-5 h-5" />
      </div>
      <div class="flex flex-col">
        <span class="text-lg font-bold text-foreground">
          {Math.round(shows.reduce((acc, s) => acc + s.qcPassRate, 0) / (shows.length || 1))}%
        </span>
        <span class="text-xs text-muted-foreground">Average QC Pass</span>
      </div>
    </Card>

    <Card class="flex items-center gap-3.5 rounded-2xl border border-border bg-card p-4 shadow-sm">
      <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-purple-500/10 text-purple-600">
        <Sparkles class="w-5 h-5" />
      </div>
      <div class="flex flex-col">
        <span class="text-lg font-bold text-foreground">{shows.length}</span>
        <span class="text-xs text-muted-foreground">Active Productions</span>
      </div>
    </Card>
  </div>

  <!-- Filter & Search Toolbar -->
  <Card class="flex flex-col gap-3 rounded-2xl border border-border bg-card p-4 shadow-sm md:flex-row md:items-center md:justify-between">
    <div class="flex items-center gap-2">
      <h2 class="text-sm font-bold text-foreground">Productions</h2>
      <Badge variant="secondary" class="text-xs">{shows.length}</Badge>
    </div>

    <div class="relative w-full md:w-64">
      <Search class="pointer-events-none absolute left-2.5 top-2.5 w-3.5 h-3.5 text-muted-foreground" />
      <Input
        type="text"
        bind:value={search}
        placeholder="Search shows by title or venue..."
        class="h-8 text-xs pl-8"
      />
    </div>
  </Card>

  <!-- Shows List Grid -->
  <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
    {#each filteredShows as show (show.id)}
      <Card class="flex flex-col justify-between rounded-2xl border border-border bg-card p-6 shadow-sm transition-all duration-200 hover:-translate-y-1 hover:shadow-md hover:border-primary/40">
        <div class="flex flex-col gap-3">
          <div class="flex items-start justify-between gap-2">
            <div>
              <h3 class="text-base font-bold text-foreground">{show.title}</h3>
              <div class="flex items-center gap-1.5 text-xs text-muted-foreground mt-0.5">
                <MapPin class="w-3 h-3" />
                <span>{show.venue} • {show.startDate} to {show.endDate}</span>
              </div>
            </div>
            <Badge variant="secondary" class="text-[10px]">
              Sprint {show.activeSprints}
            </Badge>
          </div>

          <p class="text-xs leading-relaxed text-muted-foreground">
            {show.description}
          </p>

          <!-- Progress Metrics -->
          <div class="grid grid-cols-3 gap-2 rounded-lg border border-border bg-muted/40 p-2.5 text-center text-xs">
            <div class="flex flex-col">
              <span class="font-extrabold text-foreground">{show.targetNumbers}</span>
              <span class="text-[10px] text-muted-foreground uppercase">Numbers</span>
            </div>
            <div class="flex flex-col border-x border-border">
              <span class="font-extrabold text-foreground">{show.rehearsalHours}h</span>
              <span class="text-[10px] text-muted-foreground uppercase">Practice</span>
            </div>
            <div class="flex flex-col">
              <span class="font-extrabold text-emerald-600">{show.qcPassRate}%</span>
              <span class="text-[10px] text-muted-foreground uppercase">QC Passed</span>
            </div>
          </div>
        </div>

        <div class="flex items-center justify-end border-t border-border pt-4 mt-6">
          <Button href="/studio/shows/{show.id}/overview" variant="default" size="sm" class="gap-1.5">
            <span>{$tStore('admin_shows.card_btn_open')}</span>
            <ArrowRight class="w-3.5 h-3.5" />
          </Button>
        </div>
      </Card>
    {/each}
  </div>
</div>

<!-- Modal: Create New Show -->
{#if isCreateModalOpen}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) isCreateModalOpen = false; }}>
    <Dialog.Content class="max-w-lg">
      <Dialog.Header>
        <Dialog.Title class="text-base font-bold">
          {$tStore('admin_shows.modal_create_title')}
        </Dialog.Title>
        <Dialog.Description class="text-xs text-muted-foreground">
          Initialize a new production workspace for managing music numbers and sprints.
        </Dialog.Description>
      </Dialog.Header>

      <form onsubmit={handleCreateShow} class="flex flex-col gap-3 py-2">
        <div class="flex flex-col gap-1.5">
          <Label for="show-title" class="text-xs font-semibold">
            {$tStore('admin_shows.modal_title_label')} *
          </Label>
          <Input
            id="show-title"
            type="text"
            placeholder={$tStore('admin_shows.modal_title_placeholder')}
            bind:value={newTitle}
            required
            class="h-8 text-xs"
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="show-desc" class="text-xs font-semibold">
            {$tStore('admin_shows.modal_desc_label')}
          </Label>
          <Input
            id="show-desc"
            type="text"
            placeholder={$tStore('admin_shows.modal_desc_placeholder')}
            bind:value={newDescription}
            class="h-8 text-xs"
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="show-venue" class="text-xs font-semibold">
            {$tStore('admin_shows.modal_venue_label')}
          </Label>
          <Input
            id="show-venue"
            type="text"
            placeholder={$tStore('admin_shows.modal_venue_placeholder')}
            bind:value={newVenue}
            class="h-8 text-xs"
          />
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div class="flex flex-col gap-1.5">
            <Label for="start-date" class="text-xs font-semibold">
              {$tStore('admin_shows.modal_start_date')}
            </Label>
            <Input
              id="start-date"
              type="date"
              bind:value={newStartDate}
              class="h-8 text-xs"
            />
          </div>

          <div class="flex flex-col gap-1.5">
            <Label for="end-date" class="text-xs font-semibold">
              {$tStore('admin_shows.modal_end_date')}
            </Label>
            <Input
              id="end-date"
              type="date"
              bind:value={newEndDate}
              class="h-8 text-xs"
            />
          </div>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="target-numbers" class="text-xs font-semibold">
            {$tStore('admin_shows.modal_target_numbers')}
          </Label>
          <Input
            id="target-numbers"
            type="number"
            min="1"
            max="50"
            bind:value={newTargetNumbers}
            class="h-8 text-xs"
          />
        </div>

        <Dialog.Footer class="pt-3">
          <Button
            type="button"
            variant="outline"
            size="sm"
            onclick={() => (isCreateModalOpen = false)}
          >
            {$tStore('admin_shows.modal_btn_cancel')}
          </Button>
          <Button type="submit" variant="default" size="sm">
            {$tStore('admin_shows.modal_btn_submit')}
          </Button>
        </Dialog.Footer>
      </form>
    </Dialog.Content>
  </Dialog.Root>
{/if}
