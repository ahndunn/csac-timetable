<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { Users, Clock, CircleCheck } from '@lucide/svelte';
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
  import { canManageSongScoped, canAuditSongScoped } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';
  import type { SongNumber } from './NumberCard.svelte';

  interface Props {
    numbers: SongNumber[];
    showId: string;
    userRole: UserRole;
    currentUserName: string;
    onOpenLineup: (song: SongNumber) => void;
    onOpenQc: (song: SongNumber) => void;
    onAdvanceStage: (song: SongNumber, nextStage: SongNumber['stage']) => void;
  }

  let {
    numbers,
    showId,
    userRole,
    currentUserName,
    onOpenLineup,
    onOpenQc,
    onAdvanceStage,
  }: Props = $props();

  function getStageBadgeVariant(stage: string) {
    switch (stage) {
      case 'draft':
        return 'bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 border-slate-200 dark:border-slate-700';
      case 'in_practice':
        return 'bg-blue-500/10 text-blue-600 border-blue-200 dark:border-blue-800';
      case 'ready_for_qc':
        return 'bg-primary/10 text-primary border-primary/20';
      case 'qc_approved':
        return 'bg-indigo-500/10 text-indigo-600 border-indigo-200 dark:border-indigo-800';
      case 'stage_ready':
        return 'bg-emerald-500/10 text-emerald-600 border-emerald-200 dark:border-emerald-800';
      default:
        return 'bg-muted text-muted-foreground';
    }
  }

  function getStageLabel(stage: string) {
    switch (stage) {
      case 'draft':
        return $tStore('show_mgmt.status.draft');
      case 'in_practice':
        return $tStore('studio_shows.kanban_practice');
      case 'ready_for_qc':
        return $tStore('studio_shows.kanban_ready_qc');
      case 'qc_approved':
        return $tStore('studio_shows.kanban_qc_approved');
      case 'stage_ready':
        return $tStore('studio_shows.kanban_stage_ready');
      default:
        return stage;
    }
  }
</script>

<Card class="p-0 overflow-hidden shadow-xs">
  <div class="overflow-x-auto">
    <Table>
      <TableHeader>
        <TableRow class="bg-muted/50 border-border">
          <TableHead class="w-12 text-center">{$tStore('studio_shows.th_order')}</TableHead>
          <TableHead>{$tStore('studio_shows.th_song')} & {$tStore('studio_shows.th_genre')}</TableHead>
          <TableHead>{$tStore('studio_shows.th_pm')}</TableHead>
          <TableHead>{$tStore('studio_shows.label_qc_reviewer')}</TableHead>
          <TableHead>{$tStore('studio_shows.th_lineup')}</TableHead>
          <TableHead>{$tStore('studio_shows.th_qc')}</TableHead>
          <TableHead class="text-right">{$tStore('studio_shows.th_actions')}</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {#each numbers as song, index (song.id)}
          <TableRow>
            <TableCell class="text-center font-mono text-xs font-bold text-muted-foreground">
              {index + 1}
            </TableCell>
            <TableCell>
              <div class="font-bold text-xs text-foreground">{song.title}</div>
              <div class="text-[11px] text-muted-foreground">{song.genre}</div>
            </TableCell>
            <TableCell class="text-xs font-semibold">{song.pmName}</TableCell>
            <TableCell class="text-xs text-muted-foreground">{song.qcReviewer}</TableCell>
            <TableCell>
              <div class="flex flex-wrap gap-1 max-w-[280px]">
                {#if song.lineup?.vocalLead}
                  <span class="text-[10px] bg-primary/10 text-primary px-1.5 py-0.5 rounded font-medium inline-flex items-center gap-0.5">
                    🎤 {song.lineup.vocalLead}
                  </span>
                {/if}
                {#if song.lineup?.guitarLead}
                  <span class="text-[10px] bg-blue-500/10 text-blue-600 px-1.5 py-0.5 rounded font-medium inline-flex items-center gap-0.5">
                    🎸 {song.lineup.guitarLead}
                  </span>
                {/if}
                {#if song.lineup?.bass}
                  <span class="text-[10px] bg-purple-500/10 text-purple-600 px-1.5 py-0.5 rounded font-medium inline-flex items-center gap-0.5">
                    🎸 {song.lineup.bass}
                  </span>
                {/if}
                {#if song.lineup?.drums}
                  <span class="text-[10px] bg-emerald-500/10 text-emerald-600 px-1.5 py-0.5 rounded font-medium inline-flex items-center gap-0.5">
                    🥁 {song.lineup.drums}
                  </span>
                {/if}
                {#if song.lineup?.keys}
                  <span class="text-[10px] bg-teal-500/10 text-teal-600 px-1.5 py-0.5 rounded font-medium inline-flex items-center gap-0.5">
                    🎹 {song.lineup.keys}
                  </span>
                {/if}
                {#if !song.lineup?.vocalLead && !song.lineup?.guitarLead && !song.lineup?.bass && !song.lineup?.drums && !song.lineup?.keys}
                  <span class="text-[10px] text-muted-foreground italic">—</span>
                {/if}
              </div>
            </TableCell>
            <TableCell>
              <Badge variant="outline" class="{getStageBadgeVariant(song.stage)} text-[11px] font-semibold">
                {getStageLabel(song.stage)}
              </Badge>
            </TableCell>
            <TableCell class="text-right">
              <div class="inline-flex items-center gap-1">
                <a
                  href="/studio/shows/{showId}/sprints?song={encodeURIComponent(song.title)}"
                  class="inline-flex items-center justify-center h-7 px-2 text-xs rounded bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-200 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
                  title="Sprint Schedule"
                >
                  <Clock class="w-3.5 h-3.5" />
                </a>

                {#if canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
                  <Button
                    variant="outline"
                    size="sm"
                    class="h-7 px-2"
                    onclick={() => onOpenLineup(song)}
                    title="Assign Band Lineup"
                  >
                    <Users class="w-3.5 h-3.5" />
                  </Button>

                  {#if song.stage === 'draft'}
                    <Button
                      size="sm"
                      class="h-7 px-2.5 text-xs font-semibold bg-primary hover:bg-primary/90 text-primary-foreground"
                      onclick={() => onAdvanceStage(song, 'in_practice')}
                    >
                      Practice
                    </Button>
                  {:else if song.stage === 'in_practice'}
                    <Button
                      size="sm"
                      class="h-7 px-2.5 text-xs font-semibold bg-primary hover:bg-primary/90 text-primary-foreground"
                      onclick={() => onAdvanceStage(song, 'ready_for_qc')}
                    >
                      QC
                    </Button>
                  {/if}
                {/if}

                {#if song.stage === 'ready_for_qc' && canAuditSongScoped(userRole, false, song.qcReviewer === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
                  <Button
                    size="sm"
                    class="h-7 px-2.5 text-xs bg-primary hover:bg-primary/90 text-primary-foreground font-semibold"
                    onclick={() => onOpenQc(song)}
                  >
                    Audit
                  </Button>
                {/if}

                {#if song.stage === 'qc_approved' && canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
                  <Button
                    size="sm"
                    class="h-7 px-2.5 text-xs bg-emerald-600 hover:bg-emerald-700 text-white"
                    onclick={() => onAdvanceStage(song, 'stage_ready')}
                  >
                    Promote
                  </Button>
                {/if}

                {#if song.stage === 'stage_ready'}
                  <span class="inline-flex items-center gap-1 text-[11px] font-bold text-emerald-600 px-2">
                    <CircleCheck class="w-3.5 h-3.5" />
                  </span>
                {/if}
              </div>
            </TableCell>
          </TableRow>
        {/each}
      </TableBody>
    </Table>
  </div>
</Card>
