<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore } from '$lib/i18n';
  import { auth } from '$lib/stores/auth.svelte';
  import {
    Music,
    Calendar,
    Users,
    ShieldAlert,
    ArrowRight,
    Sparkles,
    Lock,
    FileSpreadsheet,
    Activity,
  } from '@lucide/svelte';
</script>

<svelte:head>
  <title>{$tStore('hub.page_title')}</title>
  <meta name="description" content={$tStore('hub.meta_description')} />
</svelte:head>

<Navbar />

<div class="hub-container">
  <!-- Hero Bento Banner -->
  <div class="hero-bento bento-card">
    <div class="hero-content">
      <div class="hero-tag">
        <Sparkles size={16} class="text-orange" />
        <span>{$tStore('hub.tag')}</span>
      </div>
      <h1 class="hero-title">{$tStore('hub.hero_title')}</h1>
      <p class="hero-description">
        {$tStore('hub.hero_description')}
      </p>

      <div class="hero-actions">
        <a href="/studio" class="primary-hero-btn">
          <Music size={18} />
          <span>Launch Music Studio</span>
          <ArrowRight size={18} />
        </a>
        <a href="/studio/gear" class="secondary-hero-btn">
          <FileSpreadsheet size={18} />
          <span>Instrument Fleet</span>
        </a>
        {#if auth.isAuthenticated}
          <a href="/admin/shows" class="secondary-hero-btn">
            <Calendar size={18} />
            <span>Admin Shows</span>
          </a>
        {:else}
          <a href="/auth/login" class="secondary-hero-btn">
            <Lock size={18} />
            <span>{$tStore('hub.sign_in_admin')}</span>
          </a>
        {/if}
      </div>
    </div>
  </div>

  <!-- Bento Grid Cards -->
  <div class="bento-grid">
    <!-- Card 1: Music Studio Workspace -->
    <div class="bento-card card-featured">
      <div class="card-icon-wrap bg-orange-soft">
        <Music size={24} class="text-orange" />
      </div>
      <div class="card-body">
        <h3 class="card-title">Music Production Studio</h3>
        <p class="card-desc">
          Show-driven music numbers, Kanban practice stages, QC verdict audits, and 1-click weekly sprint free-time registration.
        </p>
      </div>
      <div class="card-footer">
        <a href="/studio" class="card-link">
          <span>Open Studio</span>
          <ArrowRight size={16} />
        </a>
      </div>
    </div>

    <!-- Card 2: Instrument Fleet & Custody -->
    <div class="bento-card">
      <div class="card-icon-wrap bg-blue-soft">
        <FileSpreadsheet size={24} class="text-blue" />
      </div>
      <div class="card-body">
        <h3 class="card-title">Instrument Fleet & Custody</h3>
        <p class="card-desc">
          Centralized registry tracking club property and member-owned instruments, physical custody holders, and zero-conflict reservations.
        </p>
      </div>
      <div class="card-footer">
        <a href="/studio/gear" class="card-link">
          <span>Manage Fleet</span>
          <ArrowRight size={16} />
        </a>
      </div>
    </div>

    <!-- Card 3: Admin Show Studio -->
    <div class="bento-card">
      <div class="card-icon-wrap bg-green-soft">
        <Calendar size={24} class="text-green" />
      </div>
      <div class="card-body">
        <h3 class="card-title">Admin Show Studio</h3>
        <p class="card-desc">
          Design music shows, define production date windows, monitor rehearsal metrics, and track QC approval rates in real-time.
        </p>
      </div>
      <div class="card-footer">
        <a href="/admin/shows" class="card-link">
          <span>Admin Shows</span>
          <ArrowRight size={16} />
        </a>
      </div>
    </div>

    <!-- Card 4: Member Governance -->
    <div class="bento-card">
      <div class="card-icon-wrap bg-purple-soft">
        <Users size={24} class="text-purple" />
      </div>
      <div class="card-body">
        <h3 class="card-title">{$tStore('hub.card_users_title')}</h3>
        <p class="card-desc">
          {$tStore('hub.card_users_desc')}
        </p>
      </div>
      <div class="card-footer">
        <a href="/admin/users" class="card-link">
          <span>{$tStore('hub.card_users_link')}</span>
          <ArrowRight size={16} />
        </a>
      </div>
    </div>

    <!-- Card 5: Quorum Demotion Approval -->
    <div class="bento-card">
      <div class="card-icon-wrap bg-red-soft">
        <ShieldAlert size={24} class="text-red" />
      </div>
      <div class="card-body">
        <h3 class="card-title">{$tStore('hub.card_quorum_title')}</h3>
        <p class="card-desc">
          {$tStore('hub.card_quorum_desc')}
        </p>
      </div>
      <div class="card-footer">
        <a href="/admin/approve" class="card-link">
          <span>{$tStore('hub.card_quorum_link')}</span>
          <ArrowRight size={16} />
        </a>
      </div>
    </div>
  </div>
</div>

<style>
  .hub-container {
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
    display: flex;
    flex-direction: column;
  }

  .hero-bento {
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
    font-size: 28px;
    font-weight: 800;
    color: #0f172a;
    margin: 0 0 8px 0;
  }

  .hero-description {
    font-size: 14px;
    color: #64748b;
    margin: 0 0 20px 0;
    max-width: 720px;
    line-height: 1.5;
  }

  .hero-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
  }

  .primary-hero-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    background: #ff6b00;
    color: #ffffff;
    padding: 10px 20px;
    border-radius: 10px;
    font-weight: 700;
    font-size: 14px;
    text-decoration: none;
    box-shadow: 0 2px 4px rgba(255, 107, 0, 0.2);
  }

  .secondary-hero-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    color: #334155;
    padding: 10px 18px;
    border-radius: 10px;
    font-weight: 600;
    font-size: 14px;
    text-decoration: none;
  }

  .bento-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 20px;
  }

  .card-icon-wrap {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 16px;
  }

  .bg-orange-soft { background: rgba(255, 107, 0, 0.1); }
  .bg-blue-soft { background: rgba(59, 130, 246, 0.1); }
  .bg-green-soft { background: rgba(22, 163, 74, 0.1); }
  .bg-purple-soft { background: rgba(147, 51, 234, 0.1); }
  .bg-red-soft { background: rgba(239, 68, 68, 0.1); }

  .text-orange { color: #ff6b00; }
  .text-blue { color: #3b82f6; }
  .text-green { color: #16a34a; }
  .text-purple { color: #9333ea; }
  .text-red { color: #ef4444; }

  .card-body {
    flex: 1;
  }

  .card-title {
    font-size: 17px;
    font-weight: 700;
    color: #0f172a;
    margin: 0 0 6px 0;
  }

  .card-desc {
    font-size: 13px;
    color: #64748b;
    line-height: 1.4;
    margin: 0 0 16px 0;
  }

  .card-footer {
    border-top: 1px solid #f1f5f9;
    padding-top: 12px;
  }

  .card-link {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-weight: 700;
    font-size: 13px;
    color: #ff6b00;
    text-decoration: none;
  }
</style>
