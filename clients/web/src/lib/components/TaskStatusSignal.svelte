<script lang="ts">
  import {
    Sparkles,
    Loader2,
    CheckCircle2,
    Clock,
    Cpu,
    AlertCircle,
  } from '@lucide/svelte';

  let { status = 'idle' }: { status?: 'idle' | 'syncing' | 'saved' | 'queued' | 'processing' | 'completed' | 'failed' } = $props();

  const statusMap = {
    idle: { label: 'System Ready', icon: Sparkles, spin: false, type: 'idle' },
    syncing: { label: 'Syncing Kafka', icon: Loader2, spin: true, type: 'syncing' },
    saved: { label: 'Availability Saved', icon: CheckCircle2, spin: false, type: 'saved' },
    queued: { label: 'Scheduler Queued', icon: Clock, spin: false, type: 'queued' },
    processing: { label: 'Computing CSP Solver', icon: Cpu, spin: true, type: 'processing' },
    completed: { label: 'Schedule Completed', icon: CheckCircle2, spin: false, type: 'completed' },
    failed: { label: 'Computation Failed', icon: AlertCircle, spin: false, type: 'failed' }
  };

  let current = $derived(statusMap[status] || statusMap.idle);
  let IconComponent = $derived(current.icon);
</script>

<div class="status-micro-pill pill-{current.type}">
  <span class="icon-wrap {current.spin ? 'is-spinning' : ''}">
    <IconComponent size={13} />
  </span>
  <span class="status-label">{current.label}</span>
</div>

<style>
  .status-micro-pill {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    border-radius: 20px;
    font-family: 'JetBrains Mono', ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.03em;
    text-transform: uppercase;
    box-shadow: 0 2px 8px -2px rgba(0, 0, 0, 0.06);
    backdrop-filter: blur(8px);
    transition: all 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .icon-wrap {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .is-spinning {
    animation: pillSpin 1s linear infinite;
  }

  @keyframes pillSpin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  /* Status Colors */
  .pill-idle {
    background: rgba(240, 253, 244, 0.95);
    color: #16a34a;
    border: 1px solid #bbf7d0;
  }

  .pill-syncing {
    background: rgba(254, 243, 199, 0.95);
    color: #d97706;
    border: 1px solid #fde68a;
  }

  .pill-saved {
    background: rgba(220, 252, 231, 0.95);
    color: #15803d;
    border: 1px solid #86efac;
  }

  .pill-queued {
    background: rgba(255, 237, 213, 0.95);
    color: #ff6b00;
    border: 1px solid #ffc599;
    box-shadow: 0 0 12px rgba(255, 107, 0, 0.15);
  }

  .pill-processing {
    background: rgba(238, 242, 255, 0.95);
    color: #4f46e5;
    border: 1px solid #c7d2fe;
    box-shadow: 0 0 12px rgba(79, 70, 229, 0.15);
  }

  .pill-completed {
    background: rgba(220, 252, 231, 0.95);
    color: #16a34a;
    border: 1px solid #86efac;
  }

  .pill-failed {
    background: rgba(254, 226, 226, 0.95);
    color: #dc2626;
    border: 1px solid #fca5a5;
  }
</style>
