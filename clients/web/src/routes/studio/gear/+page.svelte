<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore } from '$lib/i18n';
  import type {
    GearItem,
    GearCategory,
    GearOwnershipType,
    GearLendingPolicy,
    GearAvailabilityStatus,
    ShowGearAllocation,
    GearShowStatus,
  } from '$lib/types/timetable';
  import {
    Plus,
    Search,
    RefreshCw,
    Shield,
    MapPin,
    UserCheck,
    Music,
    AlertTriangle,
    CheckCircle2,
    Lock,
    Unlock,
    Radio,
    Clock,
    UserX,
    EyeOff,
    Check,
    Phone,
    Calendar,
    ArrowRight,
    Sparkles,
    Guitar,
    Piano,
    Drum,
    Volume2,
    Sliders,
    Layers,
    Cable,
    HelpCircle,
    Info,
    DollarSign,
    QrCode,
    Tag,
    LayoutGrid,
    TableProperties,
  } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import * as Table from '$lib/components/ui/table';
  import * as Dialog from '$lib/components/ui/dialog';

  // Active Tab & View Modes
  let activeTab = $state<'catalog' | 'show_checklist' | 'orphans'>('catalog');
  let catalogViewMode = $state<'cards' | 'table'>('cards');
  let showViewMode = $state<'cards' | 'table'>('cards');

  // Master Gear Catalog
  let gearList = $state<GearItem[]>([
    {
      id: 'inst-1',
      name: 'Fender Player Plus Stratocaster HSS (Club Property)',
      category: 'strings',
      ownership: 'club_property',
      custodianName: 'Minh Pháp',
      locationNote: 'Studio Rehearsal Room A - Rack 1',
      status: 'free_to_borrow',
      lendingPolicy: 'open_to_all',
      serialNumber: 'MX22049182',
      estimatedValueVND: 22000000,
    },
    {
      id: 'inst-2',
      name: 'Yamaha TRBX504 Active 4-String Bass (Hoàng Nam)',
      category: 'strings',
      ownership: 'member_owned',
      ownerName: 'Hoàng Nam',
      ownerId: 'user-nam',
      custodianName: 'Hoàng Nam',
      locationNote: 'CSAC Storage Locker #3',
      status: 'in_use',
      lendingPolicy: 'show_only',
      serialNumber: 'YAM-504-88',
      estimatedValueVND: 14500000,
    },
    {
      id: 'inst-3',
      name: 'Roland RD-88 Stage Piano 88-Keys (Club Property)',
      category: 'keys',
      ownership: 'club_property',
      custodianName: 'Bảo Anh',
      locationNote: 'Auditorium Main Stage Left',
      status: 'in_use',
      lendingPolicy: 'open_to_all',
      serialNumber: 'ROL-RD88-9901',
      estimatedValueVND: 31000000,
    },
    {
      id: 'inst-4',
      name: 'Pearl Export EXX 5-Piece Drum Kit (Club Property)',
      category: 'drums',
      ownership: 'club_property',
      custodianName: 'Thu Hà',
      locationNote: 'Studio Rehearsal Room B - Drum Riser',
      status: 'free_to_borrow',
      lendingPolicy: 'open_to_all',
      estimatedValueVND: 26000000,
    },
    {
      id: 'inst-5',
      name: 'Radial ProDI Passive Direct Box (Duy Anh)',
      category: 'audio_di',
      ownership: 'member_owned',
      ownerName: 'Duy Anh',
      ownerId: 'user-duyanh',
      custodianName: 'Duy Anh',
      locationNote: 'Duy Anh Personal Gig Bag',
      status: 'free_to_borrow',
      lendingPolicy: 'approval_required',
      estimatedValueVND: 3200000,
    },
    {
      id: 'inst-6',
      name: 'Boss Katana-100 MkII Guitar Amp (Club Property)',
      category: 'amps_cabs',
      ownership: 'club_property',
      custodianName: 'Minh Pháp',
      locationNote: 'Studio Rehearsal Room A',
      status: 'in_maintenance',
      lendingPolicy: 'open_to_all',
      estimatedValueVND: 11500000,
      notes: 'Undergoing power jack soldering maintenance',
    },
    {
      id: 'inst-7',
      name: 'Strymon BigSky Reverberator Pedal (Quang Huy)',
      category: 'pedals_fx',
      ownership: 'member_owned',
      ownerName: 'Quang Huy',
      ownerId: 'user-huy',
      custodianName: 'Quang Huy',
      locationNote: 'Huy FX Pedalboard Case',
      status: 'unavailable',
      lendingPolicy: 'locked_private',
      estimatedValueVND: 12000000,
    },
  ]);

  // Live Show Gear Allocations Checklist
  let showAllocations = $state<ShowGearAllocation[]>([
    {
      id: 'alloc-1',
      showId: 'autumn-concert-2026',
      gearId: 'inst-1',
      gearName: 'Fender Player Plus Stratocaster HSS',
      category: 'strings',
      ownership: 'club_property',
      allocatedFor: 'music_number',
      musicNumberTitle: 'Bài Ca Hy Vọng (Lead Guitar)',
      primaryPerformerName: 'Văn Tuấn',
      status: 'active_stage',
      isOnBehalfRetrieval: false,
    },
    {
      id: 'alloc-2',
      showId: 'autumn-concert-2026',
      gearId: 'inst-2',
      gearName: 'Yamaha TRBX504 Active 4-String Bass',
      category: 'strings',
      ownership: 'member_owned',
      ownerName: 'Hoàng Nam',
      allocatedFor: 'music_number',
      musicNumberTitle: 'Hương Mùa Hè (Bass)',
      primaryPerformerName: 'Hoàng Nam',
      status: 'checked_in_venue',
      isOnBehalfRetrieval: false,
    },
    {
      id: 'alloc-3',
      showId: 'autumn-concert-2026',
      gearId: 'inst-3',
      gearName: 'Roland RD-88 Stage Piano 88-Keys',
      category: 'keys',
      ownership: 'club_property',
      allocatedFor: 'backline_common',
      primaryPerformerName: 'Bảo Anh',
      status: 'active_stage',
      isOnBehalfRetrieval: false,
    },
    {
      id: 'alloc-4',
      showId: 'autumn-concert-2026',
      gearId: 'inst-5',
      gearName: 'Radial ProDI Passive Direct Box',
      category: 'audio_di',
      ownership: 'member_owned',
      ownerName: 'Duy Anh',
      allocatedFor: 'sound_desk',
      primaryPerformerName: 'Tech Sound Desk',
      status: 'orphan', // Left behind at venue!
      isOnBehalfRetrieval: false,
    },
    {
      id: 'alloc-5',
      showId: 'autumn-concert-2026',
      gearId: 'inst-7',
      gearName: 'Strymon BigSky Reverberator Pedal',
      category: 'pedals_fx',
      ownership: 'member_owned',
      ownerName: 'Quang Huy',
      allocatedFor: 'emergency_backup',
      primaryPerformerName: 'Stage Backline',
      status: 'retrieved_proxy', // Retrieved on behalf
      isOnBehalfRetrieval: true,
      retrievedByName: 'Trần Đăng (Guitarist)',
      retrievalNote: 'Brought home in gear bag for Quang Huy',
    },
  ]);

  // Catalog Filters & Search
  let search = $state('');
  let filterOwnership = $state<'all' | 'club_property' | 'member_owned'>('all');
  let filterCategory = $state<GearCategory | 'all'>('all');

  // Show Checklist Filter
  let filterFunction = $state<'all' | 'music_number' | 'backline_common' | 'emergency_backup' | 'sound_desk'>('all');

  // Modals state
  let isRegisterModalOpen = $state(false);
  let isTransferModalOpen = $state(false);
  let isProxyModalOpen = $state(false);
  let isAdoptModalOpen = $state(false);
  let toastNotification = $state<string | null>(null);

  // Transfer Form State
  let selectedGearForTransfer = $state<GearItem | null>(null);
  let newCustodianName = $state('');
  let newLocationNote = $state('');

  // Register Form State
  let newGearName = $state('');
  let newGearCategory = $state<GearCategory>('strings');
  let newGearOwnership = $state<GearOwnershipType>('club_property');
  let newGearOwnerName = $state('');
  let newGearCustodian = $state('');
  let newGearLocation = $state('');
  let newGearPolicy = $state<GearLendingPolicy>('open_to_all');
  let newGearSerial = $state('');
  let newGearEstimatedValue = $state<number>(0);

  // Proxy Retrieval Form State
  let selectedAllocationForProxy = $state<ShowGearAllocation | null>(null);
  let proxyRetrieverName = $state('');
  let proxyRetrievalNote = $state('');

  // Adopt Orphan Form State
  let selectedAllocationForAdopt = $state<ShowGearAllocation | null>(null);
  let adopterName = $state('');
  let adopterPhone = $state('');
  let targetReturnDate = $state('');
  let pickupLocationNote = $state('');

  // Derived Values
  let filteredGear = $derived(
    gearList.filter((g) => {
      const matchSearch =
        g.name.toLowerCase().includes(search.toLowerCase()) ||
        g.category.toLowerCase().includes(search.toLowerCase()) ||
        g.custodianName.toLowerCase().includes(search.toLowerCase()) ||
        (g.ownerName && g.ownerName.toLowerCase().includes(search.toLowerCase()));
      const matchOwnership =
        filterOwnership === 'all' || g.ownership === filterOwnership;
      const matchCategory =
        filterCategory === 'all' || g.category === filterCategory;
      return matchSearch && matchOwnership && matchCategory;
    })
  );

  let filteredShowAllocations = $derived(
    showAllocations.filter((a) => {
      return filterFunction === 'all' || a.allocatedFor === filterFunction;
    })
  );

  // Orphan Safeguard Metrics
  let orphanCount = $derived(showAllocations.filter((a) => a.status === 'orphan').length);
  let activeShowCount = $derived(showAllocations.filter((a) => a.status === 'active_stage' || a.status === 'checked_in_venue').length);
  let totalClubGear = $derived(gearList.filter((g) => g.ownership === 'club_property').length);
  let totalMemberGear = $derived(gearList.filter((g) => g.ownership === 'member_owned').length);

  function showToast(msg: string) {
    toastNotification = msg;
    setTimeout(() => {
      if (toastNotification === msg) toastNotification = null;
    }, 4000);
  }

  // Action Handlers
  function openTransferModal(gear: GearItem) {
    selectedGearForTransfer = gear;
    newCustodianName = gear.custodianName;
    newLocationNote = gear.locationNote;
    isTransferModalOpen = true;
  }

  function handleTransferSubmit(e: Event) {
    e.preventDefault();
    if (!selectedGearForTransfer) return;

    gearList = gearList.map((g) =>
      g.id === selectedGearForTransfer?.id
        ? {
            ...g,
            custodianName: newCustodianName,
            locationNote: newLocationNote,
          }
        : g
    );

    isTransferModalOpen = false;
    selectedGearForTransfer = null;
    showToast($tStore('gear.success_transfer'));
  }

  function handleRegisterSubmit(e: Event) {
    e.preventDefault();
    if (!newGearName.trim()) return;

    const newItem: GearItem = {
      id: `inst-${Date.now()}`,
      name: newGearName.trim(),
      category: newGearCategory,
      ownership: newGearOwnership,
      ownerName: newGearOwnership === 'member_owned' ? newGearOwnerName.trim() : undefined,
      custodianName: newGearCustodian.trim() || (newGearOwnership === 'member_owned' ? newGearOwnerName.trim() : 'CSAC Club Storage'),
      locationNote: newGearLocation.trim() || 'Club Studio Storage',
      status: 'free_to_borrow',
      lendingPolicy: newGearPolicy,
      serialNumber: newGearSerial.trim() || undefined,
      estimatedValueVND: newGearEstimatedValue > 0 ? newGearEstimatedValue : undefined,
    };

    gearList = [newItem, ...gearList];
    isRegisterModalOpen = false;
    // reset form
    newGearName = '';
    newGearOwnerName = '';
    newGearCustodian = '';
    newGearLocation = '';
    newGearSerial = '';
    newGearEstimatedValue = 0;
    showToast($tStore('gear.success_registered'));
  }

  function toggleRevokeGear(gear: GearItem) {
    gearList = gearList.map((g) =>
      g.id === gear.id
        ? {
            ...g,
            isRevoked: !g.isRevoked,
            status: !g.isRevoked ? 'unavailable' : 'free_to_borrow',
          }
        : g
    );
    showToast($tStore('gear.success_revoked'));
  }

  // Show Checklist Actions
  function handleCheckIn(allocation: ShowGearAllocation) {
    showAllocations = showAllocations.map((a) =>
      a.id === allocation.id ? { ...a, status: 'checked_in_venue', checkInTimestamp: new Date().toLocaleTimeString() } : a
    );
  }

  function handleStageActive(allocation: ShowGearAllocation) {
    showAllocations = showAllocations.map((a) =>
      a.id === allocation.id ? { ...a, status: 'active_stage' } : a
    );
  }

  function handleOwnerRetrieve(allocation: ShowGearAllocation) {
    showAllocations = showAllocations.map((a) =>
      a.id === allocation.id
        ? {
            ...a,
            status: 'retrieved_owner',
            retrievalTimestamp: new Date().toLocaleTimeString(),
            retrievedByName: a.ownerName || a.primaryPerformerName || 'Owner',
            isOnBehalfRetrieval: false,
          }
        : a
    );
    showToast($tStore('gear.success_retrieved'));
  }

  function openProxyRetrieveModal(allocation: ShowGearAllocation) {
    selectedAllocationForProxy = allocation;
    proxyRetrieverName = '';
    proxyRetrievalNote = '';
    isProxyModalOpen = true;
  }

  function handleProxyRetrieveSubmit(e: Event) {
    e.preventDefault();
    if (!selectedAllocationForProxy || !proxyRetrieverName.trim()) return;

    showAllocations = showAllocations.map((a) =>
      a.id === selectedAllocationForProxy?.id
        ? {
            ...a,
            status: 'retrieved_proxy',
            retrievalTimestamp: new Date().toLocaleTimeString(),
            retrievedByName: proxyRetrieverName.trim(),
            isOnBehalfRetrieval: true,
            retrievalNote: proxyRetrievalNote.trim(),
          }
        : a
    );

    isProxyModalOpen = false;
    selectedAllocationForProxy = null;
    showToast($tStore('gear.success_retrieved'));
  }

  function handleMarkOrphan(allocation: ShowGearAllocation) {
    showAllocations = showAllocations.map((a) =>
      a.id === allocation.id ? { ...a, status: 'orphan' } : a
    );
    activeTab = 'orphans';
  }

  function openAdoptModal(allocation: ShowGearAllocation) {
    selectedAllocationForAdopt = allocation;
    adopterName = '';
    adopterPhone = '';
    targetReturnDate = '';
    pickupLocationNote = '';
    isAdoptModalOpen = true;
  }

  function handleAdoptSubmit(e: Event) {
    e.preventDefault();
    if (!selectedAllocationForAdopt || !adopterName.trim() || !adopterPhone.trim()) return;

    showAllocations = showAllocations.map((a) =>
      a.id === selectedAllocationForAdopt?.id
        ? {
            ...a,
            status: 'adopted',
            adoption: {
              adopterName: adopterName.trim(),
              adopterPhone: adopterPhone.trim(),
              adoptedAt: new Date().toLocaleString(),
              targetReturnDate: targetReturnDate.trim() || 'Tomorrow Afternoon',
              ownerNotified: true,
              pickupLocationNote: pickupLocationNote.trim(),
            },
          }
        : a
    );

    isAdoptModalOpen = false;
    selectedAllocationForAdopt = null;
    showToast($tStore('gear.success_adopted'));
  }

  function handleCloseShow() {
    if (orphanCount > 0) return;
    showToast($tStore('gear.success_show_closed'));
  }

  // Category Icon helper
  function getCategoryIcon(cat: GearCategory) {
    switch (cat) {
      case 'strings':
        return Guitar;
      case 'keys':
        return Piano;
      case 'drums':
        return Drum;
      case 'amps_cabs':
        return Volume2;
      case 'pedals_fx':
        return Sliders;
      case 'audio_di':
        return Layers;
      case 'cables_accessories':
        return Cable;
      default:
        return Music;
    }
  }

  const statusBadgeMap: Record<GearAvailabilityStatus, string> = {
    free_to_borrow: 'bg-emerald-500/10 text-emerald-600 border-emerald-500/20',
    in_use: 'bg-blue-500/10 text-blue-600 border-blue-500/20',
    in_maintenance: 'bg-amber-500/10 text-amber-600 border-amber-500/20',
    unavailable: 'bg-destructive/10 text-destructive border-destructive/20',
  };

  const showStatusBadgeMap: Record<GearShowStatus, { bg: string; text: string; label: string }> = {
    allocated_main: { bg: 'bg-slate-500/10 border-slate-500/20', text: 'text-slate-600 dark:text-slate-400', label: '1. Đã phân bổ' },
    allocated_backup: { bg: 'bg-indigo-500/10 border-indigo-500/20', text: 'text-indigo-600 dark:text-indigo-400', label: '1. Dự phòng' },
    checked_in_venue: { bg: 'bg-amber-500/10 border-amber-500/20', text: 'text-amber-600 dark:text-amber-400', label: '2. Có mặt tại Show' },
    active_stage: { bg: 'bg-blue-500/10 border-blue-500/20', text: 'text-blue-600 dark:text-blue-400', label: '3. Đang trên sân khấu' },
    retrieved_owner: { bg: 'bg-emerald-500/10 border-emerald-500/20', text: 'text-emerald-600 dark:text-emerald-400', label: '4. Chính chủ đã thu hồi' },
    retrieved_proxy: { bg: 'bg-purple-500/10 border-purple-500/20', text: 'text-purple-600 dark:text-purple-400', label: '4. Thu hồi hộ (Proxy)' },
    orphan: { bg: 'bg-rose-500/20 border-rose-500 text-rose-700 animate-pulse font-extrabold', text: 'text-rose-600', label: '🚨 BỎ QUÊN (ORPHAN)' },
    adopted: { bg: 'bg-teal-500/10 border-teal-500/20', text: 'text-teal-700 dark:text-teal-400', label: '🛡️ Đã nhận giữ hộ' },
  };

  const categoryOptions: { id: GearCategory; labelKey: string; icon: any }[] = [
    { id: 'strings', labelKey: 'gear.cat_strings', icon: Guitar },
    { id: 'keys', labelKey: 'gear.cat_keys', icon: Piano },
    { id: 'drums', labelKey: 'gear.cat_drums', icon: Drum },
    { id: 'amps_cabs', labelKey: 'gear.cat_amps_cabs', icon: Volume2 },
    { id: 'pedals_fx', labelKey: 'gear.cat_pedals_fx', icon: Sliders },
    { id: 'audio_di', labelKey: 'gear.cat_audio_di', icon: Layers },
    { id: 'cables_accessories', labelKey: 'gear.cat_cables_accessories', icon: Cable },
  ];

  const policyOptions: { id: GearLendingPolicy; labelKey: string; desc: string; icon: any }[] = [
    { id: 'open_to_all', labelKey: 'gear.policy_open_to_all', desc: 'Any verified club member can reserve for rehearsals', icon: CheckCircle2 },
    { id: 'approval_required', labelKey: 'gear.policy_approval_required', desc: 'Owner must approve every booking request', icon: UserCheck },
    { id: 'show_only', labelKey: 'gear.policy_show_only', desc: 'Reserved only for official concert stages', icon: Radio },
    { id: 'locked_private', labelKey: 'gear.policy_locked_private', desc: 'Owner personal gear, locked from club loan', icon: Lock },
  ];
</script>

<svelte:head>
  <title>{$tStore('gear.page_title')}</title>
</svelte:head>

<Navbar />

<div class="mx-auto flex max-w-7xl flex-col gap-6 p-4 sm:p-6">
  <!-- Toast Notification -->
  {#if toastNotification}
    <div class="fixed bottom-5 right-5 z-50 flex items-center gap-2.5 rounded-2xl border border-primary/30 bg-card/95 backdrop-blur-md px-4 py-3 shadow-2xl text-xs font-bold text-foreground animate-in slide-in-from-bottom-5">
      <div class="p-1 rounded-lg bg-primary/20 text-primary">
        <Sparkles class="w-4 h-4" />
      </div>
      <span>{toastNotification}</span>
    </div>
  {/if}

  <!-- Header Banner -->
  <Card class="flex flex-col gap-4 rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card p-6 sm:p-8 shadow-sm md:flex-row md:items-center md:justify-between transition-all">
    <div class="flex flex-col gap-2">
      <div class="flex items-center gap-2 flex-wrap">
        <Badge variant="outline" class="w-fit bg-primary/10 text-primary border-primary/25 gap-1.5 px-3 py-1 font-bold rounded-full text-xs">
          <Shield class="w-3.5 h-3.5 text-primary" />
          <span>Independent Fleet Governance</span>
        </Badge>
        <span class="text-xs text-muted-foreground">• Dual-Ownership & Show Teardown</span>
      </div>
      <h1 class="text-2xl sm:text-3xl font-black tracking-tight text-foreground">
        {$tStore('gear.heading')}
      </h1>
      <p class="text-xs sm:text-sm text-muted-foreground max-w-2xl">
        {$tStore('gear.subheading')}
      </p>
    </div>

    <div class="flex items-center gap-3 shrink-0">
      <Button
        variant="default"
        size="default"
        class="h-11 px-5 rounded-2xl font-black gap-2 shadow-lg shadow-primary/25 bg-primary hover:bg-primary/90 text-primary-foreground hover:scale-[1.02] active:scale-[0.98] transition-all"
        onclick={() => (isRegisterModalOpen = true)}
      >
        <Plus class="w-4 h-4 stroke-[3]" />
        <span>{$tStore('gear.btn_register')}</span>
      </Button>
    </div>
  </Card>

  <!-- High Priority Critical Alert: Orphan Gear Banner -->
  {#if orphanCount > 0}
    <div class="rounded-3xl border-2 border-rose-500 bg-rose-500/10 p-5 sm:p-6 shadow-xl flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 animate-in fade-in duration-300">
      <div class="flex items-start gap-3.5">
        <div class="p-2.5 rounded-2xl bg-rose-500/20 text-rose-600 shrink-0">
          <AlertTriangle class="w-6 h-6 text-rose-600 animate-bounce" />
        </div>
        <div class="flex flex-col gap-1">
          <h3 class="text-sm sm:text-base font-black text-rose-700 dark:text-rose-400 tracking-tight">
            {$tStore('gear.alert_orphan_title')}
          </h3>
          <p class="text-xs sm:text-sm text-rose-600/90 dark:text-rose-300/90 font-medium">
            {$tStore('gear.alert_orphan_desc', { count: orphanCount })}
          </p>
        </div>
      </div>
      <Button
        variant="destructive"
        size="sm"
        class="h-10 px-4 rounded-xl font-black gap-2 shrink-0 shadow-md"
        onclick={() => (activeTab = 'orphans')}
      >
        <span>{$tStore('gear.tab_orphans')} ({orphanCount})</span>
        <ArrowRight class="w-4 h-4" />
      </Button>
    </div>
  {/if}

  <!-- Metric Quick Tiles -->
  <div class="grid grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4">
    <Card class="rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card p-5 shadow-sm flex flex-col justify-between gap-3 hover:shadow-md transition-all">
      <div class="flex items-center justify-between">
        <span class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
          {$tStore('gear.stat_total_gear')}
        </span>
        <div class="p-2 rounded-xl bg-primary/10 text-primary">
          <Music class="w-4 h-4" />
        </div>
      </div>
      <div class="flex items-baseline gap-2">
        <span class="text-3xl font-black text-foreground tracking-tight">{gearList.length}</span>
        <span class="text-xs text-muted-foreground font-medium">items</span>
      </div>
    </Card>

    <Card class="rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card p-5 shadow-sm flex flex-col justify-between gap-3 hover:shadow-md transition-all">
      <div class="flex items-center justify-between">
        <span class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
          {$tStore('gear.stat_club_property')}
        </span>
        <div class="p-2 rounded-xl bg-blue-500/10 text-blue-600">
          <Shield class="w-4 h-4" />
        </div>
      </div>
      <div class="flex items-baseline gap-2">
        <span class="text-3xl font-black text-blue-600 tracking-tight">{totalClubGear}</span>
        <Badge variant="outline" class="text-[10px] bg-blue-500/10 text-blue-600 border-blue-500/25 font-bold">CSAC Owned</Badge>
      </div>
    </Card>

    <Card class="rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card p-5 shadow-sm flex flex-col justify-between gap-3 hover:shadow-md transition-all">
      <div class="flex items-center justify-between">
        <span class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
          {$tStore('gear.stat_member_owned')}
        </span>
        <div class="p-2 rounded-xl bg-purple-500/10 text-purple-600">
          <UserCheck class="w-4 h-4" />
        </div>
      </div>
      <div class="flex items-baseline gap-2">
        <span class="text-3xl font-black text-purple-600 tracking-tight">{totalMemberGear}</span>
        <Badge variant="outline" class="text-[10px] bg-purple-500/10 text-purple-600 border-purple-500/25 font-bold">Member Lent</Badge>
      </div>
    </Card>

    <Card class="rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card p-5 shadow-sm flex flex-col justify-between gap-3 hover:shadow-md transition-all {orphanCount > 0 ? 'border-rose-500/40 bg-rose-500/5' : ''}">
      <div class="flex items-center justify-between">
        <span class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
          {$tStore('gear.stat_orphans_active')}
        </span>
        <div class="p-2 rounded-xl {orphanCount > 0 ? 'bg-rose-500/20 text-rose-600 animate-pulse' : 'bg-emerald-500/10 text-emerald-600'}">
          <AlertTriangle class="w-4 h-4" />
        </div>
      </div>
      <div class="flex items-baseline gap-2">
        <span class="text-3xl font-black tracking-tight {orphanCount > 0 ? 'text-rose-600' : 'text-emerald-600'}">
          {orphanCount}
        </span>
        {#if orphanCount > 0}
          <Badge variant="destructive" class="text-[10px] font-black animate-pulse">ACTION REQUIRED</Badge>
        {:else}
          <span class="text-xs text-emerald-600 font-semibold flex items-center gap-1">
            <CheckCircle2 class="w-3.5 h-3.5" /> All Safe
          </span>
        {/if}
      </div>
    </Card>
  </div>

  <!-- Workstation Navigation Tabs -->
  <div class="flex items-center gap-2 border-b border-border/60 pb-2 overflow-x-auto">
    <Button
      variant={activeTab === 'catalog' ? 'default' : 'ghost'}
      size="sm"
      class="font-black gap-2 text-xs rounded-2xl h-9 px-4 transition-all"
      onclick={() => (activeTab = 'catalog')}
    >
      <Layers class="w-3.5 h-3.5" />
      <span>{$tStore('gear.tab_catalog')}</span>
    </Button>
    <Button
      variant={activeTab === 'show_checklist' ? 'default' : 'ghost'}
      size="sm"
      class="font-black gap-2 text-xs rounded-2xl h-9 px-4 transition-all"
      onclick={() => (activeTab = 'show_checklist')}
    >
      <Radio class="w-3.5 h-3.5" />
      <span>{$tStore('gear.tab_show_checklist')}</span>
      {#if activeShowCount > 0}
        <Badge variant="secondary" class="text-[10px] py-0 px-1.5 ml-1 bg-primary/20 text-primary font-bold">
          {activeShowCount}
        </Badge>
      {/if}
    </Button>
    <Button
      variant={activeTab === 'orphans' ? 'destructive' : 'ghost'}
      size="sm"
      class="font-black gap-2 text-xs rounded-2xl h-9 px-4 transition-all relative {orphanCount > 0 && activeTab !== 'orphans' ? 'text-rose-600 font-black' : ''}"
      onclick={() => (activeTab = 'orphans')}
    >
      <AlertTriangle class="w-3.5 h-3.5" />
      <span>{$tStore('gear.tab_orphans')}</span>
      {#if orphanCount > 0}
        <span class="flex h-2 w-2 relative">
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-rose-400 opacity-75"></span>
          <span class="relative inline-flex rounded-full h-2 w-2 bg-rose-500"></span>
        </span>
      {/if}
    </Button>
  </div>

  <!-- TAB 1: EQUIPMENT CATALOG -->
  {#if activeTab === 'catalog'}
    <!-- Filter & Search & View Mode Toolbar -->
    <Card class="flex flex-col gap-3 rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card p-4 shadow-sm md:flex-row md:items-center md:justify-between">
      <div class="flex items-center gap-2 flex-wrap">
        <Button
          variant={filterOwnership === 'all' ? 'default' : 'outline'}
          size="sm"
          class="text-xs h-8 rounded-xl font-bold"
          onclick={() => (filterOwnership = 'all')}
        >
          {$tStore('gear.filter_all')}
        </Button>
        <Button
          variant={filterOwnership === 'club_property' ? 'default' : 'outline'}
          size="sm"
          class="text-xs h-8 rounded-xl font-bold"
          onclick={() => (filterOwnership = 'club_property')}
        >
          {$tStore('gear.filter_club')}
        </Button>
        <Button
          variant={filterOwnership === 'member_owned' ? 'default' : 'outline'}
          size="sm"
          class="text-xs h-8 rounded-xl font-bold"
          onclick={() => (filterOwnership = 'member_owned')}
        >
          {$tStore('gear.filter_member')}
        </Button>
      </div>

      <div class="flex items-center gap-2.5 w-full md:w-auto justify-between md:justify-end">
        <div class="relative w-full sm:w-64">
          <Search class="pointer-events-none absolute left-3 top-2.5 w-4 h-4 text-muted-foreground" />
          <Input
            type="text"
            placeholder={$tStore('gear.search_placeholder')}
            bind:value={search}
            class="h-9 pl-9 text-xs rounded-2xl bg-muted/30 border-border/80 focus:bg-background"
          />
        </div>

        <!-- View Switcher -->
        <div class="flex items-center rounded-2xl border border-border/80 bg-muted/30 p-1 shrink-0">
          <button
            type="button"
            class="flex items-center gap-1.5 px-3 py-1 text-xs font-bold rounded-xl transition-all {catalogViewMode === 'cards' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => (catalogViewMode = 'cards')}
          >
            <LayoutGrid class="w-3.5 h-3.5" />
            <span class="hidden sm:inline">{$tStore('gear.view_cards')}</span>
          </button>
          <button
            type="button"
            class="flex items-center gap-1.5 px-3 py-1 text-xs font-bold rounded-xl transition-all {catalogViewMode === 'table' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => (catalogViewMode = 'table')}
          >
            <TableProperties class="w-3.5 h-3.5" />
            <span class="hidden sm:inline">{$tStore('gear.view_table')}</span>
          </button>
        </div>
      </div>
    </Card>

    <!-- VIEW 1: BENTO GRID CARDS (Default & Zero Horizontal Overflow) -->
    {#if catalogViewMode === 'cards'}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each filteredGear as item (item.id)}
          <Card class="flex flex-col justify-between rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card p-5 shadow-sm hover:shadow-md transition-all gap-4 {item.isRevoked ? 'opacity-65 bg-muted/10' : ''}">
            <!-- Top Card Row -->
            <div class="flex flex-col gap-3">
              <div class="flex items-start justify-between gap-2">
                <div class="flex items-center gap-3">
                  <div class="flex h-11 w-11 items-center justify-center rounded-2xl bg-primary/10 text-primary shrink-0 shadow-sm">
                    <svelte:component this={getCategoryIcon(item.category)} class="h-5 w-5" />
                  </div>
                  <div class="flex flex-col">
                    <span class="text-sm font-black text-foreground line-clamp-1">{item.name}</span>
                    <div class="flex items-center gap-2 mt-0.5">
                      <Badge variant="secondary" class="text-[10px] py-0 px-1.5 font-medium rounded-md">
                        {$tStore(`gear.cat_${item.category}`)}
                      </Badge>
                      {#if item.serialNumber}
                        <span class="text-[10px] font-mono text-muted-foreground">SN: {item.serialNumber}</span>
                      {/if}
                    </div>
                  </div>
                </div>
              </div>

              <!-- Badges Strip -->
              <div class="flex items-center gap-1.5 flex-wrap">
                {#if item.ownership === 'club_property'}
                  <Badge variant="outline" class="bg-blue-500/10 text-blue-600 border-blue-500/25 text-[10px] py-0.5 px-2 font-bold rounded-lg">
                    CSAC Club Property
                  </Badge>
                {:else}
                  <Badge variant="outline" class="bg-purple-500/10 text-purple-600 border-purple-500/25 text-[10px] py-0.5 px-2 font-bold rounded-lg">
                    Owner: {item.ownerName}
                  </Badge>
                {/if}

                {#if item.isRevoked}
                  <Badge variant="destructive" class="text-[10px] py-0.5 px-2 font-bold rounded-lg">
                    {$tStore('gear.status_revoked')}
                  </Badge>
                {:else if item.lendingPolicy === 'open_to_all'}
                  <Badge variant="outline" class="bg-emerald-500/10 text-emerald-600 border-emerald-500/25 text-[10px] py-0.5 px-2 rounded-lg font-medium">
                    {$tStore('gear.policy_open_to_all')}
                  </Badge>
                {:else if item.lendingPolicy === 'approval_required'}
                  <Badge variant="outline" class="bg-amber-500/10 text-amber-600 border-amber-500/25 text-[10px] py-0.5 px-2 rounded-lg font-medium">
                    {$tStore('gear.policy_approval_required')}
                  </Badge>
                {:else if item.lendingPolicy === 'show_only'}
                  <Badge variant="outline" class="bg-blue-500/10 text-blue-600 border-blue-500/25 text-[10px] py-0.5 px-2 rounded-lg font-medium">
                    {$tStore('gear.policy_show_only')}
                  </Badge>
                {:else}
                  <Badge variant="outline" class="bg-rose-500/10 text-rose-600 border-rose-500/25 text-[10px] py-0.5 px-2 rounded-lg font-medium">
                    {$tStore('gear.policy_locked_private')}
                  </Badge>
                {/if}
              </div>

              <!-- Custody & Location Surface -->
              <div class="rounded-2xl border border-border/70 bg-muted/25 p-3 flex flex-col gap-1.5 text-xs">
                <div class="flex items-center justify-between">
                  <span class="text-muted-foreground font-medium">{$tStore('gear.lbl_custodian')}</span>
                  <div class="flex items-center gap-1.5 font-bold text-foreground">
                    <UserCheck class="w-3.5 h-3.5 text-emerald-600" />
                    <span>{item.custodianName}</span>
                  </div>
                </div>
                <div class="flex items-center justify-between border-t border-border/50 pt-1.5">
                  <span class="text-muted-foreground font-medium">{$tStore('gear.lbl_location')}</span>
                  <div class="flex items-center gap-1 text-foreground font-medium truncate max-w-[190px]">
                    <MapPin class="w-3 h-3 text-primary shrink-0" />
                    <span class="truncate">{item.locationNote}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Card Bottom Action Row -->
            <div class="flex items-center justify-between pt-3 border-t border-border/60">
              <span class="inline-flex items-center rounded-lg px-2.5 py-1 text-[11px] font-bold border {statusBadgeMap[item.status]}">
                {#if item.status === 'free_to_borrow'}
                  {$tStore('gear.status_free')}
                {:else if item.status === 'in_use'}
                  {$tStore('gear.status_in_use')}
                {:else if item.status === 'in_maintenance'}
                  {$tStore('gear.status_in_maintenance')}
                {:else}
                  {$tStore('gear.status_unavailable')}
                {/if}
              </span>

              <div class="flex items-center gap-1.5">
                <Button
                  variant="outline"
                  size="sm"
                  class="gap-1.5 text-xs h-8 px-3 rounded-xl font-bold"
                  onclick={() => openTransferModal(item)}
                >
                  <RefreshCw class="w-3.5 h-3.5" />
                  <span>{$tStore('gear.btn_transfer')}</span>
                </Button>
                {#if item.ownership === 'member_owned'}
                  <Button
                    variant="ghost"
                    size="sm"
                    class="text-xs h-8 px-2.5 rounded-xl font-bold {item.isRevoked ? 'text-emerald-600' : 'text-muted-foreground'}"
                    onclick={() => toggleRevokeGear(item)}
                  >
                    {#if item.isRevoked}
                      <Unlock class="w-3.5 h-3.5" />
                    {:else}
                      <Lock class="w-3.5 h-3.5" />
                    {/if}
                  </Button>
                {/if}
              </div>
            </div>
          </Card>
        {/each}
      </div>
    {:else}
      <!-- VIEW 2: HIGH-DENSITY COMPACT TABLE WITH OVERFLOW PROTECTION -->
      <Card class="rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card shadow-sm overflow-hidden p-0">
        <div class="overflow-x-auto w-full">
          <Table.Root class="min-w-[800px]">
            <Table.TableHeader>
              <Table.TableRow class="bg-muted/40">
                <Table.TableHead class="text-xs font-bold text-foreground">{$tStore('gear.th_gear')}</Table.TableHead>
                <Table.TableHead class="text-xs font-bold text-foreground">{$tStore('gear.th_category')}</Table.TableHead>
                <Table.TableHead class="text-xs font-bold text-foreground">{$tStore('gear.th_ownership')}</Table.TableHead>
                <Table.TableHead class="text-xs font-bold text-foreground">{$tStore('gear.th_policy')}</Table.TableHead>
                <Table.TableHead class="text-xs font-bold text-foreground">{$tStore('gear.th_custody')}</Table.TableHead>
                <Table.TableHead class="text-xs font-bold text-foreground">{$tStore('gear.th_status')}</Table.TableHead>
                <Table.TableHead class="text-right text-xs font-bold text-foreground">{$tStore('gear.th_actions')}</Table.TableHead>
              </Table.TableRow>
            </Table.TableHeader>
            <Table.TableBody>
              {#each filteredGear as item (item.id)}
                <Table.TableRow class={item.isRevoked ? 'opacity-60 bg-muted/20' : ''}>
                  <Table.TableCell>
                    <div class="flex flex-col gap-0.5">
                      <div class="flex items-center gap-2">
                        <div class="p-1.5 rounded-lg bg-primary/10 text-primary shrink-0">
                          <svelte:component this={getCategoryIcon(item.category)} class="w-3.5 h-3.5" />
                        </div>
                        <span class="text-xs font-bold text-foreground">{item.name}</span>
                      </div>
                      {#if item.serialNumber}
                        <span class="text-[10px] text-muted-foreground pl-7 font-mono">SN: {item.serialNumber}</span>
                      {/if}
                    </div>
                  </Table.TableCell>
                  <Table.TableCell>
                    <Badge variant="secondary" class="text-[10px] py-0.5 font-medium rounded-lg whitespace-nowrap">
                      {$tStore(`gear.cat_${item.category}`)}
                    </Badge>
                  </Table.TableCell>
                  <Table.TableCell>
                    {#if item.ownership === 'club_property'}
                      <Badge variant="outline" class="bg-blue-500/10 text-blue-600 border-blue-500/25 text-[10px] py-0.5 font-bold rounded-lg whitespace-nowrap">
                        CSAC Club
                      </Badge>
                    {:else}
                      <Badge variant="outline" class="bg-purple-500/10 text-purple-600 border-purple-500/25 text-[10px] py-0.5 font-bold rounded-lg whitespace-nowrap">
                        Owner: {item.ownerName}
                      </Badge>
                    {/if}
                  </Table.TableCell>
                  <Table.TableCell>
                    {#if item.isRevoked}
                      <Badge variant="destructive" class="text-[10px] py-0.5 rounded-lg whitespace-nowrap">
                        {$tStore('gear.status_revoked')}
                      </Badge>
                    {:else if item.lendingPolicy === 'open_to_all'}
                      <Badge variant="outline" class="bg-emerald-500/10 text-emerald-600 border-emerald-500/25 text-[10px] py-0.5 rounded-lg font-medium whitespace-nowrap">
                        {$tStore('gear.policy_open_to_all')}
                      </Badge>
                    {:else if item.lendingPolicy === 'approval_required'}
                      <Badge variant="outline" class="bg-amber-500/10 text-amber-600 border-amber-500/25 text-[10px] py-0.5 rounded-lg font-medium whitespace-nowrap">
                        {$tStore('gear.policy_approval_required')}
                      </Badge>
                    {:else if item.lendingPolicy === 'show_only'}
                      <Badge variant="outline" class="bg-blue-500/10 text-blue-600 border-blue-500/25 text-[10px] py-0.5 rounded-lg font-medium whitespace-nowrap">
                        {$tStore('gear.policy_show_only')}
                      </Badge>
                    {:else}
                      <Badge variant="outline" class="bg-rose-500/10 text-rose-600 border-rose-500/25 text-[10px] py-0.5 rounded-lg font-medium whitespace-nowrap">
                        {$tStore('gear.policy_locked_private')}
                      </Badge>
                    {/if}
                  </Table.TableCell>
                  <Table.TableCell>
                    <div class="flex flex-col gap-0.5">
                      <div class="flex items-center gap-1.5 text-xs font-semibold text-foreground whitespace-nowrap">
                        <UserCheck class="w-3.5 h-3.5 text-emerald-600 shrink-0" />
                        <span>{item.custodianName}</span>
                      </div>
                      <div class="flex items-center gap-1 text-[11px] text-muted-foreground whitespace-nowrap">
                        <MapPin class="w-3 h-3 shrink-0" />
                        <span class="truncate max-w-[150px]">{item.locationNote}</span>
                      </div>
                    </div>
                  </Table.TableCell>
                  <Table.TableCell>
                    <span class="inline-flex items-center rounded-lg px-2 py-0.5 text-[10px] font-bold border whitespace-nowrap {statusBadgeMap[item.status]}">
                      {#if item.status === 'free_to_borrow'}
                        {$tStore('gear.status_free')}
                      {:else if item.status === 'in_use'}
                        {$tStore('gear.status_in_use')}
                      {:else if item.status === 'in_maintenance'}
                        {$tStore('gear.status_in_maintenance')}
                      {:else}
                        {$tStore('gear.status_unavailable')}
                      {/if}
                    </span>
                  </Table.TableCell>
                  <Table.TableCell class="text-right">
                    <div class="flex items-center justify-end gap-1.5 whitespace-nowrap">
                      <Button
                        variant="outline"
                        size="sm"
                        class="gap-1 text-[11px] h-7 px-2.5 rounded-xl font-bold"
                        onclick={() => openTransferModal(item)}
                      >
                        <RefreshCw class="w-3 h-3" />
                        <span>{$tStore('gear.btn_transfer')}</span>
                      </Button>
                      {#if item.ownership === 'member_owned'}
                        <Button
                          variant="ghost"
                          size="sm"
                          class="text-[11px] h-7 px-2 rounded-xl font-bold {item.isRevoked ? 'text-emerald-600' : 'text-muted-foreground'}"
                          onclick={() => toggleRevokeGear(item)}
                        >
                          {#if item.isRevoked}
                            <Unlock class="w-3 h-3 mr-1" />
                            <span>{$tStore('gear.btn_restore')}</span>
                          {:else}
                            <Lock class="w-3 h-3 mr-1" />
                            <span>{$tStore('gear.btn_revoke')}</span>
                          {/if}
                        </Button>
                      {/if}
                    </div>
                  </Table.TableCell>
                </Table.TableRow>
              {/each}
            </Table.TableBody>
          </Table.Root>
        </div>
      </Card>
    {/if}
  {/if}

  <!-- TAB 2: LIVE SHOW CUSTODY CHECKLIST -->
  {#if activeTab === 'show_checklist'}
    <Card class="flex flex-col gap-4 rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card p-6 shadow-sm">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h2 class="text-base sm:text-lg font-black text-foreground tracking-tight">
            {$tStore('gear.show_checklist_title')}
          </h2>
          <p class="text-xs sm:text-sm text-muted-foreground">
            {$tStore('gear.show_checklist_desc')}
          </p>
        </div>

        <div class="flex items-center gap-2.5">
          <!-- View Switcher -->
          <div class="flex items-center rounded-2xl border border-border/80 bg-muted/30 p-1 shrink-0">
            <button
              type="button"
              class="flex items-center gap-1.5 px-3 py-1 text-xs font-bold rounded-xl transition-all {showViewMode === 'cards' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
              onclick={() => (showViewMode = 'cards')}
            >
              <LayoutGrid class="w-3.5 h-3.5" />
              <span class="hidden sm:inline">{$tStore('gear.view_cards')}</span>
            </button>
            <button
              type="button"
              class="flex items-center gap-1.5 px-3 py-1 text-xs font-bold rounded-xl transition-all {showViewMode === 'table' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
              onclick={() => (showViewMode = 'table')}
            >
              <TableProperties class="w-3.5 h-3.5" />
              <span class="hidden sm:inline">{$tStore('gear.view_table')}</span>
            </button>
          </div>

          <!-- Close Show Action Button with Hard Safeguard -->
          <Button
            variant={orphanCount > 0 ? 'secondary' : 'default'}
            size="sm"
            disabled={orphanCount > 0}
            class="h-10 px-4 rounded-2xl font-black gap-2 {orphanCount > 0 ? 'opacity-50 cursor-not-allowed' : 'bg-emerald-600 hover:bg-emerald-700 text-white shadow-md'}"
            onclick={handleCloseShow}
          >
            <CheckCircle2 class="w-4 h-4" />
            <span>{$tStore('gear.btn_close_show')}</span>
          </Button>
        </div>
      </div>

      <!-- Function Filters -->
      <div class="flex items-center gap-2 flex-wrap pt-3 border-t border-border/60">
        <Button
          variant={filterFunction === 'all' ? 'default' : 'outline'}
          size="sm"
          class="text-xs h-8 rounded-xl font-bold"
          onclick={() => (filterFunction = 'all')}
        >
          {$tStore('gear.func_all')}
        </Button>
        <Button
          variant={filterFunction === 'music_number' ? 'default' : 'outline'}
          size="sm"
          class="text-xs h-8 rounded-xl font-bold"
          onclick={() => (filterFunction = 'music_number')}
        >
          {$tStore('gear.func_music_number')}
        </Button>
        <Button
          variant={filterFunction === 'backline_common' ? 'default' : 'outline'}
          size="sm"
          class="text-xs h-8 rounded-xl font-bold"
          onclick={() => (filterFunction = 'backline_common')}
        >
          {$tStore('gear.func_backline_common')}
        </Button>
        <Button
          variant={filterFunction === 'emergency_backup' ? 'default' : 'outline'}
          size="sm"
          class="text-xs h-8 rounded-xl font-bold"
          onclick={() => (filterFunction = 'emergency_backup')}
        >
          {$tStore('gear.func_emergency_backup')}
        </Button>
        <Button
          variant={filterFunction === 'sound_desk' ? 'default' : 'outline'}
          size="sm"
          class="text-xs h-8 rounded-xl font-bold"
          onclick={() => (filterFunction = 'sound_desk')}
        >
          {$tStore('gear.func_sound_desk')}
        </Button>
      </div>
    </Card>

    <!-- VIEW 1: SHOW CUSTODY BENTO GRID CARDS -->
    {#if showViewMode === 'cards'}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each filteredShowAllocations as alloc (alloc.id)}
          <Card class="flex flex-col justify-between rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card p-5 shadow-sm hover:shadow-md transition-all gap-4 {alloc.status === 'orphan' ? 'border-rose-500/60 bg-rose-500/5 ring-1 ring-rose-500/20' : ''}">
            <div class="flex flex-col gap-3">
              <!-- Top Row -->
              <div class="flex items-start justify-between gap-2">
                <div class="flex flex-col">
                  <span class="text-sm font-black text-foreground">{alloc.gearName}</span>
                  <div class="flex items-center gap-2 text-[10px] text-muted-foreground mt-0.5">
                    {#if alloc.ownership === 'club_property'}
                      <span class="text-blue-600 font-bold">CSAC Property</span>
                    {:else}
                      <span class="text-purple-600 font-bold">Owner: {alloc.ownerName}</span>
                    {/if}
                    <span>• {alloc.primaryPerformerName}</span>
                  </div>
                </div>

                <span class="inline-flex items-center rounded-full px-2.5 py-0.5 text-[10px] font-bold border shrink-0 {showStatusBadgeMap[alloc.status].bg} {showStatusBadgeMap[alloc.status].text}">
                  {showStatusBadgeMap[alloc.status].label}
                </span>
              </div>

              <!-- Function Badge & Title -->
              <div class="flex items-center gap-2 flex-wrap">
                <Badge variant="outline" class="text-[10px] py-0.5 px-2 font-bold rounded-lg bg-muted/40">
                  {$tStore(`gear.func_${alloc.allocatedFor}`)}
                </Badge>
                {#if alloc.musicNumberTitle}
                  <span class="text-xs font-semibold text-foreground italic">
                    {alloc.musicNumberTitle}
                  </span>
                {/if}
              </div>

              <!-- Live Status Details Box -->
              <div class="rounded-2xl border border-border/70 bg-muted/25 p-3 flex flex-col gap-1.5 text-xs">
                {#if alloc.status === 'retrieved_owner'}
                  <div class="flex items-center justify-between text-emerald-600 font-bold">
                    <span class="text-muted-foreground font-normal">Retrieved by:</span>
                    <div class="flex items-center gap-1">
                      <UserCheck class="w-3.5 h-3.5" />
                      <span>{alloc.retrievedByName}</span>
                    </div>
                  </div>
                {:else if alloc.status === 'retrieved_proxy'}
                  <div class="flex flex-col gap-1 text-purple-600">
                    <div class="flex items-center justify-between font-bold">
                      <span class="text-muted-foreground font-normal">Proxy Retriever:</span>
                      <div class="flex items-center gap-1 text-purple-600">
                        <AlertTriangle class="w-3.5 h-3.5 text-amber-500" />
                        <span>{alloc.retrievedByName}</span>
                      </div>
                    </div>
                    {#if alloc.retrievalNote}
                      <span class="text-[10px] text-muted-foreground italic border-t border-border/40 pt-1">
                        "{alloc.retrievalNote}"
                      </span>
                    {/if}
                  </div>
                {:else if alloc.status === 'adopted'}
                  <div class="flex items-center justify-between text-teal-600 font-bold">
                    <span class="text-muted-foreground font-normal">Safekeeping Adopter:</span>
                    <div class="flex items-center gap-1">
                      <Shield class="w-3.5 h-3.5" />
                      <span>{alloc.adoption?.adopterName}</span>
                    </div>
                  </div>
                {:else if alloc.status === 'orphan'}
                  <div class="flex items-center justify-center py-1 text-rose-600 font-black tracking-tight animate-pulse text-xs">
                    🚨 LEFT UNCLAIMED AT VENUE
                  </div>
                {:else}
                  <div class="flex items-center justify-between text-muted-foreground">
                    <span>Venue Check-In:</span>
                    <span class="font-semibold text-foreground">{alloc.checkInTimestamp || 'Pending arrival'}</span>
                  </div>
                {/if}
              </div>
            </div>

            <!-- Card Bottom Actions -->
            <div class="flex items-center justify-end gap-1.5 pt-3 border-t border-border/60 flex-wrap">
              {#if alloc.status === 'allocated_main' || alloc.status === 'allocated_backup'}
                <Button
                  variant="outline"
                  size="sm"
                  class="text-xs h-8 px-3 rounded-xl gap-1.5 border-amber-500/40 text-amber-600 hover:bg-amber-500/10 font-bold w-full"
                  onclick={() => handleCheckIn(alloc)}
                >
                  <Check class="w-3.5 h-3.5" />
                  <span>{$tStore('gear.btn_checkin_venue')}</span>
                </Button>
              {/if}

              {#if alloc.status === 'checked_in_venue'}
                <Button
                  variant="outline"
                  size="sm"
                  class="text-xs h-8 px-3 rounded-xl font-bold"
                  onclick={() => handleStageActive(alloc)}
                >
                  <Radio class="w-3.5 h-3.5 mr-1 text-blue-500" />
                  <span>{$tStore('gear.btn_stage_active')}</span>
                </Button>
              {/if}

              {#if alloc.status === 'active_stage' || alloc.status === 'checked_in_venue'}
                <Button
                  variant="outline"
                  size="sm"
                  class="text-xs h-8 px-3 rounded-xl text-emerald-600 border-emerald-500/30 hover:bg-emerald-500/10 font-bold"
                  onclick={() => handleOwnerRetrieve(alloc)}
                >
                  <UserCheck class="w-3.5 h-3.5 mr-1" />
                  <span>{$tStore('gear.btn_retrieve_owner')}</span>
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  class="text-xs h-8 px-2.5 rounded-xl text-purple-600 border-purple-500/30 hover:bg-purple-500/10 font-bold"
                  onclick={() => openProxyRetrieveModal(alloc)}
                >
                  <UserX class="w-3.5 h-3.5 mr-1" />
                  <span>{$tStore('gear.btn_retrieve_proxy')}</span>
                </Button>
                <Button
                  variant="destructive"
                  size="sm"
                  class="text-xs h-8 px-2.5 rounded-xl font-black"
                  onclick={() => handleMarkOrphan(alloc)}
                >
                  <AlertTriangle class="w-3.5 h-3.5 mr-1" />
                  <span>{$tStore('gear.btn_mark_orphan')}</span>
                </Button>
              {/if}

              {#if alloc.status === 'orphan'}
                <Button
                  variant="default"
                  size="sm"
                  class="text-xs h-8 px-4 rounded-xl font-black bg-teal-600 hover:bg-teal-700 text-white w-full shadow-md"
                  onclick={() => openAdoptModal(alloc)}
                >
                  <Shield class="w-3.5 h-3.5 mr-1.5" />
                  <span>{$tStore('gear.btn_adopt_gear')}</span>
                </Button>
              {/if}
            </div>
          </Card>
        {/each}
      </div>
    {:else}
      <!-- VIEW 2: SHOW CUSTODY COMPACT TABLE WITH OVERFLOW PROTECTION -->
      <Card class="rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card shadow-sm overflow-hidden p-0">
        <div class="overflow-x-auto w-full">
          <Table.Root class="min-w-[800px]">
            <Table.TableHeader>
              <Table.TableRow class="bg-muted/40">
                <Table.TableHead class="text-xs font-bold text-foreground">{$tStore('gear.th_gear')}</Table.TableHead>
                <Table.TableHead class="text-xs font-bold text-foreground">{$tStore('gear.th_allocated_for')}</Table.TableHead>
                <Table.TableHead class="text-xs font-bold text-foreground">{$tStore('gear.th_show_stage')}</Table.TableHead>
                <Table.TableHead class="text-xs font-bold text-foreground">{$tStore('gear.th_checkin')}</Table.TableHead>
                <Table.TableHead class="text-xs font-bold text-foreground">{$tStore('gear.th_retrieval')}</Table.TableHead>
                <Table.TableHead class="text-right text-xs font-bold text-foreground">{$tStore('gear.th_actions')}</Table.TableHead>
              </Table.TableRow>
            </Table.TableHeader>
            <Table.TableBody>
              {#each filteredShowAllocations as alloc (alloc.id)}
                <Table.TableRow class={alloc.status === 'orphan' ? 'bg-rose-500/10' : ''}>
                  <Table.TableCell>
                    <div class="flex flex-col gap-0.5">
                      <span class="text-xs font-bold text-foreground whitespace-nowrap">{alloc.gearName}</span>
                      <div class="flex items-center gap-2 text-[10px] text-muted-foreground whitespace-nowrap">
                        {#if alloc.ownership === 'club_property'}
                          <span class="text-blue-600 font-semibold">CSAC Property</span>
                        {:else}
                          <span class="text-purple-600 font-semibold">Owner: {alloc.ownerName}</span>
                        {/if}
                        <span>• {alloc.primaryPerformerName}</span>
                      </div>
                    </div>
                  </Table.TableCell>

                  <Table.TableCell>
                    <div class="flex flex-col gap-0.5">
                      <Badge variant="outline" class="w-fit text-[10px] py-0.5 font-bold rounded-lg whitespace-nowrap">
                        {$tStore(`gear.func_${alloc.allocatedFor}`)}
                      </Badge>
                      {#if alloc.musicNumberTitle}
                        <span class="text-[11px] text-muted-foreground italic truncate max-w-[150px]">
                          {alloc.musicNumberTitle}
                        </span>
                      {/if}
                    </div>
                  </Table.TableCell>

                  <Table.TableCell>
                    <span class="inline-flex items-center rounded-full px-2.5 py-0.5 text-[10px] font-bold border whitespace-nowrap {showStatusBadgeMap[alloc.status].bg} {showStatusBadgeMap[alloc.status].text}">
                      {showStatusBadgeMap[alloc.status].label}
                    </span>
                  </Table.TableCell>

                  <Table.TableCell>
                    {#if alloc.status === 'allocated_main' || alloc.status === 'allocated_backup'}
                      <Button
                        variant="outline"
                        size="sm"
                        class="text-[11px] h-7 px-2.5 rounded-xl gap-1 border-amber-500/40 text-amber-600 hover:bg-amber-500/10 font-bold whitespace-nowrap"
                        onclick={() => handleCheckIn(alloc)}
                      >
                        <Check class="w-3 h-3" />
                        <span>{$tStore('gear.btn_checkin_venue')}</span>
                      </Button>
                    {:else}
                      <div class="flex items-center gap-1.5 text-xs text-emerald-600 font-bold whitespace-nowrap">
                        <CheckCircle2 class="w-3.5 h-3.5" />
                        <span>{alloc.checkInTimestamp || 'Checked in'}</span>
                      </div>
                    {/if}
                  </Table.TableCell>

                  <Table.TableCell>
                    {#if alloc.status === 'retrieved_owner'}
                      <div class="flex items-center gap-1.5 text-xs text-emerald-600 font-bold whitespace-nowrap">
                        <UserCheck class="w-3.5 h-3.5" />
                        <span>{alloc.retrievedByName}</span>
                      </div>
                    {:else if alloc.status === 'retrieved_proxy'}
                      <div class="flex flex-col gap-0.5">
                        <div class="flex items-center gap-1 text-xs text-purple-600 font-bold whitespace-nowrap">
                          <AlertTriangle class="w-3 h-3 text-amber-500" />
                          <span>{alloc.retrievedByName} (Proxy)</span>
                        </div>
                        {#if alloc.retrievalNote}
                          <span class="text-[10px] text-muted-foreground italic truncate max-w-[150px]">{alloc.retrievalNote}</span>
                        {/if}
                      </div>
                    {:else if alloc.status === 'adopted'}
                      <div class="flex flex-col gap-0.5 whitespace-nowrap">
                        <div class="flex items-center gap-1 text-xs text-teal-600 font-bold">
                          <Shield class="w-3 h-3" />
                          <span>Adopted: {alloc.adoption?.adopterName}</span>
                        </div>
                        <span class="text-[10px] text-muted-foreground">Tel: {alloc.adoption?.adopterPhone}</span>
                      </div>
                    {:else if alloc.status === 'orphan'}
                      <Badge variant="destructive" class="text-[10px] font-black animate-pulse rounded-lg whitespace-nowrap">
                        UNCLAIMED ORPHAN
                      </Badge>
                    {:else}
                      <span class="text-[11px] text-muted-foreground">—</span>
                    {/if}
                  </Table.TableCell>

                  <Table.TableCell class="text-right">
                    <div class="flex items-center justify-end gap-1.5 whitespace-nowrap">
                      {#if alloc.status === 'checked_in_venue'}
                        <Button
                          variant="outline"
                          size="sm"
                          class="text-[11px] h-7 px-2.5 rounded-xl font-bold"
                          onclick={() => handleStageActive(alloc)}
                        >
                          <Radio class="w-3 h-3 mr-1 text-blue-500" />
                          <span>{$tStore('gear.btn_stage_active')}</span>
                        </Button>
                      {/if}

                      {#if alloc.status === 'active_stage' || alloc.status === 'checked_in_venue'}
                        <Button
                          variant="outline"
                          size="sm"
                          class="text-[11px] h-7 px-2.5 rounded-xl text-emerald-600 border-emerald-500/30 hover:bg-emerald-500/10 font-bold"
                          onclick={() => handleOwnerRetrieve(alloc)}
                        >
                          <UserCheck class="w-3 h-3 mr-1" />
                          <span>{$tStore('gear.btn_retrieve_owner')}</span>
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          class="text-[11px] h-7 px-2.5 rounded-xl text-purple-600 border-purple-500/30 hover:bg-purple-500/10 font-bold"
                          onclick={() => openProxyRetrieveModal(alloc)}
                        >
                          <UserX class="w-3 h-3 mr-1" />
                          <span>{$tStore('gear.btn_retrieve_proxy')}</span>
                        </Button>
                        <Button
                          variant="destructive"
                          size="sm"
                          class="text-[11px] h-7 px-2.5 rounded-xl font-black shadow-sm"
                          onclick={() => handleMarkOrphan(alloc)}
                        >
                          <AlertTriangle class="w-3 h-3 mr-1" />
                          <span>{$tStore('gear.btn_mark_orphan')}</span>
                        </Button>
                      {/if}

                      {#if alloc.status === 'orphan'}
                        <Button
                          variant="default"
                          size="sm"
                          class="text-[11px] h-7 px-3 rounded-xl font-black bg-teal-600 hover:bg-teal-700 text-white shadow-sm"
                          onclick={() => openAdoptModal(alloc)}
                        >
                          <Shield class="w-3 h-3 mr-1" />
                          <span>{$tStore('gear.btn_adopt_gear')}</span>
                        </Button>
                      {/if}
                    </div>
                  </Table.TableCell>
                </Table.TableRow>
              {/each}
            </Table.TableBody>
          </Table.Root>
        </div>
      </Card>
    {/if}
  {/if}

  <!-- TAB 3: ORPHAN WATCHLIST & ADOPTION -->
  {#if activeTab === 'orphans'}
    <div class="flex flex-col gap-4">
      <Card class="p-6 rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card shadow-sm flex flex-col gap-2">
        <h2 class="text-base sm:text-lg font-black text-foreground flex items-center gap-2 tracking-tight">
          <AlertTriangle class="w-5 h-5 text-rose-500" />
          <span>{$tStore('gear.tab_orphans')}</span>
        </h2>
        <p class="text-xs sm:text-sm text-muted-foreground">
          Track equipment left behind at the venue. Any team member can "adopt" gear to safely transport it home until the registered owner retrieves it.
        </p>
      </Card>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        {#each showAllocations.filter((a) => a.status === 'orphan' || a.status === 'adopted') as item (item.id)}
          <Card class="p-6 rounded-3xl border {item.status === 'orphan' ? 'border-rose-500/50 bg-rose-500/5' : 'border-teal-500/40 bg-teal-500/5'} flex flex-col justify-between gap-4 shadow-sm">
            <div class="flex flex-col gap-3">
              <div class="flex items-start justify-between gap-2">
                <div class="flex flex-col gap-1">
                  <span class="text-base font-black text-foreground">{item.gearName}</span>
                  <span class="text-xs text-muted-foreground">
                    Owner: <strong class="text-foreground">{item.ownerName || 'CSAC Club Property'}</strong>
                  </span>
                </div>
                {#if item.status === 'orphan'}
                  <Badge variant="destructive" class="text-[10px] font-black animate-pulse rounded-lg">
                    🚨 ORPHAN
                  </Badge>
                {:else}
                  <Badge variant="outline" class="bg-teal-500/10 text-teal-700 border-teal-500/30 text-[10px] font-black rounded-lg">
                    🛡️ ADOPTED
                  </Badge>
                {/if}
              </div>

              {#if item.status === 'adopted' && item.adoption}
                <div class="rounded-2xl border border-teal-500/20 bg-background/80 p-4 flex flex-col gap-1.5 text-xs">
                  <div class="flex items-center gap-2 font-bold text-teal-700 dark:text-teal-400">
                    <UserCheck class="w-4 h-4" />
                    <span>Temporary Custodian: {item.adoption.adopterName}</span>
                  </div>
                  <div class="flex items-center gap-2 text-muted-foreground">
                    <Phone class="w-4 h-4" />
                    <span>Phone: {item.adoption.adopterPhone}</span>
                  </div>
                  <div class="flex items-center gap-2 text-muted-foreground">
                    <Calendar class="w-4 h-4" />
                    <span>Expected Return: {item.adoption.targetReturnDate}</span>
                  </div>
                  <div class="mt-1 text-[11px] text-emerald-600 font-bold flex items-center gap-1">
                    <CheckCircle2 class="w-3.5 h-3.5" />
                    <span>Owner notified via SMS / In-App notification</span>
                  </div>
                </div>
              {/if}
            </div>

            {#if item.status === 'orphan'}
              <Button
                variant="default"
                size="default"
                class="h-11 rounded-2xl font-black gap-2 bg-teal-600 hover:bg-teal-700 text-white w-full shadow-md"
                onclick={() => openAdoptModal(item)}
              >
                <Shield class="w-4 h-4" />
                <span>{$tStore('gear.btn_adopt_gear')}</span>
              </Button>
            {/if}
          </Card>
        {/each}
      </div>
    </div>
  {/if}
</div>

<!-- ========================================================================= -->
<!-- PREMIUM REDESIGNED BENTO MODAL: REGISTER NEW GEAR                         -->
<!-- ========================================================================= -->
{#if isRegisterModalOpen}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) isRegisterModalOpen = false; }}>
    <Dialog.Content class="max-w-2xl max-h-[90vh] overflow-hidden p-0 rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card shadow-2xl flex flex-col">
      <!-- Modal Header Banner (Fixed Header) -->
      <div class="relative shrink-0 overflow-hidden bg-gradient-to-br from-primary/15 via-primary/5 to-transparent p-6 sm:p-7 border-b border-border/60">
        <div class="flex items-start justify-between">
          <div class="flex items-center gap-3">
            <div class="flex h-11 w-11 items-center justify-center rounded-2xl bg-primary text-primary-foreground shadow-lg shadow-primary/25">
              <Plus class="h-6 w-6 stroke-[3]" />
            </div>
            <div class="flex flex-col gap-0.5">
              <Badge variant="outline" class="w-fit bg-primary/10 text-primary border-primary/25 text-[10px] font-black uppercase tracking-wider py-0 px-2 rounded-full">
                CSAC Fleet Asset Registry
              </Badge>
              <Dialog.Title class="text-xl sm:text-2xl font-black tracking-tight text-foreground">
                {$tStore('gear.modal_register_title')}
              </Dialog.Title>
            </div>
          </div>
        </div>
        <p class="mt-2 text-xs sm:text-sm text-muted-foreground max-w-lg">
          Add new instruments or sound gear to the central inventory, define ownership sovereignty, custody holder, and custom lending policies.
        </p>
      </div>

      <!-- Modal Body Form (Scrollable body cleanly within rounded boundaries) -->
      <form onsubmit={handleRegisterSubmit} class="flex flex-col gap-6 p-6 sm:p-7 overflow-y-auto max-h-[calc(90vh-140px)]">
        <!-- Section 1: Core Identification -->
        <div class="flex flex-col gap-4">
          <div class="flex items-center gap-2">
            <Tag class="w-4 h-4 text-primary" />
            <h4 class="text-xs font-black uppercase tracking-wider text-muted-foreground">1. Asset Identification & Category</h4>
          </div>

          <div class="flex flex-col gap-1.5">
            <Label for="reg-name" class="text-xs font-bold text-foreground">
              {$tStore('gear.field_gear_name')} <span class="text-primary">*</span>
            </Label>
            <Input
              id="reg-name"
              type="text"
              bind:value={newGearName}
              placeholder="e.g. Gibson Les Paul Standard 60s Iced Tea"
              required
              class="h-11 rounded-2xl text-xs sm:text-sm bg-muted/30 border-border/80 focus:bg-background focus:ring-2 focus:ring-primary/20 transition-all font-medium"
            />
          </div>

          <!-- Category Bento Grid Selector -->
          <div class="flex flex-col gap-2">
            <Label class="text-xs font-bold text-foreground">{$tStore('gear.field_category')}</Label>
            <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
              {#each categoryOptions as opt}
                <button
                  type="button"
                  class="flex items-center gap-2 p-2.5 rounded-2xl border text-left text-xs font-bold transition-all {newGearCategory === opt.id ? 'border-primary bg-primary/10 text-primary ring-2 ring-primary/20 shadow-sm' : 'border-border/70 bg-muted/20 hover:bg-muted/50 text-foreground'}"
                  onclick={() => (newGearCategory = opt.id)}
                >
                  <div class="p-1 rounded-lg {newGearCategory === opt.id ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                    <svelte:component this={opt.icon} class="w-3.5 h-3.5" />
                  </div>
                  <span class="truncate">{$tStore(opt.labelKey)}</span>
                </button>
              {/each}
            </div>
          </div>
        </div>

        <!-- Section 2: Ownership Classification -->
        <div class="flex flex-col gap-4 pt-4 border-t border-border/60">
          <div class="flex items-center gap-2">
            <Shield class="w-4 h-4 text-primary" />
            <h4 class="text-xs font-black uppercase tracking-wider text-muted-foreground">2. Ownership & Accountability</h4>
          </div>

          <!-- Ownership Dual Card Selector -->
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <button
              type="button"
              class="flex flex-col gap-1 p-3.5 rounded-2xl border text-left transition-all {newGearOwnership === 'club_property' ? 'border-blue-500 bg-blue-500/10 ring-2 ring-blue-500/20 shadow-sm' : 'border-border/70 bg-muted/20 hover:bg-muted/40'}"
              onclick={() => (newGearOwnership = 'club_property')}
            >
              <div class="flex items-center justify-between">
                <span class="text-xs font-black text-foreground">CSAC Club Property</span>
                <Badge variant="outline" class="text-[10px] bg-blue-500/10 text-blue-600 border-blue-500/30 font-bold">Public Fleet</Badge>
              </div>
              <p class="text-[11px] text-muted-foreground">Owned by the university club, open for general band rehearsals and show allocations.</p>
            </button>

            <button
              type="button"
              class="flex flex-col gap-1 p-3.5 rounded-2xl border text-left transition-all {newGearOwnership === 'member_owned' ? 'border-purple-500 bg-purple-500/10 ring-2 ring-purple-500/20 shadow-sm' : 'border-border/70 bg-muted/20 hover:bg-muted/40'}"
              onclick={() => (newGearOwnership = 'member_owned')}
            >
              <div class="flex items-center justify-between">
                <span class="text-xs font-black text-foreground">Member-Owned Gear</span>
                <Badge variant="outline" class="text-[10px] bg-purple-500/10 text-purple-600 border-purple-500/30 font-bold">Personal Asset</Badge>
              </div>
              <p class="text-[11px] text-muted-foreground">Personal musician equipment brought to club; owner retains sovereignty and policy controls.</p>
            </button>
          </div>

          {#if newGearOwnership === 'member_owned'}
            <div class="flex flex-col gap-1.5 p-3.5 rounded-2xl border border-purple-500/30 bg-purple-500/5 animate-in fade-in">
              <Label for="reg-owner-name" class="text-xs font-bold text-foreground">
                {$tStore('gear.field_owner_name')} <span class="text-purple-600">*</span>
              </Label>
              <Input
                id="reg-owner-name"
                type="text"
                bind:value={newGearOwnerName}
                placeholder="e.g. Minh Pháp / Hoàng Nam"
                required
                class="h-10 rounded-xl text-xs bg-background border-purple-500/30 focus:border-purple-500"
              />
            </div>
          {/if}
        </div>

        <!-- Section 3: Lending Policy Grid -->
        <div class="flex flex-col gap-4 pt-4 border-t border-border/60">
          <div class="flex items-center gap-2">
            <Lock class="w-4 h-4 text-primary" />
            <h4 class="text-xs font-black uppercase tracking-wider text-muted-foreground">3. Lending Policy & Authorization</h4>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
            {#each policyOptions as pol}
              <button
                type="button"
                class="flex items-start gap-3 p-3 rounded-2xl border text-left transition-all {newGearPolicy === pol.id ? 'border-primary bg-primary/10 ring-2 ring-primary/20 shadow-sm' : 'border-border/70 bg-muted/20 hover:bg-muted/40'}"
                onclick={() => (newGearPolicy = pol.id)}
              >
                <div class="p-1.5 rounded-xl mt-0.5 {newGearPolicy === pol.id ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                  <svelte:component this={pol.icon} class="w-3.5 h-3.5" />
                </div>
                <div class="flex flex-col gap-0.5">
                  <span class="text-xs font-bold text-foreground">{$tStore(pol.labelKey)}</span>
                  <span class="text-[11px] text-muted-foreground leading-tight">{pol.desc}</span>
                </div>
              </button>
            {/each}
          </div>
        </div>

        <!-- Section 4: Physical Custody, Location & Financial Tracking -->
        <div class="flex flex-col gap-4 pt-4 border-t border-border/60">
          <div class="flex items-center gap-2">
            <MapPin class="w-4 h-4 text-primary" />
            <h4 class="text-xs font-black uppercase tracking-wider text-muted-foreground">4. Custody Holder & Location Tracking</h4>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <div class="flex flex-col gap-1.5">
              <Label for="reg-custodian" class="text-xs font-bold text-foreground">
                {$tStore('gear.field_custodian_name')}
              </Label>
              <Input
                id="reg-custodian"
                type="text"
                bind:value={newGearCustodian}
                placeholder="Person physically holding gear"
                class="h-10 rounded-2xl text-xs bg-muted/30 border-border/80 focus:bg-background"
              />
            </div>

            <div class="flex flex-col gap-1.5">
              <Label for="reg-location" class="text-xs font-bold text-foreground">
                {$tStore('gear.field_location_note')}
              </Label>
              <Input
                id="reg-location"
                type="text"
                bind:value={newGearLocation}
                placeholder="e.g. Studio Room A Locker #2"
                class="h-10 rounded-2xl text-xs bg-muted/30 border-border/80 focus:bg-background"
              />
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <div class="flex flex-col gap-1.5">
              <Label for="reg-serial" class="text-xs font-bold text-foreground">
                {$tStore('gear.field_serial_number')}
              </Label>
              <Input
                id="reg-serial"
                type="text"
                bind:value={newGearSerial}
                placeholder="SN-123456"
                class="h-10 rounded-2xl text-xs font-mono bg-muted/30 border-border/80 focus:bg-background"
              />
            </div>

            <div class="flex flex-col gap-1.5">
              <Label for="reg-val" class="text-xs font-bold text-foreground">
                {$tStore('gear.field_estimated_value')}
              </Label>
              <Input
                id="reg-val"
                type="number"
                bind:value={newGearEstimatedValue}
                placeholder="25000000"
                class="h-10 rounded-2xl text-xs bg-muted/30 border-border/80 focus:bg-background"
              />
            </div>
          </div>
        </div>

        <!-- Footer Actions -->
        <div class="flex items-center justify-end gap-3 pt-4 border-t border-border/60">
          <Button
            type="button"
            variant="outline"
            size="default"
            class="h-11 px-5 rounded-2xl font-bold"
            onclick={() => (isRegisterModalOpen = false)}
          >
            {$tStore('gear.btn_cancel')}
          </Button>
          <Button
            type="submit"
            variant="default"
            size="default"
            class="h-11 px-6 rounded-2xl font-black gap-2 shadow-lg shadow-primary/25 bg-primary hover:bg-primary/90 text-primary-foreground"
          >
            <Check class="w-4 h-4 stroke-[3]" />
            <span>{$tStore('gear.btn_submit_register')}</span>
          </Button>
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Root>
{/if}

<!-- MODAL 2: TRANSFER CUSTODY -->
{#if isTransferModalOpen && selectedGearForTransfer}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) isTransferModalOpen = false; }}>
    <Dialog.Content class="max-w-md p-6 rounded-3xl border border-black/[0.08] dark:border-white/[0.08] bg-card shadow-2xl">
      <Dialog.Header>
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-2xl bg-primary/10 text-primary">
            <RefreshCw class="w-5 h-5" />
          </div>
          <div class="flex flex-col gap-0.5">
            <Dialog.Title class="text-lg font-black text-foreground">
              {$tStore('gear.modal_transfer_title')}
            </Dialog.Title>
            <Dialog.Description class="text-xs text-muted-foreground truncate max-w-[280px]">
              {selectedGearForTransfer.name}
            </Dialog.Description>
          </div>
        </div>
      </Dialog.Header>

      <form onsubmit={handleTransferSubmit} class="flex flex-col gap-4 py-3 text-xs">
        <div class="flex flex-col gap-1.5">
          <Label for="custodian-name" class="font-bold text-foreground">{$tStore('gear.field_custodian_name')} *</Label>
          <Input id="custodian-name" type="text" bind:value={newCustodianName} required class="h-10 rounded-2xl text-xs bg-muted/30 border-border/80 focus:bg-background" />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="location-note" class="font-bold text-foreground">{$tStore('gear.field_location_note')} *</Label>
          <Input id="location-note" type="text" bind:value={newLocationNote} required class="h-10 rounded-2xl text-xs bg-muted/30 border-border/80 focus:bg-background" />
        </div>

        <div class="flex items-center justify-end gap-2.5 pt-3 border-t border-border/60">
          <Button type="button" variant="outline" size="sm" class="h-10 px-4 rounded-xl font-bold" onclick={() => (isTransferModalOpen = false)}>
            {$tStore('gear.btn_cancel')}
          </Button>
          <Button type="submit" variant="default" size="sm" class="h-10 px-5 rounded-xl font-black bg-primary text-primary-foreground shadow-md">
            {$tStore('gear.btn_save_transfer')}
          </Button>
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Root>
{/if}

<!-- MODAL 3: RETRIEVE ON BEHALF (PROXY) -->
{#if isProxyModalOpen && selectedAllocationForProxy}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) isProxyModalOpen = false; }}>
    <Dialog.Content class="max-w-md p-6 rounded-3xl border border-amber-500/30 bg-card shadow-2xl">
      <Dialog.Header>
        <div class="flex items-center gap-3 text-amber-600">
          <div class="p-2.5 rounded-2xl bg-amber-500/15">
            <AlertTriangle class="w-5 h-5" />
          </div>
          <div class="flex flex-col gap-0.5">
            <Dialog.Title class="text-lg font-black text-foreground">
              {$tStore('gear.modal_proxy_title')}
            </Dialog.Title>
            <Dialog.Description class="text-xs text-amber-600/90 font-medium">
              {$tStore('gear.modal_proxy_warning', { owner: selectedAllocationForProxy.ownerName || 'Club' })}
            </Dialog.Description>
          </div>
        </div>
      </Dialog.Header>

      <form onsubmit={handleProxyRetrieveSubmit} class="flex flex-col gap-4 py-3 text-xs">
        <div class="flex flex-col gap-1.5">
          <Label for="proxy-name" class="font-bold text-foreground">{$tStore('gear.field_proxy_name')} *</Label>
          <Input id="proxy-name" type="text" bind:value={proxyRetrieverName} placeholder="Full Name of Proxy Collector" required class="h-10 rounded-2xl text-xs bg-muted/30 border-border/80 focus:bg-background" />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="proxy-note" class="font-bold text-foreground">{$tStore('gear.field_proxy_note')}</Label>
          <Input id="proxy-note" type="text" bind:value={proxyRetrievalNote} placeholder="e.g. Taking in car for Minh Pháp" class="h-10 rounded-2xl text-xs bg-muted/30 border-border/80 focus:bg-background" />
        </div>

        <div class="flex items-center justify-end gap-2.5 pt-3 border-t border-border/60">
          <Button type="button" variant="outline" size="sm" class="h-10 px-4 rounded-xl font-bold" onclick={() => (isProxyModalOpen = false)}>
            {$tStore('gear.btn_cancel')}
          </Button>
          <Button type="submit" variant="default" size="sm" class="h-10 px-5 rounded-xl font-black bg-amber-600 hover:bg-amber-700 text-white shadow-md">
            {$tStore('gear.btn_confirm_proxy')}
          </Button>
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Root>
{/if}

<!-- MODAL 4: ADOPT ORPHAN GEAR -->
{#if isAdoptModalOpen && selectedAllocationForAdopt}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) isAdoptModalOpen = false; }}>
    <Dialog.Content class="max-w-md p-6 rounded-3xl border border-teal-500/30 bg-card shadow-2xl">
      <Dialog.Header>
        <div class="flex items-center gap-3 text-teal-600">
          <div class="p-2.5 rounded-2xl bg-teal-500/15">
            <Shield class="w-5 h-5" />
          </div>
          <div class="flex flex-col gap-0.5">
            <Dialog.Title class="text-lg font-black text-foreground">
              {$tStore('gear.modal_adopt_title')}
            </Dialog.Title>
            <Dialog.Description class="text-xs text-muted-foreground truncate max-w-[280px]">
              {$tStore('gear.modal_adopt_desc')} ({selectedAllocationForAdopt.gearName})
            </Dialog.Description>
          </div>
        </div>
      </Dialog.Header>

      <form onsubmit={handleAdoptSubmit} class="flex flex-col gap-4 py-3 text-xs">
        <div class="flex flex-col gap-1.5">
          <Label for="adopt-name" class="font-bold text-foreground">{$tStore('gear.field_adopter_name')} *</Label>
          <Input id="adopt-name" type="text" bind:value={adopterName} placeholder="Your Full Name" required class="h-10 rounded-2xl text-xs bg-muted/30 border-border/80 focus:bg-background" />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="adopt-phone" class="font-bold text-foreground">{$tStore('gear.field_adopter_phone')} *</Label>
          <Input id="adopt-phone" type="tel" bind:value={adopterPhone} placeholder="0901234567" required class="h-10 rounded-2xl text-xs bg-muted/30 border-border/80 focus:bg-background" />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="adopt-date" class="font-bold text-foreground">{$tStore('gear.field_target_return')}</Label>
          <Input id="adopt-date" type="text" bind:value={targetReturnDate} placeholder="e.g. Tomorrow 14:00 at Club Room" class="h-10 rounded-2xl text-xs bg-muted/30 border-border/80 focus:bg-background" />
        </div>

        <div class="flex items-center justify-end gap-2.5 pt-3 border-t border-border/60">
          <Button type="button" variant="outline" size="sm" class="h-10 px-4 rounded-xl font-bold" onclick={() => (isAdoptModalOpen = false)}>
            {$tStore('gear.btn_cancel')}
          </Button>
          <Button type="submit" variant="default" size="sm" class="h-10 px-5 rounded-xl font-black bg-teal-600 hover:bg-teal-700 text-white shadow-md">
            {$tStore('gear.btn_confirm_adopt')}
          </Button>
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Root>
{/if}
