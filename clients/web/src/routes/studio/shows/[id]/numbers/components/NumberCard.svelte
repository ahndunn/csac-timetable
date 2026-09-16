<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    MicVocal,
    Guitar,
    Disc3,
    MessageSquare,
    Clock,
    Users,
    UserCheck,
    CircleCheck,
  } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';
  import { canManageSongScoped, canAuditSongScoped } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';

  export interface SongNumber {
    id: string;
    title: string;
    genre: string;
    pmName: string;
    stage: 'draft' | 'in_practice' | 'ready_for_qc' | 'qc_approved' | 'stage_ready';
    qcReviewer: string;
    qcNotes?: string;
    lineup?: {
      vocalLead?: string;
      guitarLead?: string;
      bass?: string;
      drums?: string;
      keys?: string;
    };
  }

  interface Props {
    song: SongNumber;
    index: number;
    showId: string;
    userRole: UserRole;
    currentUserName: string;
    onOpenLineup: (song: SongNumber) => void;
    onOpenQc: (song: SongNumber) => void;
    onAdvanceStage: (song: SongNumber, nextStage: SongNumber['stage']) => void;
  }

  let {
    song,
    index,
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

<Card class="p-4 flex flex-col justify-between gap-3 transition-all duration-200 hover:-translate-y-0.5 hover:border-primary/40 hover:shadow-md {song.stage === 'stage_ready' ? 'border-l-4 border-l-emerald-500' : ''}">
  <div>
    <div class="flex items-center justify-between mb-2">
      <span class="font-mono text-[11px] font-bold text-muted-foreground bg-muted px-2 py-0.5 rounded">#{index + 1}</span>
      <Badge variant="outline" class="font-bold text-[11px] {getStageBadgeVariant(song.stage)}">
        {getStageLabel(song.stage)}
      </Badge>
    </div>

    <h3 class="text-base font-bold text-foreground leading-snug m-0">{song.title}</h3>
    <div class="text-xs text-muted-foreground font-medium mt-0.5">{song.genre}</div>
    <div class="flex flex-wrap gap-2 mt-2 text-xs text-muted-foreground">
      <span class="bg-muted/50 px-1.5 py-0.5 rounded border border-border"><strong>PM:</strong> {song.pmName}</span>
      <span class="bg-muted/50 px-1.5 py-0.5 rounded border border-border"><strong>QC:</strong> {song.qcReviewer}</span>
    </div>

    <!-- Lineup Section -->
    <div class="mt-3 flex flex-col gap-1.5">
      <div class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">Band Allocation:</div>
      <div class="flex flex-wrap gap-1.5">
        {#if song.lineup?.vocalLead}
          <span class="inline-flex items-center gap-1 text-[11px] font-semibold px-2 py-0.5 rounded bg-primary/10 text-primary">
            <MicVocal class="w-2.5 h-2.5" /> {song.lineup.vocalLead}
          </span>
        {/if}
        {#if song.lineup?.guitarLead}
          <span class="inline-flex items-center gap-1 text-[11px] font-semibold px-2 py-0.5 rounded bg-blue-500/10 text-blue-600">
            <Guitar class="w-2.5 h-2.5" /> {song.lineup.guitarLead}
          </span>
        {/if}
        {#if song.lineup?.bass}
          <span class="inline-flex items-center gap-1 text-[11px] font-semibold px-2 py-0.5 rounded bg-purple-500/10 text-purple-600">
            <Disc3 class="w-2.5 h-2.5" /> {song.lineup.bass}
          </span>
        {/if}
        {#if song.lineup?.drums}
          <span class="inline-flex items-center gap-1 text-[11px] font-semibold px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-600">
            🥁 {song.lineup.drums}
          </span>
        {/if}
        {#if song.lineup?.keys}
          <span class="inline-flex items-center gap-1 text-[11px] font-semibold px-2 py-0.5 rounded bg-teal-500/10 text-teal-600">
            🎹 {song.lineup.keys}
          </span>
        {/if}
        {#if !song.lineup?.vocalLead && !song.lineup?.guitarLead && !song.lineup?.bass && !song.lineup?.drums && !song.lineup?.keys}
          <span class="text-[11px] text-muted-foreground italic bg-muted/40 px-2 py-0.5 rounded">Lineup Unassigned</span>
        {/if}
      </div>
    </div>

    <!-- QC Notes Box -->
    {#if song.qcNotes}
      <div class="mt-2.5 flex items-start gap-1.5 text-xs p-2 rounded-lg border {song.stage === 'ready_for_qc' ? 'bg-primary/10 text-primary border-primary/30' : 'bg-muted/40 text-foreground border-border'}">
        <MessageSquare class="w-3.5 h-3.5 shrink-0 mt-0.5" />
        <span>{song.qcNotes}</span>
      </div>
    {/if}
  </div>

  <!-- Responsive Action Buttons Row -->
  <div class="flex flex-col gap-2 pt-2 border-t border-border">
    <div class="flex flex-wrap gap-1.5 w-full">
      <a
        href="/studio/shows/{showId}/sprints?song={encodeURIComponent(song.title)}"
        class="flex-1 inline-flex items-center justify-center gap-1.5 text-xs font-semibold px-2.5 py-1.5 rounded-md bg-secondary text-secondary-foreground hover:bg-secondary/80 border border-border/50 transition-colors shadow-2xs"
        title="View all scheduled rehearsal sessions in Sprint Calendar"
      >
        <Clock class="w-3 h-3" />
        <span>Sprint</span>
      </a>

      {#if canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
        <Button
          variant="outline"
          size="sm"
          class="flex-1 gap-1 text-xs h-8"
          onclick={() => onOpenLineup(song)}
        >
          <Users class="w-3 h-3" />
          <span>Lineup</span>
        </Button>

        {#if song.stage === 'draft'}
          <Button
            variant="secondary"
            size="sm"
            class="flex-1 text-xs h-8 border border-border/70 font-semibold"
            onclick={() => onAdvanceStage(song, 'in_practice')}
          >
            <span>Start Practice</span>
          </Button>
        {:else if song.stage === 'in_practice'}
          <Button
            size="sm"
            class="flex-1 text-xs h-8 bg-primary hover:bg-primary/90 text-primary-foreground font-semibold"
            onclick={() => onAdvanceStage(song, 'ready_for_qc')}
          >
            <span>Submit QC</span>
          </Button>
        {/if}
      {/if}

      {#if song.stage === 'ready_for_qc' && canAuditSongScoped(userRole, false, song.qcReviewer === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
        <Button
          size="sm"
          class="flex-1 text-xs h-8 bg-primary hover:bg-primary/90 text-primary-foreground gap-1 font-semibold"
          onclick={() => onOpenQc(song)}
        >
          <UserCheck class="w-3 h-3" />
          <span>Audit QC</span>
        </Button>
      {/if}

      {#if song.stage === 'qc_approved' && canManageSongScoped(userRole, false, song.pmName === currentUserName || userRole === 'admin' || userRole === 'moderator' || userRole === 'dm')}
        <Button
          size="sm"
          class="flex-1 text-xs h-8 bg-emerald-600 hover:bg-emerald-700 text-white gap-1"
          onclick={() => onAdvanceStage(song, 'stage_ready')}
        >
          <CircleCheck class="w-3 h-3" />
          <span>Promote</span>
        </Button>
      {/if}
    </div>

    {#if song.stage === 'stage_ready'}
      <div class="inline-flex items-center justify-center gap-1.5 w-full py-1.5 text-xs font-bold text-emerald-600 bg-emerald-500/10 rounded-lg">
        <CircleCheck class="w-3.5 h-3.5 text-emerald-600" />
        <span>100% Stage Ready</span>
      </div>
    {/if}
  </div>
</Card>
