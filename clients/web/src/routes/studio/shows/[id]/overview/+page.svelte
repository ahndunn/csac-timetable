<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { page } from '$app/stores';
  import {
    Music,
    Clock,
    CheckCircle2,
    Activity,
    AlertTriangle,
    ArrowRight,
    Award,
    Users,
    Calendar,
    ChevronRight,
    Shield,
  } from '@lucide/svelte';

  let { data } = $props();

  const showId = $derived($page.params.id || 'show-2026-annual');
  const overview = $derived(
    data?.overview || {
      readiness_percent: 75,
      total_numbers: 12,
      total_hours: 48,
      qc_approved_count: 9,
      highlights: [],
      milestones: [],
    }
  );
</script>

<div class="overview-subpage">
  <!-- Key Metrics Bento Grid with 1-Click Deep Links -->
  <div class="metrics-grid">
    <a href="/studio/shows/{showId}/numbers?stage=stage_ready" class="metric-card bento-card metric-link">
      <div class="metric-header">
        <span class="metric-label">{$tStore('studio_shows.readiness')}</span>
        <Activity size={18} class="text-orange" />
      </div>
      <div class="metric-value">{overview.readiness_percent}%</div>
      <div class="progress-bar">
        <div class="progress-fill" style="width: {overview.readiness_percent}%;"></div>
      </div>
      <div class="metric-footer-row">
        <span class="metric-sub">{overview.qc_approved_count} of {overview.total_numbers} numbers ready for stage</span>
        <ChevronRight size={14} class="link-arrow" />
      </div>
    </a>

    <a href="/studio/shows/{showId}/numbers" class="metric-card bento-card metric-link">
      <div class="metric-header">
        <span class="metric-label">{$tStore('studio_shows.total_numbers')}</span>
        <Music size={18} class="text-blue" />
      </div>
      <div class="metric-value">{overview.total_numbers}</div>
      <div class="metric-footer-row">
        <span class="metric-sub">Across 4 performance categories</span>
        <ChevronRight size={14} class="link-arrow" />
      </div>
    </a>

    <a href="/studio/shows/{showId}/sprints" class="metric-card bento-card metric-link">
      <div class="metric-header">
        <span class="metric-label">{$tStore('studio_shows.total_hours')}</span>
        <Clock size={18} class="text-purple" />
      </div>
      <div class="metric-value">{overview.total_hours} hrs</div>
      <div class="metric-footer-row">
        <span class="metric-sub">3 practice sprints completed</span>
        <ChevronRight size={14} class="link-arrow" />
      </div>
    </a>

    <a href="/studio/shows/{showId}/numbers?stage=qc_approved" class="metric-card bento-card metric-link">
      <div class="metric-header">
        <span class="metric-label">{$tStore('studio_shows.qc_approved')}</span>
        <CheckCircle2 size={18} class="text-green" />
      </div>
      <div class="metric-value">{overview.qc_approved_count} / {overview.total_numbers}</div>
      <div class="metric-footer-row">
        <span class="metric-sub">Reviewed by designated QC team</span>
        <ChevronRight size={14} class="link-arrow" />
      </div>
    </a>
  </div>

  <!-- Quick Action Jump Bar (Cross-Screen Synergy) -->
  <div class="quick-action-bar bento-card">
    <div class="quick-action-title">
      <Shield size={16} class="text-orange" />
      <span>Quick Workflow Jump:</span>
    </div>
    <div class="quick-action-links">
      <a href="/studio/shows/{showId}/numbers?stage=ready_for_qc" class="quick-chip chip-orange">
        <Activity size={13} />
        <span>3 Numbers Ready for QC</span>
        <ArrowRight size={12} />
      </a>
      <a href="/studio/shows/{showId}/sprints" class="quick-chip chip-blue">
        <Calendar size={13} />
        <span>Manage 15m Sprint Timetable</span>
        <ArrowRight size={12} />
      </a>
      <a href="/studio/shows/{showId}/roster" class="quick-chip chip-purple">
        <Users size={13} />
        <span>Inspect Show Leadership & Roster</span>
        <ArrowRight size={12} />
      </a>
    </div>
  </div>

  <!-- Lineup Summary & Milestones -->
  <div class="two-column-grid">
    <div class="bento-card">
      <div class="section-header-row">
        <h3>Show Lineup Highlights</h3>
        <a href="/studio/shows/{showId}/numbers" class="view-all-link">
          <span>View All 12 Numbers</span>
          <ChevronRight size={14} />
        </a>
      </div>
      <div class="lineup-list">
        <a href="/studio/shows/{showId}/numbers?q=H%C3%A0o%20Kh%C3%AD%20Vi%E1%BB%87t%20Nam" class="lineup-item-link">
          <div class="lineup-icon"><Music size={16} class="text-orange" /></div>
          <div class="lineup-info">
            <div class="lineup-title">"Hào Khí Việt Nam" (Grand Symphony)</div>
            <div class="lineup-meta">Leader (PM): Minh Pháp • Band: Full Orchestra</div>
          </div>
          <span class="badge-stage">Stage Ready</span>
        </a>

        <a href="/studio/shows/{showId}/numbers?q=%C4%90i%20Gi%E1%BB%AFa%20Tr%E1%BB%9Di%20R%E1%BB%B1c%20R%E1%BB%A1" class="lineup-item-link">
          <div class="lineup-icon"><Music size={16} class="text-orange" /></div>
          <div class="lineup-info">
            <div class="lineup-title">"Đi Giữa Trời Rực Rỡ" (Pop Rock)</div>
            <div class="lineup-meta">Leader (PM): Hoàng Nam • Drums: Thu Hà</div>
          </div>
          <span class="badge-qc">QC Approved</span>
        </a>

        <a href="/studio/shows/{showId}/numbers?q=Gi%E1%BB%8Dt%20S%C6%B0%C6%A1ng%20Tr%C3%AAn%20M%C3%AD%20M%E1%BA%AFt" class="lineup-item-link">
          <div class="lineup-icon"><Music size={16} class="text-orange" /></div>
          <div class="lineup-info">
            <div class="lineup-title">"Giọt Sương Trên Mí Mắt" (Acoustic Quartet)</div>
            <div class="lineup-meta">Leader (PM): Bảo Anh • Guitar: Tùng Dương</div>
          </div>
          <span class="badge-practice">In Practice</span>
        </a>
      </div>
    </div>

    <div class="bento-card">
      <div class="section-header-row">
        <h3>Production Milestones</h3>
        <a href="/studio/shows/{showId}/sprints" class="view-all-link">
          <span>View Sprints</span>
          <ChevronRight size={14} />
        </a>
      </div>
      <div class="milestone-timeline">
        <div class="milestone-step done">
          <div class="step-dot"><CheckCircle2 size={14} /></div>
          <div class="step-content">
            <div class="step-title">Sprint 1: Song Arrangement & Scratch Demo</div>
            <div class="step-date">Completed Sept 15, 2026</div>
          </div>
        </div>

        <div class="milestone-step done">
          <div class="step-dot"><CheckCircle2 size={14} /></div>
          <div class="step-content">
            <div class="step-title">Sprint 2: Band Rehearsals & Vocal Harmonies</div>
            <div class="step-date">Completed Sept 25, 2026</div>
          </div>
        </div>

        <div class="milestone-step active">
          <div class="step-dot"><Clock size={14} /></div>
          <div class="step-content">
            <div class="step-title">Sprint 3: Quality Check (QC) Stage Audits</div>
            <div class="step-date">In Progress (Ends Oct 02)</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .overview-subpage {
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

  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 16px;
  }

  .metric-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .metric-label {
    font-size: 13px;
    font-weight: 600;
    color: #64748b;
  }

  .metric-value {
    font-size: 26px;
    font-weight: 800;
    color: #0f172a;
  }

  .progress-bar {
    height: 6px;
    background: #f1f5f9;
    border-radius: 3px;
    margin: 8px 0;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: #ff6b00;
    border-radius: 3px;
  }

  .metric-sub {
    font-size: 12px;
    color: #94a3b8;
  }

  .two-column-grid {
    display: grid;
    grid-template-columns: 3fr 2fr;
    gap: 20px;
  }

  .metric-link {
    text-decoration: none;
    color: inherit;
    transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1), box-shadow 0.2s ease, border-color 0.2s ease;
    cursor: pointer;
  }

  .metric-link:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
    border-color: #cbd5e1;
  }

  .metric-footer-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .link-arrow {
    color: #94a3b8;
    transition: transform 0.2s ease, color 0.2s ease;
  }

  .metric-link:hover .link-arrow {
    transform: translateX(3px);
    color: #ff6b00;
  }

  .quick-action-bar {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 14px 20px;
    background: #ffffff;
    flex-wrap: wrap;
  }

  .quick-action-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 700;
    color: #0f172a;
  }

  .quick-action-links {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
  }

  .quick-chip {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: 20px;
    font-size: 12px;
    font-weight: 600;
    text-decoration: none;
    transition: all 0.2s ease;
  }

  .chip-orange {
    background: rgba(255, 107, 0, 0.08);
    color: #ff6b00;
    border: 1px solid rgba(255, 107, 0, 0.2);
  }

  .chip-orange:hover {
    background: rgba(255, 107, 0, 0.16);
    transform: translateY(-1px);
  }

  .chip-blue {
    background: rgba(37, 99, 235, 0.08);
    color: #2563eb;
    border: 1px solid rgba(37, 99, 235, 0.2);
  }

  .chip-blue:hover {
    background: rgba(37, 99, 235, 0.16);
    transform: translateY(-1px);
  }

  .chip-purple {
    background: rgba(147, 51, 234, 0.08);
    color: #9333ea;
    border: 1px solid rgba(147, 51, 234, 0.2);
  }

  .chip-purple:hover {
    background: rgba(147, 51, 234, 0.16);
    transform: translateY(-1px);
  }

  .section-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .section-header-row h3 {
    margin: 0;
  }

  .view-all-link {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    font-weight: 600;
    color: #ff6b00;
    text-decoration: none;
  }

  .view-all-link:hover {
    text-decoration: underline;
  }

  .lineup-item-link {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px;
    background: #f8fafc;
    border-radius: 10px;
    text-decoration: none;
    color: inherit;
    border: 1px solid transparent;
    transition: all 0.2s ease;
  }

  .lineup-item-link:hover {
    background: #f1f5f9;
    border-color: #e2e8f0;
    transform: translateX(2px);
  }

  .lineup-icon {
    width: 36px;
    height: 36px;
    background: rgba(255, 107, 0, 0.1);
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .lineup-info { flex: 1; }

  .lineup-title {
    font-size: 14px;
    font-weight: 700;
    color: #0f172a;
  }

  .lineup-meta {
    font-size: 12px;
    color: #64748b;
  }

  .badge-stage {
    background: rgba(22, 163, 74, 0.1);
    color: #16a34a;
    padding: 4px 10px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 700;
  }

  .badge-qc {
    background: rgba(59, 130, 246, 0.1);
    color: #2563eb;
    padding: 4px 10px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 700;
  }

  .badge-practice {
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    padding: 4px 10px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 700;
  }

  .milestone-timeline {
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin-top: 14px;
  }

  .milestone-step {
    display: flex;
    align-items: flex-start;
    gap: 12px;
  }

  .step-dot {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: #e2e8f0;
    color: #64748b;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .milestone-step.done .step-dot {
    background: rgba(22, 163, 74, 0.1);
    color: #16a34a;
  }

  .milestone-step.active .step-dot {
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
  }

  .step-title {
    font-size: 13px;
    font-weight: 700;
    color: #0f172a;
  }

  .step-date {
    font-size: 11px;
    color: #94a3b8;
  }

  @media (max-width: 840px) {
    .two-column-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
