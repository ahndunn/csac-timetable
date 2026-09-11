<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore } from '$lib/i18n';
  import {
    Music,
    Calendar,
    ArrowRight,
    Sparkles,
    CheckCircle2,
    Clock,
    FileSpreadsheet,
    Shield,
    Users,
  } from '@lucide/svelte';

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

<div class="studio-hub-container">
  <!-- Studio Hero Section -->
  <div class="hero-bento bento-card">
    <div class="hero-content">
      <div class="hero-tag">
        <Sparkles size={16} class="text-orange" />
        <span>CSAC Music Production Studio</span>
      </div>
      <h1 class="hero-title">Show-Driven Music Production & Rehearsals</h1>
      <p class="hero-description">
        Manage performance numbers, practice sprints, Quality Check (QC) audits, and independent instrument fleet custody in one unified workspace.
      </p>
    </div>

    <div class="quick-actions">
      <a href="/studio/gear" class="bento-btn bento-btn-secondary">
        <FileSpreadsheet size={16} class="text-blue" />
        <span>{$tStore('nav.gear')}</span>
      </a>
      <a href="/admin/shows" class="bento-btn bento-btn-primary">
        <Calendar size={16} />
        <span>{$tStore('nav.admin_shows')}</span>
      </a>
    </div>
  </div>

  <!-- Active Shows Directory -->
  <div class="section-title">
    <h2>Active Music Shows</h2>
    <p>Select a show workspace to manage song numbers, practice sprints, free time, and band roster.</p>
  </div>

  <div class="shows-grid">
    {#each activeShows as show (show.id)}
      <div class="show-card bento-card">
        <div class="card-header">
          <div class="show-icon"><Music size={20} class="text-orange" /></div>
          <div>
            <h3 class="show-title">{show.title}</h3>
            <span class="show-venue">{show.venue} • {show.startDate}</span>
          </div>
        </div>

        <p class="show-desc">{show.description}</p>

        <div class="show-stats">
          <div class="stat">
            <span class="stat-num">{show.numbersCount}</span>
            <span class="stat-tag">Numbers</span>
          </div>
          <div class="stat">
            <span class="stat-num text-green">{show.qcPassRate}%</span>
            <span class="stat-tag">QC Pass Rate</span>
          </div>
        </div>

        <a href="/studio/shows/{show.id}/overview" class="bento-btn bento-btn-primary open-show-btn">
          <span>Enter Show Studio Workspace</span>
          <ArrowRight size={15} />
        </a>
      </div>
    {/each}
  </div>
</div>

<style>
  .studio-hub-container {
    max-width: 1280px;
    margin: 0 auto;
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .bento-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 16px;
    padding: 24px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  }

  .hero-bento {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: linear-gradient(135deg, #ffffff 0%, #fffbf7 100%);
    border: 1px solid rgba(255, 107, 0, 0.2);
  }

  .hero-tag {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 12px;
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    border-radius: 20px;
    font-size: 12px;
    font-weight: 700;
    margin-bottom: 12px;
  }

  .hero-title {
    font-size: 26px;
    font-weight: 800;
    color: #0f172a;
    margin: 0 0 8px 0;
  }

  .hero-description {
    font-size: 14px;
    color: #64748b;
    margin: 0;
    max-width: 640px;
  }

  .quick-actions {
    display: flex;
    gap: 12px;
  }

  .section-title h2 {
    font-size: 20px;
    font-weight: 800;
    color: #0f172a;
    margin: 0 0 4px 0;
  }

  .section-title p {
    font-size: 13px;
    color: #64748b;
    margin: 0;
  }

  .shows-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
    gap: 20px;
  }

  .show-card {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .show-icon {
    width: 44px;
    height: 44px;
    border-radius: 12px;
    background: rgba(255, 107, 0, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .show-title {
    font-size: 17px;
    font-weight: 700;
    color: #0f172a;
    margin: 0 0 2px 0;
  }

  .show-venue {
    font-size: 12px;
    color: #64748b;
  }

  .show-desc {
    font-size: 13px;
    color: #475569;
    line-height: 1.4;
    margin: 0;
  }

  .show-stats {
    display: flex;
    gap: 16px;
    background: #f8fafc;
    padding: 10px 16px;
    border-radius: 10px;
  }

  .stat {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .stat-num {
    font-size: 16px;
    font-weight: 800;
    color: #0f172a;
  }

  .stat-num.text-green { color: #16a34a; }

  .stat-tag {
    font-size: 11px;
    color: #94a3b8;
    text-transform: uppercase;
    font-weight: 600;
  }

  .open-show-btn {
    width: 100%;
    justify-content: center;
    text-decoration: none;
  }
</style>
