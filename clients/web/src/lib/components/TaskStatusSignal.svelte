<script lang="ts">
  let { status = 'idle' }: { status?: 'idle' | 'syncing' | 'saved' | 'queued' | 'processing' | 'completed' | 'failed' } = $props();

  const statusConfig = {
    idle: { label: 'READY', icon: '🟢', bg: 'bg-emerald-500/10 text-emerald-400 border-emerald-500/30' },
    syncing: { label: 'SYNCING KAFKA', icon: '🔄', bg: 'bg-amber-500/10 text-amber-400 border-amber-500/30 animate-pulse' },
    saved: { label: 'AVAILABILITY SAVED', icon: '✔️', bg: 'bg-emerald-500/20 text-emerald-300 border-emerald-500/40' },
    queued: { label: 'SCHEDULER QUEUED', icon: '⏳', bg: 'bg-cyan-500/10 text-cyan-400 border-cyan-500/30 animate-pulse' },
    processing: { label: 'COMPUTING CSP SOLVER', icon: '⚙️', bg: 'bg-purple-500/20 text-purple-300 border-purple-500/40 animate-pulse' },
    completed: { label: 'SCHEDULE COMPLETED', icon: '🟢', bg: 'bg-emerald-500/20 text-emerald-300 border-emerald-500/40' },
    failed: { label: 'COMPUTATION FAILED', icon: '🔴', bg: 'bg-rose-500/20 text-rose-300 border-rose-500/40' }
  };
</script>

<div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full border text-xs font-mono font-medium shadow-sm transition-all duration-300 {statusConfig[status].bg}">
  <span>{statusConfig[status].icon}</span>
  <span>{statusConfig[status].label}</span>
</div>
