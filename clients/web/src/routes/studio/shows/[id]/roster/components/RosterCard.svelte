<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    Music,
    Clock,
    CircleCheck,
    PenLine,
    Trash2,
    Shield,
  } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';
  import { canEditPerformerProfile, canManageShowRoster } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';

  export interface ShowRosterMember {
    id: string;
    fullName: string;
    email: string;
    phone?: string;
    showRole: 'DM' | 'PM' | 'QC' | 'Performer';
    assignedSongs: string[];
    primaryInstrument: string;
    secondaryInstruments: string[];
    practiceHours: number;
    attendanceRate: number;
    workloadFlag?: 'optimal' | 'moderate' | 'fatigued';
  }

  export interface WorkloadInfo {
    variant: string;
    labelKey: string;
    icon: any;
    descKey: string;
  }

  interface Props {
    member: ShowRosterMember;
    showId: string;
    userRole: UserRole;
    workload: WorkloadInfo;
    onEdit: (member: ShowRosterMember) => void;
    onRemove: (member: ShowRosterMember) => void;
  }

  let { member, showId, userRole, workload, onEdit, onRemove }: Props = $props();

  const WorkloadIcon = $derived(workload.icon);
</script>

<Card class="p-4 flex flex-col justify-between gap-3 transition-all duration-200 hover:-translate-y-0.5 hover:border-primary/40 hover:shadow-md">
  <div>
    <div class="flex items-start justify-between gap-2">
      <div>
        <h3 class="text-sm font-bold text-foreground m-0">{member.fullName}</h3>
        <p class="text-xs text-muted-foreground m-0 truncate">{member.email}</p>
      </div>
      <Badge variant="outline" class="font-bold text-[10px] {member.showRole === 'DM' ? 'bg-purple-500/10 text-purple-600 border-purple-200 dark:border-purple-800' : member.showRole === 'PM' ? 'bg-blue-500/10 text-blue-600 border-blue-200 dark:border-blue-800' : member.showRole === 'QC' ? 'bg-orange-500/10 text-primary border-orange-200 dark:border-orange-800' : 'bg-slate-500/10 text-slate-600 dark:text-slate-300 border-slate-200 dark:border-slate-700'}">
        {member.showRole}
      </Badge>
    </div>

    <!-- Instruments Tag Strip -->
    <div class="flex flex-wrap gap-1.5 mt-3">
      <Badge variant="outline" class="bg-primary/10 text-primary border-primary/20 gap-1 text-[11px] font-bold">
        <Music class="w-3 h-3" />
        <span>{$tStore(`show_mgmt.roles.${member.primaryInstrument}`)}</span>
      </Badge>
      {#each member.secondaryInstruments as secInst}
        <Badge variant="secondary" class="text-[11px]">
          + {$tStore(`show_mgmt.roles.${secInst}`)}
        </Badge>
      {/each}
    </div>

    <!-- Assigned Songs Deep Links -->
    <div class="mt-3 flex flex-col gap-1">
      <div class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">Assigned Rehearsal Songs:</div>
      <div class="flex flex-wrap gap-1">
        {#each member.assignedSongs as song}
          <a
            href="/studio/shows/{showId}/numbers?q={encodeURIComponent(song)}"
            class="text-[11px] bg-slate-100 dark:bg-slate-800 hover:bg-primary/10 text-slate-700 dark:text-slate-300 hover:text-primary px-2 py-0.5 rounded transition-colors"
          >
            {song}
          </a>
        {:else}
          <span class="text-[11px] text-muted-foreground italic">No assigned numbers</span>
        {/each}
      </div>
    </div>
  </div>

  <div class="flex flex-col gap-2 pt-2 border-t border-border mt-1">
    <!-- Workload & Attendance Indicators -->
    <div class="flex items-center justify-between text-xs">
      <div class="flex items-center gap-2">
        <span class="text-muted-foreground font-semibold flex items-center gap-1">
          <Clock class="w-3 h-3" /> {member.practiceHours}h
        </span>
        <span class="text-emerald-600 font-semibold flex items-center gap-1">
          <CircleCheck class="w-3 h-3" /> {member.attendanceRate}%
        </span>
      </div>
      <Badge variant="outline" class="font-bold text-[10px] gap-1 {workload.variant}">
        <WorkloadIcon class="w-3 h-3" />
        <span>{$tStore(workload.labelKey)}</span>
      </Badge>
    </div>

    <!-- Member Action Controls -->
    {#if canEditPerformerProfile(userRole)}
      <div class="flex items-center gap-1.5 pt-1">
        <Button
          variant="outline"
          size="sm"
          class="flex-1 text-xs h-7 gap-1"
          onclick={() => onEdit(member)}
        >
          <PenLine class="w-3 h-3" />
          <span>Edit Profile</span>
        </Button>
        {#if canManageShowRoster(userRole)}
          <Button
            variant="ghost"
            size="sm"
            class="text-xs h-7 gap-1 text-red-600 hover:text-red-700 hover:bg-red-50 dark:hover:bg-red-950/40"
            onclick={() => onRemove(member)}
          >
            <Trash2 class="w-3 h-3" />
            <span>Remove</span>
          </Button>
        {/if}
      </div>
    {/if}
  </div>
</Card>
