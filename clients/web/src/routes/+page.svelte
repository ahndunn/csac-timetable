<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '$lib/api/client';
  import { auth } from '$lib/stores/auth.svelte';
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore } from '$lib/i18n';
  import {
    Calendar,
    FileSpreadsheet,
    Users,
    ShieldAlert,
    ArrowRight,
    Sparkles,
    Vote,
    Activity,
    Lock,
    CheckCircle2,
    Music,
  } from '@lucide/svelte';

  let activeEvents = $state<any[]>([]);
  let isLoadingEvents = $state(true);

  onMount(async () => {
    try {
      const res = await api.events.list();
      activeEvents = res.filter((e: any) => e.status === 'open');
    } catch {
      activeEvents = [];
    } finally {
      isLoadingEvents = false;
    }
  });
</script>

<svelte:head>
  <title>{$tStore('hub.page_title')}</title>
  <meta name="description" content={$tStore('hub.meta_description')} />
</svelte:head>

<Navbar />

<div class="hub-container">
  <!-- Hero Bento Banner -->
  <div class="hero-bento">
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
          <span>{$tStore('studio.heading')}</span>
          <ArrowRight size={18} />
        </a>
        <a href="/utils/timetable" class="secondary-hero-btn">
          <Calendar size={18} />
          <span>{$tStore('hub.open_timetable')}</span>
        </a>
        {#if auth.isAuthenticated}
          <a href="/admin/users" class="secondary-hero-btn">
            <Users size={18} />
            <span>{$tStore('hub.admin_console')}</span>
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

  <!-- Bento Grid Navigation Modules -->
  <div class="bento-grid">
    <!-- Card 1: CSAC Production Studio & Gear Fleet (Featured Primary) -->
    <div class="bento-card card-featured">
      <div class="card-icon-wrap bg-orange-soft">
        <Music size={24} class="text-orange" />
      </div>
      <div class="card-body">
        <h3 class="card-title">{$tStore('studio.nav_title')}</h3>
        <p class="card-desc">
          {$tStore('studio.subheading')}
        </p>
      </div>
      <div class="card-footer">
        <a href="/studio" class="card-link">
          <span>{$tStore('studio.heading')}</span>
          <ArrowRight size={16} />
        </a>
      </div>
    </div>

    <!-- Card 2: Active Voting Events -->
    <div class="bento-card">
      <div class="card-icon-wrap bg-green-soft">
        <Vote size={24} class="text-green" />
      </div>
      <div class="card-body">
        <h3 class="card-title">{$tStore('hub.card_events_title')}</h3>
        <p class="card-desc">
          {#if activeEvents.length > 0}
            {$tStore('hub.card_events_desc_active', { count: activeEvents.length })}
          {:else}
            {$tStore('hub.card_events_desc_empty')}
          {/if}
        </p>
      </div>
      <div class="card-footer">
        <a href="/admin/events" class="card-link">
          <span>{$tStore('hub.card_events_link')}</span>
          <ArrowRight size={16} />
        </a>
      </div>
    </div>

    <!-- Card 3: User Directory & RBAC -->
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

    <!-- Card 4: Quorum Approval Governance -->
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

    <!-- Card 5: Legacy Timetable Solver Utilities -->
    <div class="bento-card">
      <div class="card-icon-wrap bg-orange-soft">
        <Calendar size={24} class="text-orange" />
      </div>
      <div class="card-body">
        <h3 class="card-title">{$tStore('hub.card_timetable_title')}</h3>
        <p class="card-desc">
          {$tStore('hub.card_timetable_desc')}
        </p>
      </div>
      <div class="card-footer">
        <a href="/utils/timetable" class="card-link">
          <span>{$tStore('hub.card_timetable_link')}</span>
          <ArrowRight size={16} />
        </a>
      </div>
    </div>

    <!-- Card 6: Excel Workbook Inspector -->
    <div class="bento-card">
      <div class="card-icon-wrap bg-blue-soft">
        <FileSpreadsheet size={24} class="text-blue" />
      </div>
      <div class="card-body">
        <h3 class="card-title">{$tStore('hub.card_inspector_title')}</h3>
        <p class="card-desc">
          {$tStore('hub.card_inspector_desc')}
        </p>
      </div>
      <div class="card-footer">
        <a href="/utils/inspector" class="card-link">
          <span>{$tStore('hub.card_inspector_link')}</span>
          <ArrowRight size={16} />
        </a>
      </div>
    </div>

    <!-- Card 7: Infrastructure & Observability Status -->
    <div class="bento-card">
      <div class="card-icon-wrap bg-emerald-soft">
        <Activity size={24} class="text-emerald" />
      </div>
      <div class="card-body">
        <h3 class="card-title">{$tStore('hub.card_observability_title')}</h3>
        <p class="card-desc">
          {$tStore('hub.card_observability_desc')}
        </p>
      </div>
      <div class="card-footer">
        <div class="status-indicators">
          <span class="status-pill"><CheckCircle2 size={12} class="text-green" /> {$tStore('hub.status_pg')}</span>
          <span class="status-pill"><CheckCircle2 size={12} class="text-green" /> {$tStore('hub.status_redis')}</span>
          <span class="status-pill"><CheckCircle2 size={12} class="text-green" /> {$tStore('hub.status_kafka')}</span>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .hub-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2.5rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .hero-bento {
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 28px;
    padding: 3rem 2.5rem;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.03);
    position: relative;
    overflow: hidden;
  }

  .hero-bento::after {
    content: '';
    position: absolute;
    top: -40px;
    right: -40px;
    width: 250px;
    height: 250px;
    background: radial-gradient(circle, rgba(255, 107, 0, 0.12) 0%, rgba(255, 255, 255, 0) 70%);
    pointer-events: none;
  }

  .hero-tag {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: rgba(255, 107, 0, 0.08);
    border: 1px solid rgba(255, 107, 0, 0.2);
    padding: 0.35rem 0.85rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 700;
    color: #ff6b00;
    margin-bottom: 1rem;
  }

  .hero-title {
    font-size: 2.25rem;
    font-weight: 800;
    color: #0f172a;
    line-height: 1.2;
    margin: 0 0 1rem 0;
    max-width: 800px;
  }

  .hero-description {
    font-size: 1.05rem;
    color: #64748b;
    line-height: 1.6;
    margin: 0 0 2rem 0;
    max-width: 750px;
  }

  .hero-actions {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .primary-hero-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: #ff6b00;
    color: #ffffff;
    padding: 0.85rem 1.65rem;
    border-radius: 14px;
    font-weight: 700;
    font-size: 0.95rem;
    text-decoration: none;
    transition: all 0.2s;
    box-shadow: 0 4px 14px rgba(255, 107, 0, 0.3);
  }

  .primary-hero-btn:hover {
    background: #e65c00;
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(255, 107, 0, 0.4);
  }

  .secondary-hero-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    color: #334155;
    padding: 0.85rem 1.45rem;
    border-radius: 14px;
    font-weight: 600;
    font-size: 0.95rem;
    text-decoration: none;
    transition: all 0.2s;
  }

  .secondary-hero-btn:hover {
    background: #f1f5f9;
    border-color: #cbd5e1;
  }

  .bento-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(340px, 1fr));
    gap: 1.5rem;
  }

  .bento-card {
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 24px;
    padding: 1.75rem;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.02);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 1.25rem;
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .bento-card:hover {
    transform: translateY(-3px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.05);
  }

  .card-icon-wrap {
    width: 52px;
    height: 52px;
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .bg-orange-soft { background: rgba(255, 107, 0, 0.1); }
  .bg-blue-soft { background: rgba(59, 130, 246, 0.1); }
  .bg-green-soft { background: rgba(34, 197, 94, 0.1); }
  .bg-purple-soft { background: rgba(147, 51, 234, 0.1); }
  .bg-red-soft { background: rgba(239, 68, 68, 0.1); }
  .bg-emerald-soft { background: rgba(16, 185, 129, 0.1); }

  .text-orange { color: #ff6b00; }
  .text-blue { color: #3b82f6; }
  .text-green { color: #16a34a; }
  .text-purple { color: #9333ea; }
  .text-red { color: #ef4444; }
  .text-emerald { color: #10b981; }

  .card-title {
    font-size: 1.2rem;
    font-weight: 700;
    color: #0f172a;
    margin: 0 0 0.4rem 0;
  }

  .card-desc {
    font-size: 0.9rem;
    color: #64748b;
    line-height: 1.5;
    margin: 0;
  }

  .card-footer {
    padding-top: 1rem;
    border-top: 1px solid #f1f5f9;
  }

  .card-link {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    color: #ff6b00;
    font-weight: 700;
    font-size: 0.9rem;
    text-decoration: none;
    transition: gap 0.2s;
  }

  .card-link:hover {
    gap: 0.65rem;
  }

  .status-indicators {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .status-pill {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    padding: 0.25rem 0.55rem;
    border-radius: 8px;
    font-size: 0.75rem;
    font-weight: 600;
    color: #334155;
  }
</style>
