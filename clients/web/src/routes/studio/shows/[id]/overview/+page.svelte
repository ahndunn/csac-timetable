<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    Music,
    Clock,
    CheckCircle2,
    Activity,
    AlertTriangle,
    ArrowRight,
    Award,
  } from '@lucide/svelte';
</script>

<div class="overview-subpage">
  <!-- Key Metrics Bento Grid -->
  <div class="metrics-grid">
    <div class="metric-card bento-card">
      <div class="metric-header">
        <span class="metric-label">{$tStore('studio_shows.readiness')}</span>
        <Activity size={18} class="text-orange" />
      </div>
      <div class="metric-value">75%</div>
      <div class="progress-bar">
        <div class="progress-fill" style="width: 75%;"></div>
      </div>
      <span class="metric-sub">9 of 12 numbers ready for stage</span>
    </div>

    <div class="metric-card bento-card">
      <div class="metric-header">
        <span class="metric-label">{$tStore('studio_shows.total_numbers')}</span>
        <Music size={18} class="text-blue" />
      </div>
      <div class="metric-value">12</div>
      <span class="metric-sub">Across 4 performance categories</span>
    </div>

    <div class="metric-card bento-card">
      <div class="metric-header">
        <span class="metric-label">{$tStore('studio_shows.total_hours')}</span>
        <Clock size={18} class="text-purple" />
      </div>
      <div class="metric-value">48 hrs</div>
      <span class="metric-sub">3 practice sprints completed</span>
    </div>

    <div class="metric-card bento-card">
      <div class="metric-header">
        <span class="metric-label">{$tStore('studio_shows.qc_approved')}</span>
        <CheckCircle2 size={18} class="text-green" />
      </div>
      <div class="metric-value">9 / 12</div>
      <span class="metric-sub">Reviewed by designated QC team</span>
    </div>
  </div>

  <!-- Lineup Summary & Milestones -->
  <div class="two-column-grid">
    <div class="bento-card">
      <h3>Show Lineup Highlights</h3>
      <div class="lineup-list">
        <div class="lineup-item">
          <div class="lineup-icon"><Music size={16} class="text-orange" /></div>
          <div class="lineup-info">
            <div class="lineup-title">"Hào Khí Việt Nam" (Grand Symphony)</div>
            <div class="lineup-meta">Lead Vocal: Minh Pháp • Band: Full Orchestra</div>
          </div>
          <span class="badge-stage">Stage Ready</span>
        </div>

        <div class="lineup-item">
          <div class="lineup-icon"><Music size={16} class="text-orange" /></div>
          <div class="lineup-info">
            <div class="lineup-title">"Đi Giữa Trời Rực Rỡ" (Pop Rock)</div>
            <div class="lineup-meta">Lead Guitar: Hoàng Nam • Drums: Thu Hà</div>
          </div>
          <span class="badge-qc">QC Approved</span>
        </div>

        <div class="lineup-item">
          <div class="lineup-icon"><Music size={16} class="text-orange" /></div>
          <div class="lineup-info">
            <div class="lineup-title">"Giọt Sương Trên Mí Mắt" (Acoustic Quartet)</div>
            <div class="lineup-meta">Lead Vocal: Bảo Anh • Guitar: Tùng Dương</div>
          </div>
          <span class="badge-practice">In Practice</span>
        </div>
      </div>
    </div>

    <div class="bento-card">
      <h3>Production Milestones</h3>
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

  .lineup-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-top: 14px;
  }

  .lineup-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px;
    background: #f8fafc;
    border-radius: 10px;
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
