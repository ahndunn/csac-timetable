<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore } from '$lib/i18n';
  import {
    Calendar,
    Plus,
    Music,
    Users,
    Activity,
    CheckCircle2,
    Clock,
    AlertTriangle,
    ArrowRight,
    Search,
    MapPin,
  } from '@lucide/svelte';

  interface Show {
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

  let shows = $state<Show[]>([
    {
      id: 'show-2026-annual',
      title: 'CSAC Annual Concert 2026',
      description: 'Main annual cultural concert featuring 12 band numbers and orchestral arrangements.',
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
      description: 'Intimate acoustic unplugged session with vocal harmonies & classical guitars.',
      venue: 'Studio Lounge B',
      startDate: '2026-11-05',
      endDate: '2026-11-12',
      targetNumbers: 6,
      activeSprints: 1,
      qcPassRate: 40,
      rehearsalHours: 18,
    },
  ]);

  let isCreateModalOpen = $state(false);
  let search = $state('');

  // Form State
  let newTitle = $state('');
  let newDesc = $state('');
  let newVenue = $state('');
  let newStartDate = $state('2026-10-20');
  let newEndDate = $state('2026-10-30');
  let newTarget = $state(8);

  const filteredShows = $derived(
    shows.filter(
      (s) =>
        s.title.toLowerCase().includes(search.toLowerCase()) ||
        s.venue.toLowerCase().includes(search.toLowerCase())
    )
  );

  function handleCreateShow(e: Event) {
    e.preventDefault();
    if (!newTitle.trim()) return;

    const newShow: Show = {
      id: `show-${Date.now()}`,
      title: newTitle,
      description: newDesc,
      venue: newVenue || 'CSAC Studio',
      startDate: newStartDate,
      endDate: newEndDate,
      targetNumbers: newTarget,
      activeSprints: 1,
      qcPassRate: 0,
      rehearsalHours: 0,
    };

    shows = [newShow, ...shows];
    isCreateModalOpen = false;

    // Reset Form
    newTitle = '';
    newDesc = '';
    newVenue = '';
  }
</script>

<svelte:head>
  <title>{$tStore('admin_shows.page_title')}</title>
</svelte:head>

<Navbar />

<div class="admin-shows-page">
  <!-- Header Banner -->
  <div class="page-header bento-card">
    <div class="header-content">
      <div class="header-badge">
        <Calendar size={16} class="text-orange" />
        <span>{$tStore('admin_shows.navbar_title')}</span>
      </div>
      <h1>{$tStore('admin_shows.heading')}</h1>
      <p>{$tStore('admin_shows.subheading')}</p>
    </div>

    <button
      type="button"
      class="bento-btn bento-btn-primary"
      onclick={() => (isCreateModalOpen = true)}
    >
      <Plus size={16} />
      <span>{$tStore('admin_shows.btn_create_show')}</span>
    </button>
  </div>

  <!-- Monitor Stats Overview -->
  <div class="stats-grid">
    <div class="stat-card bento-card">
      <div class="stat-icon orange"><Music size={20} /></div>
      <div class="stat-info">
        <span class="stat-value">{shows.reduce((acc, s) => acc + s.targetNumbers, 0)}</span>
        <span class="stat-label">Total Music Numbers</span>
      </div>
    </div>

    <div class="stat-card bento-card">
      <div class="stat-icon blue"><Clock size={20} /></div>
      <div class="stat-info">
        <span class="stat-value">{shows.reduce((acc, s) => acc + s.rehearsalHours, 0)} hrs</span>
        <span class="stat-label">Scheduled Practice</span>
      </div>
    </div>

    <div class="stat-card bento-card">
      <div class="stat-icon green"><CheckCircle2 size={20} /></div>
      <div class="stat-info">
        <span class="stat-value">68%</span>
        <span class="stat-label">Avg QC Pass Rate</span>
      </div>
    </div>

    <div class="stat-card bento-card">
      <div class="stat-icon purple"><Activity size={20} /></div>
      <div class="stat-info">
        <span class="stat-value">{shows.reduce((acc, s) => acc + s.activeSprints, 0)}</span>
        <span class="stat-label">Active Sprints</span>
      </div>
    </div>
  </div>

  <!-- Search & Filter Controls -->
  <div class="controls-bar bento-card">
    <div class="search-input-wrapper">
      <Search size={16} class="search-icon" />
      <input
        type="text"
        bind:value={search}
        placeholder="Search shows by title or venue..."
        class="search-input"
      />
    </div>
  </div>

  <!-- Shows List -->
  <div class="shows-grid">
    {#each filteredShows as show (show.id)}
      <div class="show-card bento-card">
        <div class="show-header">
          <div>
            <h3 class="show-title">{show.title}</h3>
            <div class="show-venue">
              <MapPin size={13} />
              <span>{show.venue} • {show.startDate} to {show.endDate}</span>
            </div>
          </div>
          <span class="badge-sprint">Sprint {show.activeSprints}</span>
        </div>

        <p class="show-desc">{show.description}</p>

        <!-- Progress Metrics -->
        <div class="show-metrics">
          <div class="metric">
            <span class="metric-num">{show.targetNumbers}</span>
            <span class="metric-tag">Numbers</span>
          </div>
          <div class="metric">
            <span class="metric-num">{show.rehearsalHours}h</span>
            <span class="metric-tag">Practice</span>
          </div>
          <div class="metric">
            <span class="metric-num text-green">{show.qcPassRate}%</span>
            <span class="metric-tag">QC Approved</span>
          </div>
        </div>

        <div class="show-footer">
          <a href="/studio/shows/{show.id}/overview" class="bento-btn bento-btn-secondary open-btn">
            <span>{$tStore('admin_shows.card_btn_open')}</span>
            <ArrowRight size={14} />
          </a>
        </div>
      </div>
    {/each}
  </div>
</div>

<!-- Modal: Create Show -->
{#if isCreateModalOpen}
  <div class="modal-backdrop" onclick={() => (isCreateModalOpen = false)} role="presentation">
    <div class="modal-card bento-card" onclick={(e) => e.stopPropagation()} role="dialog">
      <h2>{$tStore('admin_shows.modal_create_title')}</h2>

      <form onsubmit={handleCreateShow} class="modal-form">
        <div class="form-group">
          <label for="show-title">{$tStore('admin_shows.modal_title_label')}</label>
          <input
            id="show-title"
            type="text"
            bind:value={newTitle}
            placeholder={$tStore('admin_shows.modal_title_placeholder')}
            required
            class="form-input"
          />
        </div>

        <div class="form-group">
          <label for="show-desc">{$tStore('admin_shows.modal_desc_label')}</label>
          <textarea
            id="show-desc"
            bind:value={newDesc}
            placeholder={$tStore('admin_shows.modal_desc_placeholder')}
            rows="3"
            class="form-input"
          ></textarea>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="show-venue">{$tStore('admin_shows.modal_venue_label')}</label>
            <input
              id="show-venue"
              type="text"
              bind:value={newVenue}
              placeholder={$tStore('admin_shows.modal_venue_placeholder')}
              class="form-input"
            />
          </div>

          <div class="form-group">
            <label for="show-target">{$tStore('admin_shows.modal_target_numbers')}</label>
            <input
              id="show-target"
              type="number"
              bind:value={newTarget}
              min="1"
              max="30"
              class="form-input"
            />
          </div>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="show-start">{$tStore('admin_shows.modal_start_date')}</label>
            <input id="show-start" type="date" bind:value={newStartDate} class="form-input" />
          </div>

          <div class="form-group">
            <label for="show-end">{$tStore('admin_shows.modal_end_date')}</label>
            <input id="show-end" type="date" bind:value={newEndDate} class="form-input" />
          </div>
        </div>

        <div class="modal-actions">
          <button
            type="button"
            class="bento-btn"
            onclick={() => (isCreateModalOpen = false)}
          >
            {$tStore('admin_shows.modal_btn_cancel')}
          </button>
          <button type="submit" class="bento-btn bento-btn-primary">
            {$tStore('admin_shows.modal_btn_submit')}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .admin-shows-page {
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

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 16px;
  }

  .stat-card {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 16px 20px;
  }

  .stat-icon {
    width: 44px;
    height: 44px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .stat-icon.orange { background: rgba(255, 107, 0, 0.1); color: #ff6b00; }
  .stat-icon.blue { background: rgba(59, 130, 246, 0.1); color: #3b82f6; }
  .stat-icon.green { background: rgba(22, 163, 74, 0.1); color: #16a34a; }
  .stat-icon.purple { background: rgba(147, 51, 234, 0.1); color: #9333ea; }

  .stat-value {
    display: block;
    font-size: 22px;
    font-weight: 800;
    color: #0f172a;
  }

  .stat-label {
    font-size: 12px;
    color: #64748b;
    font-weight: 600;
  }

  .controls-bar {
    padding: 12px 16px;
  }

  .search-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
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
    font-size: 14px;
    outline: none;
  }

  .search-input:focus {
    border-color: #ff6b00;
  }

  .shows-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
    gap: 20px;
  }

  .show-card {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .show-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .show-title {
    font-size: 17px;
    font-weight: 700;
    color: #0f172a;
    margin: 0 0 4px 0;
  }

  .show-venue {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: #64748b;
  }

  .badge-sprint {
    background: #f1f5f9;
    color: #475569;
    padding: 3px 8px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 700;
  }

  .show-desc {
    font-size: 13px;
    color: #475569;
    line-height: 1.4;
    margin: 0;
  }

  .show-metrics {
    display: flex;
    gap: 12px;
    background: #f8fafc;
    padding: 10px 14px;
    border-radius: 10px;
  }

  .metric {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .metric-num {
    font-weight: 700;
    font-size: 15px;
    color: #0f172a;
  }

  .metric-num.text-green { color: #16a34a; }

  .metric-tag {
    font-size: 10px;
    color: #94a3b8;
    text-transform: uppercase;
    font-weight: 600;
  }

  .open-btn {
    width: 100%;
    justify-content: center;
    text-decoration: none;
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
    max-width: 520px;
  }

  .modal-card h2 {
    font-size: 18px;
    font-weight: 800;
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

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 10px;
  }
</style>
