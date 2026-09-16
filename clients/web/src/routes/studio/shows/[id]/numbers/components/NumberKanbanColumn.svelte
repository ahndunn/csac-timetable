<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { Users, UserCheck, CircleCheck } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { canManageSongScoped, canAuditSongScoped } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';
  import type { SongNumber } from './NumberCard.svelte';

  interface Props {
    title: string;
    stageKey: SongNumber['stage'];
    count: number;
    colorClass: string;
    bgBadgeClass: string;
    numbers: SongNumber[];
    userRole: UserRole;
    currentUserName: string;
    onOpenLineup: (song: SongNumber) => void;
    onOpenQc: (song: SongNumber) => void;
    onAdvanceStage: (song: SongNumber, nextStage: SongNumber['stage']) => void;
  }

  let {
    title,
    stageKey,
    count,
    colorClass,
    bgBadgeClass,
    numbers,
    userRole,
    currentUserName,
    onOpenLineup,
    onOpenQc,
    onAdvanceStage,
  }: Props = $props();

  const columnNumbers = $derived(numbers.filter((n) => n.stage === stageKey));
  function getBorderLeftClass(stage: SongNumber['stage']) {
    switch (stage) {
      case 'stage_ready':
        return 'border-l-emerald-500';
      case 'qc_approved':
        return 'border-l-indigo-500';
      case 'ready_for_qc':
        return 'border-l-primary';
      case 'in_practice':
        return 'border-l-blue-500';
      default:
        return 'border-l-slate-400';
    }
  }
</script>

<Card class="p-3 bg-slate-50 dark:bg-slate-900/50 flex flex-col gap-3 min-h-[480px]">
  <div class="flex items-center justify-between">
    <span class="text-xs font-bold {colorClass}">{title}</span>
    <span class="text-xs font-bold {bgBadgeClass} px-2 py-0.5 rounded-full">{count}</span>
  </div>

  <div class="flex flex-col gap-2.5">
    {#each columnNumbers as song (song.id)}
      <Card class="p-3 bg-white dark:bg-card border-l-4 shadow-xs {getBorderLeftClass(stageKey)}">
        <h4 class="text-xs font-bold text-foreground leading-tight m-0">{song.title}</h4>
        <div class="text-[11px] text-muted-foreground mt-0.5">{song.genre}</div>

        <div class="mt-2 text-xs text-muted-foreground">
          {#if stageKey === 'ready_for_qc'}
            <div class="text-[11px] text-muted-foreground">QC: {song.qcReviewer}</div>
          {:else}
            <div>Leader: {song.pmName}</div>
          {/if}
        </div>

        {#if stageKey === 'draft' && canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
          <div class="mt-2">
            <Button
              size="sm"
              class="w-full text-[11px] h-7 bg-primary hover:bg-primary/90 text-primary-foreground font-semibold"
              onclick={() => onAdvanceStage(song, 'in_practice')}
            >
              Start Practice
            </Button>
          </div>
        {:else if stageKey === 'in_practice' && canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
          <div class="flex items-center gap-1.5 mt-2">
            <Button
              variant="outline"
              size="sm"
              class="flex-1 text-[11px] h-7 px-2"
              onclick={() => onOpenLineup(song)}
            >
              <Users class="w-3 h-3" />
              <span>Lineup</span>
            </Button>
            <Button
              size="sm"
              class="flex-1 text-[11px] h-7 bg-primary hover:bg-primary/90 text-primary-foreground font-semibold"
              onclick={() => onAdvanceStage(song, 'ready_for_qc')}
            >
              To QC
            </Button>
          </div>
        {:else if stageKey === 'ready_for_qc' && canAuditSongScoped(userRole, false, song.qcReviewer === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
          <div class="mt-2">
            <Button
              size="sm"
              class="w-full text-[11px] h-7 bg-primary hover:bg-primary/90 text-primary-foreground gap-1 font-semibold"
              onclick={() => onOpenQc(song)}
            >
              <UserCheck class="w-3 h-3" />
              <span>Audit & Submit QC</span>
            </Button>
          </div>
        {:else if stageKey === 'qc_approved'}
          {#if song.qcNotes}
            <div class="text-[10px] text-emerald-600 dark:text-emerald-400 bg-emerald-50 dark:bg-emerald-950/40 p-1.5 rounded mt-1.5">
              QC: {song.qcNotes}
            </div>
          {/if}
          {#if canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
            <div class="mt-2">
              <Button
                size="sm"
                class="w-full text-[11px] h-7 bg-emerald-600 hover:bg-emerald-700 text-white"
                onclick={() => onAdvanceStage(song, 'stage_ready')}
              >
                Promote to Ready
              </Button>
            </div>
          {/if}
        {:else if stageKey === 'stage_ready'}
          <div class="flex items-center justify-between mt-2 pt-2 border-t border-slate-100 dark:border-slate-800">
            {#if canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
              <Button
                variant="outline"
                size="sm"
                class="text-[11px] h-7 px-2"
                onclick={() => onOpenLineup(song)}
              >
                Lineup
              </Button>
            {/if}
            <span class="text-[10px] font-bold text-emerald-600 flex items-center gap-1">
              <CircleCheck class="w-3 h-3" /> Ready
            </span>
          </div>
        {/if}
      </Card>
    {/each}
  </div>
</Card>
