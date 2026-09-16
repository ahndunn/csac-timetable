<script lang="ts">
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';
  import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
  } from '$lib/components/ui/table';
  import type { ScheduledRehearsal } from './SprintRehearsalGrid.svelte';

  interface Props {
    sessions: ScheduledRehearsal[];
  }

  let { sessions }: Props = $props();
</script>

<Card class="p-0 overflow-hidden shadow-xs">
  <Table>
    <TableHeader>
      <TableRow class="bg-slate-50 dark:bg-slate-900/60">
        <TableHead>Day & Time</TableHead>
        <TableHead>Song & Session</TableHead>
        <TableHead>PM Leader</TableHead>
        <TableHead>Studio Room</TableHead>
        <TableHead>Lineup Performers</TableHead>
        <TableHead>Status</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      {#if sessions.length === 0}
        <TableRow>
          <TableCell colspan={6} class="text-center py-6 text-xs text-muted-foreground">
            No rehearsals match the selected filters.
          </TableCell>
        </TableRow>
      {:else}
        {#each sessions as session (session.id)}
          <TableRow>
            <TableCell>
              <div class="font-bold text-xs text-foreground">{session.dayName}</div>
              <div class="text-[11px] text-muted-foreground">{session.startTime} – {session.endTime} ({session.durationMinutes}m)</div>
            </TableCell>
            <TableCell>
              <div class="font-bold text-xs text-foreground">{session.songTitle}</div>
              <div class="text-[10px] text-muted-foreground">Session #{session.sessionIndex} of {session.totalTargetRehearsals}</div>
            </TableCell>
            <TableCell class="text-xs font-semibold">{session.pmName}</TableCell>
            <TableCell>
              <Badge variant="secondary" class="text-xs">{session.room}</Badge>
            </TableCell>
            <TableCell class="text-xs text-slate-600 dark:text-slate-300">
              <div class="flex flex-wrap gap-1 max-w-[280px]">
                {#each session.performers as perf}
                  <span class="text-[10px] bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 px-1.5 py-0.5 rounded font-medium">
                    {perf}
                  </span>
                {/each}
              </div>
            </TableCell>
            <TableCell>
              <Badge
                variant="outline"
                class="text-xs font-bold {session.status === 'stage_ready'
                  ? 'bg-emerald-50 dark:bg-emerald-950/40 text-emerald-600 border-emerald-200 dark:border-emerald-800'
                  : session.status === 'qc_approved'
                  ? 'bg-indigo-50 dark:bg-indigo-950/40 text-indigo-600 border-indigo-200 dark:border-indigo-800'
                  : session.status === 'ready_for_qc'
                  ? 'bg-orange-50 dark:bg-orange-950/40 text-primary border-orange-200 dark:border-orange-800'
                  : 'bg-slate-50 dark:bg-slate-800 text-slate-600 dark:text-slate-300 border-slate-200 dark:border-slate-700'}"
              >
                {session.status === 'stage_ready' ? 'Stage Ready' : session.status === 'qc_approved' ? 'QC Approved' : session.status === 'ready_for_qc' ? 'Ready QC' : 'In Practice'}
              </Badge>
            </TableCell>
          </TableRow>
        {/each}
      {/if}
    </TableBody>
  </Table>
</Card>
