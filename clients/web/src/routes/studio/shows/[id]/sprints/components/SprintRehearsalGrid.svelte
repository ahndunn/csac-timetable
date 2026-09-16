<script lang="ts">
  import { Clock, MapPin, Users } from '@lucide/svelte';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';

  export interface ScheduledRehearsal {
    id: string;
    songTitle: string;
    sessionIndex: number;
    totalTargetRehearsals: number;
    dayIdx: number;
    dayName: string;
    startTime: string;
    endTime: string;
    durationMinutes: number;
    room: string;
    pmName: string;
    performers: string[];
    status: 'in_practice' | 'ready_for_qc' | 'qc_approved' | 'stage_ready';
    color: string;
  }

  interface Props {
    days: string[];
    sessions: ScheduledRehearsal[];
  }

  let { days, sessions }: Props = $props();
</script>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 xl:grid-cols-7 gap-3">
  {#each days as day, dIdx}
    {@const daySessions = sessions.filter((s) => s.dayIdx === dIdx)}
    <div class="flex flex-col gap-2 bg-muted/40 p-3 rounded-xl border border-border">
      <div class="flex items-center justify-between pb-1 border-b border-border">
        <span class="text-xs font-bold text-foreground">{day}</span>
        <span class="text-[10px] font-semibold text-muted-foreground bg-muted px-1.5 py-0.5 rounded border border-border/50">
          {daySessions.length} sessions
        </span>
      </div>

      <div class="flex flex-col gap-2 min-h-[140px]">
        {#if daySessions.length === 0}
          <div class="text-[11px] text-muted-foreground italic text-center py-6">No rehearsals</div>
        {:else}
          {#each daySessions as session (session.id)}
            <Card class="p-2.5 flex flex-col gap-1.5 bg-white dark:bg-card border-l-4 shadow-xs" style="border-left-color: {session.color}">
              <div class="flex items-start justify-between gap-1">
                <div>
                  <h4 class="text-xs font-bold text-foreground leading-tight m-0">{session.songTitle}</h4>
                  <span class="text-[10px] text-muted-foreground">#{session.sessionIndex} of {session.totalTargetRehearsals}</span>
                </div>
                <Badge
                  variant="outline"
                  class="text-[9px] px-1 py-0 font-bold {session.status === 'stage_ready'
                    ? 'bg-emerald-50 dark:bg-emerald-950/40 text-emerald-600 border-emerald-200 dark:border-emerald-800'
                    : session.status === 'qc_approved'
                    ? 'bg-indigo-50 dark:bg-indigo-950/40 text-indigo-600 border-indigo-200 dark:border-indigo-800'
                    : session.status === 'ready_for_qc'
                    ? 'bg-orange-50 dark:bg-orange-950/40 text-primary border-orange-200 dark:border-orange-800'
                    : 'bg-slate-50 dark:bg-slate-800 text-slate-600 dark:text-slate-300 border-slate-200 dark:border-slate-700'}"
                >
                  {session.status === 'stage_ready' ? 'Ready' : session.status === 'qc_approved' ? 'QC OK' : session.status === 'ready_for_qc' ? 'Ready QC' : 'Practice'}
                </Badge>
              </div>

              <div class="flex flex-col gap-0.5 text-[11px] text-slate-600 dark:text-slate-300">
                <div class="flex items-center gap-1">
                  <Clock class="w-3 h-3 text-muted-foreground shrink-0" />
                  <span class="font-semibold text-foreground">{session.startTime}–{session.endTime}</span>
                  <span class="text-muted-foreground text-[10px]">({session.durationMinutes}m)</span>
                </div>

                <div class="flex items-center gap-1">
                  <MapPin class="w-3 h-3 text-muted-foreground shrink-0" />
                  <span class="truncate">{session.room}</span>
                </div>

                <div class="flex items-center gap-1 text-[10px] text-muted-foreground">
                  <Users class="w-3 h-3 shrink-0" />
                  <span class="truncate">{session.performers.join(', ')}</span>
                </div>
              </div>
            </Card>
          {/each}
        {/if}
      </div>
    </div>
  {/each}
</div>
