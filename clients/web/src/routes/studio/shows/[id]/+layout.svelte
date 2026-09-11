<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { page } from '$app/stores';
  import { tStore } from '$lib/i18n';
  import {
    Music,
    LayoutDashboard,
    Calendar,
    Users,
    Activity,
    CheckCircle2,
  } from '@lucide/svelte';

  let { children } = $props();

  const showId = $derived($page.params.id || 'show-2026-annual');
  const activeTab = $derived(
    $page.url.pathname.includes('/numbers')
      ? 'numbers'
      : $page.url.pathname.includes('/sprints')
      ? 'sprints'
      : $page.url.pathname.includes('/roster')
      ? 'roster'
      : 'overview'
  );
</script>

<svelte:head>
  <title>CSAC Show Studio — {showId}</title>
</svelte:head>

<Navbar />

<div class="show-layout-page">
  <!-- Show Banner & Sub-Nav Header -->
  <div class="show-banner bento-card">
    <div class="banner-top">
      <div class="show-tag">
        <Music size={15} class="text-orange" />
        <span>Active Music Show Workspace</span>
      </div>

      <div class="banner-title-row">
        <div>
          <h1 class="show-name">CSAC Annual Concert 2026</h1>
          <p class="show-meta">Venue: CSAC Main Auditorium • Production Dates: Oct 1 - Oct 15, 2026</p>
        </div>

        <div class="readiness-badge">
          <CheckCircle2 size={16} class="text-green" />
          <span>75% Stage Ready</span>
        </div>
      </div>
    </div>

    <!-- Sub-Page Navigation Tabs -->
    <nav class="sub-nav-tabs">
      <a
        href="/studio/shows/{showId}/overview"
        class="sub-tab {activeTab === 'overview' ? 'is-active' : ''}"
      >
        <LayoutDashboard size={15} />
        <span>{$tStore('studio_shows.tab_overview')}</span>
      </a>
      <a
        href="/studio/shows/{showId}/numbers"
        class="sub-tab {activeTab === 'numbers' ? 'is-active' : ''}"
      >
        <Music size={15} />
        <span>{$tStore('studio_shows.tab_numbers')}</span>
      </a>
      <a
        href="/studio/shows/{showId}/sprints"
        class="sub-tab {activeTab === 'sprints' ? 'is-active' : ''}"
      >
        <Calendar size={15} />
        <span>{$tStore('studio_shows.tab_sprints')}</span>
      </a>
      <a
        href="/studio/shows/{showId}/roster"
        class="sub-tab {activeTab === 'roster' ? 'is-active' : ''}"
      >
        <Users size={15} />
        <span>{$tStore('studio_shows.tab_roster')}</span>
      </a>
    </nav>
  </div>

  <!-- Sub-Page Main Content View -->
  <main class="show-subpage-content">
    {@render children()}
  </main>
</div>

<style>
  .show-layout-page {
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

  .show-banner {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-bottom: 0;
  }

  .show-tag {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    border-radius: 20px;
    font-size: 12px;
    font-weight: 700;
  }

  .banner-title-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    margin-top: 6px;
  }

  .show-name {
    font-size: 24px;
    font-weight: 800;
    color: #0f172a;
    margin: 0 0 4px 0;
  }

  .show-meta {
    font-size: 13px;
    color: #64748b;
    margin: 0;
  }

  .readiness-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    background: rgba(22, 163, 74, 0.1);
    color: #16a34a;
    padding: 6px 12px;
    border-radius: 20px;
    font-weight: 700;
    font-size: 13px;
  }

  .sub-nav-tabs {
    display: flex;
    gap: 8px;
    border-top: 1px solid #f1f5f9;
    padding-top: 12px;
  }

  .sub-tab {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 10px 16px;
    border-radius: 10px 10px 0 0;
    font-size: 14px;
    font-weight: 600;
    color: #64748b;
    text-decoration: none;
    transition: all 0.15s ease;
  }

  .sub-tab:hover {
    color: #0f172a;
    background: #f8fafc;
  }

  .sub-tab.is-active {
    color: #ff6b00;
    background: rgba(255, 107, 0, 0.1);
    font-weight: 700;
  }

  .show-subpage-content {
    min-height: 400px;
  }
</style>
