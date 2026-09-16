<script lang="ts">
  import { History as HistoryIcon, Activity, Clock } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogDescription,
  } from '$lib/components/ui/dialog';
  import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
  } from '$lib/components/ui/table';
  import type { ScheduleRunHistoryItem, AvailabilityHistoryItem } from '$lib/types/timetable';

  interface Props {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    computeHistory: ScheduleRunHistoryItem[];
    registrationHistory: AvailabilityHistoryItem[];
  }

  let { open = $bindable(false), onOpenChange, computeHistory, registrationHistory }: Props = $props();

  let activeHistoryTab = $state<'compute' | 'registration'>('compute');
</script>

<Dialog bind:open onOpenChange={onOpenChange}>
  <DialogContent class="max-w-3xl">
    <DialogHeader>
      <DialogTitle class="flex items-center gap-2">
        <HistoryIcon class="w-5 h-5 text-primary" />
        <span>Audit History & Run Logs</span>
      </DialogTitle>
      <DialogDescription>
        Inspect async scheduling compute execution results and member free-time updates.
      </DialogDescription>
    </DialogHeader>

    <div class="flex items-center gap-2 border-b border-border pb-2 mt-2">
      <Button
        variant={activeHistoryTab === 'compute' ? 'default' : 'ghost'}
        size="sm"
        class="gap-1.5 text-xs"
        onclick={() => (activeHistoryTab = 'compute')}
      >
        <Activity class="w-3.5 h-3.5" />
        <span>Schedule Compute History ({computeHistory.length})</span>
      </Button>
      <Button
        variant={activeHistoryTab === 'registration' ? 'default' : 'ghost'}
        size="sm"
        class="gap-1.5 text-xs"
        onclick={() => (activeHistoryTab = 'registration')}
      >
        <Clock class="w-3.5 h-3.5" />
        <span>Free-Time Registration Logs ({registrationHistory.length})</span>
      </Button>
    </div>

    <div class="max-h-[360px] overflow-y-auto mt-2">
      {#if activeHistoryTab === 'compute'}
        <Table>
          <TableHeader>
            <TableRow class="bg-muted/50 border-border">
              <TableHead>Run ID</TableHead>
              <TableHead>Triggered By</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Duration</TableHead>
              <TableHead>Score</TableHead>
              <TableHead>Conflicts</TableHead>
              <TableHead>Timestamp</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each computeHistory as item (item.id)}
              <TableRow>
                <TableCell class="font-mono text-xs font-bold">{item.id}</TableCell>
                <TableCell class="text-xs">{item.triggeredByName}</TableCell>
                <TableCell>
                  <Badge variant="outline" class="text-[10px] uppercase font-bold">{item.status}</Badge>
                </TableCell>
                <TableCell class="text-xs">{item.durationMs}ms</TableCell>
                <TableCell class="text-xs font-bold text-emerald-600">{item.score}%</TableCell>
                <TableCell class="text-xs">{item.conflictCount} conflicts</TableCell>
                <TableCell class="text-xs text-muted-foreground">{item.createdAt}</TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      {:else}
        <Table>
          <TableHeader>
            <TableRow class="bg-muted/50 border-border">
              <TableHead>User</TableHead>
              <TableHead>Actor</TableHead>
              <TableHead>Action</TableHead>
              <TableHead>Day</TableHead>
              <TableHead>Slot</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Timestamp</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each registrationHistory as reg (reg.id || `${reg.userName}_${reg.dayOfWeek}_${reg.slotLabel}_${reg.createdAt}`)}
              <TableRow>
                <TableCell class="text-xs font-bold">{reg.userName}</TableCell>
                <TableCell class="text-xs">{reg.actorName}</TableCell>
                <TableCell><Badge variant="secondary" class="text-[10px]">{reg.action}</Badge></TableCell>
                <TableCell class="text-xs">{reg.dayOfWeek}</TableCell>
                <TableCell class="font-mono text-xs">{reg.slotLabel}</TableCell>
                <TableCell class="text-xs font-semibold {reg.isAvailable ? 'text-emerald-600' : 'text-red-600'}">
                  {reg.isAvailable ? 'Available' : 'Unavailable'}
                </TableCell>
                <TableCell class="text-xs text-muted-foreground">{reg.createdAt}</TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      {/if}
    </div>
  </DialogContent>
</Dialog>
