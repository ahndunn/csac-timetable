<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { PenLine, Trash2 } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
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
  import { canEditPerformerProfile, canManageShowRoster } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';
  import type { ShowRosterMember, WorkloadInfo } from './RosterCard.svelte';

  interface Props {
    roster: ShowRosterMember[];
    userRole: UserRole;
    getWorkloadInfo: (member: ShowRosterMember) => WorkloadInfo;
    onEdit: (member: ShowRosterMember) => void;
    onRemove: (member: ShowRosterMember) => void;
  }

  let { roster, userRole, getWorkloadInfo, onEdit, onRemove }: Props = $props();
</script>

<Card class="p-0 overflow-hidden shadow-xs">
  <div class="overflow-x-auto">
    <Table>
      <TableHeader>
        <TableRow class="bg-slate-50 dark:bg-slate-900/60">
          <TableHead>{$tStore('show_mgmt.roster_page.col_member')}</TableHead>
          <TableHead>{$tStore('show_mgmt.roster_page.col_primary_inst')}</TableHead>
          <TableHead>{$tStore('show_mgmt.roster_page.col_secondary_inst')}</TableHead>
          <TableHead>{$tStore('show_mgmt.roster_page.col_assigned_songs')}</TableHead>
          <TableHead>{$tStore('show_mgmt.roster_page.col_practice_hours')}</TableHead>
          <TableHead>{$tStore('show_mgmt.roster_page.col_attendance')}</TableHead>
          <TableHead>{$tStore('show_mgmt.roster_page.col_workload')}</TableHead>
          {#if canEditPerformerProfile(userRole)}
            <TableHead class="text-right">{$tStore('show_mgmt.roster_page.col_actions')}</TableHead>
          {/if}
        </TableRow>
      </TableHeader>
      <TableBody>
        {#each roster as member (member.id)}
          {@const workload = getWorkloadInfo(member)}
          <TableRow>
            <TableCell>
              <div class="font-bold text-xs text-foreground">{member.fullName}</div>
              <div class="text-[11px] text-muted-foreground">{member.email}</div>
            </TableCell>
            <TableCell>
              <Badge variant="outline" class="bg-primary/10 text-primary border-primary/20 text-xs">
                {$tStore(`show_mgmt.roles.${member.primaryInstrument}`)}
              </Badge>
            </TableCell>
            <TableCell>
              <div class="flex flex-wrap gap-1 max-w-[160px]">
                {#each member.secondaryInstruments as sInst}
                  <span class="text-[10px] bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 px-1 py-0.5 rounded">
                    {$tStore(`show_mgmt.roles.${sInst}`)}
                  </span>
                {/each}
                {#if member.secondaryInstruments.length === 0}
                  <span class="text-muted-foreground text-xs">—</span>
                {/if}
              </div>
            </TableCell>
            <TableCell>
              <div class="flex flex-wrap gap-1 max-w-[180px]">
                {#each member.assignedSongs as song}
                  <span class="text-[10px] bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 px-1.5 py-0.5 rounded">
                    {song}
                  </span>
                {/each}
                {#if member.assignedSongs.length === 0}
                  <span class="text-muted-foreground text-xs italic">Unassigned</span>
                {/if}
              </div>
            </TableCell>
            <TableCell class="text-xs font-semibold">{member.practiceHours}h</TableCell>
            <TableCell class="text-xs font-bold text-emerald-600">{member.attendanceRate}%</TableCell>
            <TableCell>
              <Badge variant="outline" class="text-[10px] font-bold {workload.variant}">
                {$tStore(workload.labelKey)}
              </Badge>
            </TableCell>
            {#if canEditPerformerProfile(userRole)}
              <TableCell class="text-right">
                <div class="inline-flex items-center gap-1">
                  <Button variant="ghost" size="sm" class="h-7 w-7 p-0" onclick={() => onEdit(member)}>
                    <PenLine class="w-3.5 h-3.5" />
                  </Button>
                  {#if canManageShowRoster(userRole)}
                    <Button variant="ghost" size="sm" class="h-7 w-7 p-0 text-red-600 hover:text-red-700 hover:bg-red-50 dark:hover:bg-red-950/40" onclick={() => onRemove(member)}>
                      <Trash2 class="w-3.5 h-3.5" />
                    </Button>
                  {/if}
                </div>
              </TableCell>
            {/if}
          </TableRow>
        {/each}
      </TableBody>
    </Table>
  </div>
</Card>
