<script lang="ts">
  import {
    Sparkles,
    LoaderCircle,
    CircleCheck,
    Clock,
    Cpu,
    CircleAlert,
  } from '@lucide/svelte';
  import { cn } from '$lib/utils';

  let { status = 'idle' }: { status?: 'idle' | 'syncing' | 'saved' | 'queued' | 'processing' | 'completed' | 'failed' } = $props();

  const statusMap = {
    idle: {
      label: 'System Ready',
      icon: Sparkles,
      spin: false,
      classes: 'bg-green-50/95 text-green-700 border-green-200'
    },
    syncing: {
      label: 'Syncing Kafka',
      icon: LoaderCircle,
      spin: true,
      classes: 'bg-amber-50/95 text-amber-700 border-amber-200'
    },
    saved: {
      label: 'Availability Saved',
      icon: CircleCheck,
      spin: false,
      classes: 'bg-emerald-50/95 text-emerald-800 border-emerald-300'
    },
    queued: {
      label: 'Scheduler Queued',
      icon: Clock,
      spin: false,
      classes: 'bg-primary/10 text-primary border-primary/30 shadow-sm'
    },
    processing: {
      label: 'Computing CSP Solver',
      icon: Cpu,
      spin: true,
      classes: 'bg-indigo-50/95 text-indigo-700 border-indigo-200 shadow-sm shadow-indigo-500/10'
    },
    completed: {
      label: 'Schedule Completed',
      icon: CircleCheck,
      spin: false,
      classes: 'bg-green-50/95 text-green-700 border-green-300'
    },
    failed: {
      label: 'Computation Failed',
      icon: CircleAlert,
      spin: false,
      classes: 'bg-red-50/95 text-red-600 border-red-200'
    }
  };

  let current = $derived(statusMap[status] || statusMap.idle);
  let IconComponent = $derived(current.icon);
</script>

<div class={cn("inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full font-mono text-[11px] font-bold tracking-wider uppercase border shadow-sm backdrop-blur-md transition-all duration-200", current.classes)}>
  <span class={cn("inline-flex items-center justify-center", current.spin && "animate-spin")}>
    <IconComponent size={13} />
  </span>
  <span>{current.label}</span>
</div>
