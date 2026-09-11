<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore } from '$lib/i18n';
  import {
    Music,
    Search,
    Shield,
    UserCheck,
    MapPin,
    AlertCircle,
    Plus,
    RefreshCw,
    Check,
  } from '@lucide/svelte';

  interface Instrument {
    id: string;
    name: string;
    category: string;
    ownership: 'club_property' | 'member_owned';
    ownerName?: string;
    custodianName: string;
    custodianId: string;
    locationNote: string;
    status: 'free_to_borrow' | 'in_use' | 'unavailable' | 'in_maintenance';
  }

  let gearList = $state<Instrument[]>([
    {
      id: 'gear-1',
      name: 'Fender Player Stratocaster (Sunburst)',
      category: 'Electric Guitar',
      ownership: 'club_property',
      custodianName: 'Minh Pháp',
      custodianId: 'user-1',
      locationNote: 'CSAC Studio Locker A',
      status: 'free_to_borrow',
    },
    {
      id: 'gear-2',
      name: 'Yamaha Stage Custom Drum Kit',
      category: 'Drums',
      ownership: 'club_property',
      custodianName: 'Hoàng Nam',
      custodianId: 'user-2',
      locationNote: 'Main Studio Room 1',
      status: 'in_use',
    },
    {
      id: 'gear-3',
      name: 'Ibanez SR500 Bass Guitar',
      category: 'Bass Guitar',
      ownership: 'member_owned',
      ownerName: 'Bảo Anh',
      custodianName: 'Bảo Anh',
      custodianId: 'user-3',
      locationNote: 'Member Home',
      status: 'free_to_borrow',
    },
    {
      id: 'gear-4',
      name: 'Roland Juno-DS88 Keyboard',
      category: 'Keyboard',
      ownership: 'club_property',
      custodianName: 'Thu Hà',
      custodianId: 'user-4',
      locationNote: 'CSAC Studio Locker B',
      status: 'in_maintenance',
    },
  ]);

  let search = $state('');
  let filterOwnership = $state<'all' | 'club_property' | 'member_owned'>('all');

  // Modal State
  let isTransferModalOpen = $state(false);
  let selectedGear = $state<Instrument | null>(null);
  let newCustodianName = $state('');
  let newLocationNote = $state('');

  const filteredGear = $derived(
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
</script>

<svelte:head>
  <title>{$tStore('gear.page_title')}</title>
</svelte:head>

<Navbar />

<div class="gear-page">
  <!-- Header Banner -->
  <div class="page-header bento-card">
    <div class="header-content">
      <div class="header-badge">
        <Shield size={16} class="text-orange" />
        <span>Independent Fleet Governance</span>
      </div>
      <h1>{$tStore('gear.heading')}</h1>
      <p>{$tStore('gear.subheading')}</p>
    </div>

    <button type="button" class="bento-btn bento-btn-primary">
      <Plus size={16} />
      <span>{$tStore('gear.btn_register')}</span>
    </button>
  </div>

  <!-- Filter & Search Toolbar -->
  <div class="toolbar bento-card">
    <div class="filter-tabs">
      <button
        type="button"
        class="filter-tab {filterOwnership === 'all' ? 'is-active' : ''}"
        onclick={() => (filterOwnership = 'all')}
      >
        {$tStore('gear.filter_all')}
      </button>
      <button
        type="button"
        class="filter-tab {filterOwnership === 'club_property' ? 'is-active' : ''}"
        onclick={() => (filterOwnership = 'club_property')}
      >
        {$tStore('gear.filter_club')}
      </button>
      <button
        type="button"
        class="filter-tab {filterOwnership === 'member_owned' ? 'is-active' : ''}"
        onclick={() => (filterOwnership = 'member_owned')}
      >
        {$tStore('gear.filter_member')}
      </button>
    </div>

    <div class="search-input-wrapper">
      <Search size={16} class="search-icon" />
      <input
        type="text"
        bind:value={search}
        placeholder={$tStore('gear.search_placeholder')}
        class="search-input"
      />
    </div>
  </div>

  <!-- Gear Table / Cards -->
  <div class="table-container bento-card">
    <table class="gear-table">
      <thead>
        <tr>
          <th>{$tStore('gear.th_gear')}</th>
          <th>{$tStore('gear.th_category')}</th>
          <th>{$tStore('gear.th_ownership')}</th>
          <th>{$tStore('gear.th_custody')}</th>
          <th>{$tStore('gear.th_status')}</th>
          <th style="text-align: right;">{$tStore('gear.th_actions')}</th>
        </tr>
      </thead>
      <tbody>
        {#each filteredGear as item (item.id)}
          <tr>
            <td>
              <div class="gear-name-cell">
                <Music size={16} class="text-orange" />
                <span class="gear-name">{item.name}</span>
              </div>
            </td>
            <td><span class="badge-cat">{item.category}</span></td>
            <td>
              {#if item.ownership === 'club_property'}
                <span class="badge-club">CSAC Club</span>
              {:else}
                <span class="badge-member">Owner: {item.ownerName}</span>
              {/if}
            </td>
            <td>
              <div class="custody-cell">
                <div class="custodian">
                  <UserCheck size={14} class="text-green" />
                  <span>{item.custodianName}</span>
                </div>
                <div class="location">
                  <MapPin size={12} />
                  <span>{item.locationNote}</span>
                </div>
              </div>
            </td>
            <td>
              {#if item.status === 'free_to_borrow'}
                <span class="status-pill status-free">{$tStore('studio.status_free')}</span>
              {:else if item.status === 'in_use'}
                <span class="status-pill status-in-use">{$tStore('studio.status_in_use')}</span>
              {:else if item.status === 'in_maintenance'}
                <span class="status-pill status-maint">{$tStore('studio.status_maintenance')}</span>
              {:else}
                <span class="status-pill status-unavail">{$tStore('studio.status_unavailable')}</span>
              {/if}
            </td>
            <td style="text-align: right;">
              <button
                type="button"
                class="bento-btn bento-btn-sm"
                onclick={() => openTransferModal(item)}
              >
                <RefreshCw size={13} />
                <span>{$tStore('gear.btn_transfer')}</span>
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<!-- Modal: Transfer Custody -->
{#if isTransferModalOpen && selectedGear}
  <div class="modal-backdrop" onclick={() => (isTransferModalOpen = false)} role="presentation">
    <div class="modal-card bento-card" onclick={(e) => e.stopPropagation()} role="dialog">
      <h2>{$tStore('gear.modal_transfer_title')}</h2>
      <p class="modal-subtitle">Item: <strong>{selectedGear.name}</strong></p>

      <form onsubmit={handleTransferSubmit} class="modal-form">
        <div class="form-group">
          <label for="custodian">{$tStore('gear.modal_custodian')}</label>
          <input
            id="custodian"
            type="text"
            bind:value={newCustodianName}
            required
            class="form-input"
          />
        </div>

        <div class="form-group">
          <label for="location">{$tStore('gear.modal_location')}</label>
          <input
            id="location"
            type="text"
            bind:value={newLocationNote}
            placeholder={$tStore('gear.modal_location_placeholder')}
            required
            class="form-input"
          />
        </div>

        <div class="modal-actions">
          <button
            type="button"
            class="bento-btn"
            onclick={() => (isTransferModalOpen = false)}
          >
            Cancel
          </button>
          <button type="submit" class="bento-btn bento-btn-primary">
            Update Custody
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .gear-page {
    max-width: 1280px;
    margin: 0 auto;
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .bento-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 16px;
    padding: 20px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .header-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    border-radius: 20px;
    font-size: 12px;
    font-weight: 700;
    margin-bottom: 8px;
  }

  .page-header h1 {
    font-size: 24px;
    font-weight: 800;
    color: #0f172a;
    margin: 0 0 4px 0;
  }

  .page-header p {
    color: #64748b;
    font-size: 14px;
    margin: 0;
  }

  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
  }

  .filter-tabs {
    display: flex;
    gap: 6px;
    background: #f1f5f9;
    padding: 4px;
    border-radius: 10px;
  }

  .filter-tab {
    padding: 6px 14px;
    border: none;
    background: transparent;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 600;
    color: #64748b;
    cursor: pointer;
  }

  .filter-tab.is-active {
    background: #ffffff;
    color: #0f172a;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .search-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    width: 300px;
  }

  .search-icon {
    position: absolute;
    left: 12px;
    color: #94a3b8;
  }

  .search-input {
    width: 100%;
    padding: 8px 12px 8px 36px;
    border: 1px solid #cbd5e1;
    border-radius: 10px;
    font-size: 13px;
    outline: none;
  }

  .table-container {
    padding: 0;
    overflow-x: auto;
  }

  .gear-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 14px;
  }

  .gear-table th {
    background: #f8fafc;
    text-align: left;
    padding: 14px 18px;
    font-size: 12px;
    font-weight: 700;
    color: #475569;
    border-bottom: 1px solid #e2e8f0;
  }

  .gear-table td {
    padding: 14px 18px;
    border-bottom: 1px solid #f1f5f9;
    color: #1e293b;
  }

  .gear-name-cell {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .gear-name {
    font-weight: 700;
    color: #0f172a;
  }

  .badge-cat {
    background: #f1f5f9;
    padding: 3px 8px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    color: #475569;
  }

  .badge-club {
    background: rgba(59, 130, 246, 0.1);
    color: #2563eb;
    padding: 3px 8px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 700;
  }

  .badge-member {
    background: rgba(147, 51, 234, 0.1);
    color: #9333ea;
    padding: 3px 8px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 700;
  }

  .custody-cell {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .custodian {
    display: flex;
    align-items: center;
    gap: 4px;
    font-weight: 600;
    font-size: 13px;
  }

  .location {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 11px;
    color: #64748b;
  }

  .status-pill {
    padding: 4px 10px;
    border-radius: 12px;
    font-size: 12px;
    font-weight: 700;
  }

  .status-free { background: rgba(22, 163, 74, 0.1); color: #16a34a; }
  .status-in-use { background: rgba(255, 107, 0, 0.1); color: #ff6b00; }
  .status-maint { background: rgba(239, 68, 68, 0.1); color: #ef4444; }
  .status-unavail { background: #f1f5f9; color: #64748b; }

  .bento-btn-sm {
    padding: 4px 10px;
    font-size: 12px;
  }

  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 23, 42, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 16px;
  }

  .modal-card {
    width: 100%;
    max-width: 440px;
  }

  .modal-card h2 {
    font-size: 18px;
    font-weight: 800;
    margin: 0 0 4px 0;
  }

  .modal-subtitle {
    font-size: 13px;
    color: #64748b;
    margin: 0 0 16px 0;
  }

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-group label {
    font-size: 13px;
    font-weight: 600;
    color: #334155;
  }

  .form-input {
    padding: 8px 12px;
    border: 1px solid #cbd5e1;
    border-radius: 8px;
    font-size: 14px;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 10px;
  }
</style>
