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
    idle: { label: 'System Ready', icon: Sparkles, spin: false, style: 'bg-emerald-500/10 text-emerald-600 border-emerald-500/20' },
    syncing: { label: 'Syncing Kafka', icon: Loader2, spin: true, style: 'bg-amber-500/10 text-amber-600 border-amber-500/25' },
    saved: { label: 'Availability Saved', icon: CheckCircle2, spin: false, style: 'bg-emerald-500/15 text-emerald-700 border-emerald-500/30' },
    queued: { label: 'Scheduler Queued', icon: Clock, spin: false, style: 'bg-orange-500/10 text-[#ff6b00] border-[#ff6b00]/25' },
    processing: { label: 'Computing CSP Solver', icon: Cpu, spin: true, style: 'bg-indigo-500/10 text-indigo-600 border-indigo-500/25' },
    completed: { label: 'Schedule Completed', icon: CheckCircle2, spin: false, style: 'bg-emerald-500/15 text-emerald-600 border-emerald-500/30' },
    failed: { label: 'Computation Failed', icon: AlertCircle, spin: false, style: 'bg-rose-500/10 text-rose-600 border-rose-500/25' }
  };

  let current = $derived(statusMap[status] || statusMap.idle);
  let IconComponent = $derived(current.icon);
</script>

<div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full border text-[11px] font-mono font-semibold tracking-wide shadow-sm backdrop-blur-md transition-all duration-300 {current.style}">
  <IconComponent size={13} class={current.spin ? 'animate-spin' : ''} />
  <span>{current.label}</span>
</div>
