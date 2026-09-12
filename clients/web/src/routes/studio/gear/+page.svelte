<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore } from '$lib/i18n';
  import {
    FileSpreadsheet,
    Plus,
    Search,
    RefreshCw,
    Shield,
    MapPin,
    UserCheck,
    Music,
  } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import * as Table from '$lib/components/ui/table';
  import * as Dialog from '$lib/components/ui/dialog';

  interface Instrument {
    id: string;
    name: string;
    category: 'Guitar' | 'Bass' | 'Drums' | 'Keys' | 'Sound/PA';
    ownership: 'club_property' | 'member_owned';
    ownerName?: string;
    custodianName: string;
    locationNote: string;
    status: 'free_to_borrow' | 'in_use' | 'unavailable' | 'in_maintenance';
  }

  let gearList = $state<Instrument[]>([
    {
      id: 'inst-1',
      name: 'Fender Stratocaster Sunburst (Club)',
      category: 'Guitar',
      ownership: 'club_property',
      custodianName: 'Minh Pháp',
      locationNote: 'Studio Rehearsal Room A',
      status: 'free_to_borrow',
    },
    {
      id: 'inst-2',
      name: 'Yamaha TRBX 5-String Bass (Member-Lent)',
      category: 'Bass',
      ownership: 'member_owned',
      ownerName: 'Hoàng Nam',
      custodianName: 'Hoàng Nam',
      locationNote: 'CSAC Storage Locker #3',
      status: 'in_use',
    },
    {
      id: 'inst-3',
      name: 'Roland RD-88 Stage Piano (Club)',
      category: 'Keys',
      ownership: 'club_property',
      custodianName: 'Bảo Anh',
      locationNote: 'Auditorium Main Stage',
      status: 'free_to_borrow',
    },
    {
      id: 'inst-4',
      name: 'Pearl Export Drum Kit 5-Piece (Club)',
      category: 'Drums',
      ownership: 'club_property',
      custodianName: 'Thu Hà',
      locationNote: 'Studio Rehearsal Room B',
      status: 'free_to_borrow',
    },
    {
      id: 'inst-5',
      name: 'Shure SM58 Wireless Microphone Pair',
      category: 'Sound/PA',
      ownership: 'club_property',
      custodianName: 'Minh Pháp',
      locationNote: 'Sound Desk Tech Case',
      status: 'in_maintenance',
    },
  ]);

  let search = $state('');
  let filterOwnership = $state<'all' | 'club_property' | 'member_owned'>('all');
  let isTransferModalOpen = $state(false);
  let selectedGear = $state<Instrument | null>(null);
  let newCustodianName = $state('');
  let newLocationNote = $state('');

  let filteredGear = $derived(
    gearList.filter((g) => {
      const matchSearch =
        g.name.toLowerCase().includes(search.toLowerCase()) ||
        g.category.toLowerCase().includes(search.toLowerCase()) ||
        g.custodianName.toLowerCase().includes(search.toLowerCase());
      const matchFilter =
        filterOwnership === 'all' || g.ownership === filterOwnership;
      return matchSearch && matchFilter;
    })
  );

  function openTransferModal(gear: Instrument) {
    selectedGear = gear;
    newCustodianName = gear.custodianName;
    newLocationNote = gear.locationNote;
    isTransferModalOpen = true;
  }

  function handleTransferSubmit(e: Event) {
    e.preventDefault();
    if (!selectedGear) return;

    gearList = gearList.map((g) =>
      g.id === selectedGear?.id
        ? {
            ...g,
            custodianName: newCustodianName,
            locationNote: newLocationNote,
          }
        : g
    );

    isTransferModalOpen = false;
    selectedGear = null;
  }

  const statusBadgeMap = {
    free_to_borrow: 'bg-emerald-500/10 text-emerald-600 border-emerald-500/20',
    in_use: 'bg-blue-500/10 text-blue-600 border-blue-500/20',
    in_maintenance: 'bg-amber-500/10 text-amber-600 border-amber-500/20',
    unavailable: 'bg-destructive/10 text-destructive border-destructive/20',
  };
</script>

<svelte:head>
  <title>{$tStore('gear.page_title')}</title>
</svelte:head>

<Navbar />

<div class="mx-auto flex max-w-7xl flex-col gap-6 p-6">
  <!-- Header Banner -->
  <Card class="flex flex-col gap-4 rounded-2xl border border-border bg-card p-6 shadow-sm md:flex-row md:items-center md:justify-between">
    <div class="flex flex-col gap-1.5">
      <Badge variant="outline" class="w-fit bg-primary/10 text-primary border-primary/20 gap-1.5 font-bold">
        <Shield class="w-3.5 h-3.5 text-primary" />
        <span>Independent Fleet Governance</span>
      </Badge>
      <h1 class="text-2xl font-extrabold tracking-tight text-foreground">
        {$tStore('gear.heading')}
      </h1>
      <p class="text-xs text-muted-foreground">
        {$tStore('gear.subheading')}
      </p>
    </div>

    <Button variant="default" size="sm" class="font-bold gap-1.5">
      <Plus class="w-4 h-4" />
      <span>{$tStore('gear.btn_register')}</span>
    </Button>
  </Card>

  <!-- Filter & Search Toolbar -->
  <Card class="flex flex-col gap-3 rounded-2xl border border-border bg-card p-4 shadow-sm md:flex-row md:items-center md:justify-between">
    <div class="flex items-center gap-1.5 flex-wrap">
      <Button
        variant={filterOwnership === 'all' ? 'default' : 'outline'}
        size="sm"
        onclick={() => (filterOwnership = 'all')}
      >
        {$tStore('gear.filter_all')}
      </Button>
      <Button
        variant={filterOwnership === 'club_property' ? 'default' : 'outline'}
        size="sm"
        onclick={() => (filterOwnership = 'club_property')}
      >
        {$tStore('gear.filter_club')}
      </Button>
      <Button
        variant={filterOwnership === 'member_owned' ? 'default' : 'outline'}
        size="sm"
        onclick={() => (filterOwnership = 'member_owned')}
      >
        {$tStore('gear.filter_member')}
      </Button>
    </div>

    <div class="relative w-full md:w-64">
      <Search class="pointer-events-none absolute left-2.5 top-2.5 w-3.5 h-3.5 text-muted-foreground" />
      <Input
        type="text"
        placeholder={$tStore('gear.search_placeholder')}
        bind:value={search}
        class="h-8 pl-8 text-xs"
      />
    </div>
  </Card>

  <!-- Gear Inventory Table -->
  <Card class="rounded-2xl border border-border bg-card shadow-sm overflow-hidden p-0">
    <Table.Root>
      <Table.TableHeader>
        <Table.TableRow class="bg-muted/40">
          <Table.TableHead>{$tStore('gear.th_gear')}</Table.TableHead>
          <Table.TableHead>{$tStore('gear.th_category')}</Table.TableHead>
          <Table.TableHead>{$tStore('gear.th_ownership')}</Table.TableHead>
          <Table.TableHead>{$tStore('gear.th_custody')}</Table.TableHead>
          <Table.TableHead>{$tStore('gear.th_status')}</Table.TableHead>
          <Table.TableHead class="text-right">{$tStore('gear.th_actions')}</Table.TableHead>
        </Table.TableRow>
      </Table.TableHeader>
      <Table.TableBody>
        {#each filteredGear as item (item.id)}
          <Table.TableRow>
            <Table.TableCell>
              <div class="flex items-center gap-2">
                <Music class="w-4 h-4 text-primary" />
                <span class="text-xs font-bold text-foreground">{item.name}</span>
              </div>
            </Table.TableCell>
            <Table.TableCell>
              <Badge variant="secondary" class="text-[10px] py-0">
                {item.category}
              </Badge>
            </Table.TableCell>
            <Table.TableCell>
              {#if item.ownership === 'club_property'}
                <Badge variant="outline" class="bg-blue-500/10 text-blue-600 border-blue-500/20 text-[10px] py-0">
                  CSAC Club
                </Badge>
              {:else}
                <Badge variant="outline" class="bg-purple-500/10 text-purple-600 border-purple-500/20 text-[10px] py-0">
                  Owner: {item.ownerName}
                </Badge>
              {/if}
            </Table.TableCell>
            <Table.TableCell>
              <div class="flex flex-col gap-0.5">
                <div class="flex items-center gap-1.5 text-xs font-semibold text-foreground">
                  <UserCheck class="w-3.5 h-3.5 text-emerald-600" />
                  <span>{item.custodianName}</span>
                </div>
                <div class="flex items-center gap-1 text-[11px] text-muted-foreground">
                  <MapPin class="w-3 h-3" />
                  <span>{item.locationNote}</span>
                </div>
              </div>
            </Table.TableCell>
            <Table.TableCell>
              <span class="inline-flex items-center rounded px-2 py-0.5 text-[10px] font-bold border {statusBadgeMap[item.status]}">
                {#if item.status === 'free_to_borrow'}
                  {$tStore('studio.status_free')}
                {:else if item.status === 'in_use'}
                  {$tStore('studio.status_in_use')}
                {:else if item.status === 'in_maintenance'}
                  {$tStore('studio.status_maintenance')}
                {:else}
                  {$tStore('studio.status_unavailable')}
                {/if}
              </span>
            </Table.TableCell>
            <Table.TableCell class="text-right">
              <Button
                variant="outline"
                size="sm"
                class="gap-1.5 text-xs"
                onclick={() => openTransferModal(item)}
              >
                <RefreshCw class="w-3 h-3" />
                <span>{$tStore('gear.btn_transfer')}</span>
              </Button>
            </Table.TableCell>
          </Table.TableRow>
        {/each}
      </Table.TableBody>
    </Table.Root>
  </Card>
</div>

<!-- Modal: Transfer Custody -->
{#if isTransferModalOpen && selectedGear}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) isTransferModalOpen = false; }}>
    <Dialog.Content class="max-w-md">
      <Dialog.Header>
        <div class="flex items-center gap-2">
          <RefreshCw class="w-4 h-4 text-primary" />
          <Dialog.Title class="text-base font-bold">
            {$tStore('gear.modal_title')}
          </Dialog.Title>
        </div>
        <Dialog.Description class="text-xs text-muted-foreground">
          Transfer custody log for {selectedGear.name}
        </Dialog.Description>
      </Dialog.Header>

      <form onsubmit={handleTransferSubmit} class="flex flex-col gap-3 py-2">
        <div class="flex flex-col gap-1.5">
          <Label for="custodian-name" class="text-xs font-semibold">
            {$tStore('gear.form_custodian')}
          </Label>
          <Input
            id="custodian-name"
            type="text"
            bind:value={newCustodianName}
            required
            class="h-8 text-xs"
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="location-note" class="text-xs font-semibold">
            {$tStore('gear.form_location')}
          </Label>
          <Input
            id="location-note"
            type="text"
            bind:value={newLocationNote}
            required
            class="h-8 text-xs"
          />
        </div>

        <Dialog.Footer class="pt-3">
          <Button
            type="button"
            variant="outline"
            size="sm"
            onclick={() => (isTransferModalOpen = false)}
          >
            {$tStore('gear.btn_cancel')}
          </Button>
          <Button type="submit" variant="default" size="sm">
            {$tStore('gear.btn_save_transfer')}
          </Button>
        </Dialog.Footer>
      </form>
    </Dialog.Content>
  </Dialog.Root>
{/if}
