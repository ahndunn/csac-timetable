<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    ShieldCheck,
    UserCheck,
    ThumbsUp,
    RefreshCw,
    CircleCheck,
  } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Label } from '$lib/components/ui/label';
  import {
    Dialog,
    DialogContent,
    DialogTitle,
    DialogFooter,
  } from '$lib/components/ui/dialog';
  import type { SongNumber } from './NumberCard.svelte';

  interface Props {
    open: boolean;
    song: SongNumber | null;
    onOpenChange: (open: boolean) => void;
    onSubmitVerdict: (songId: string, verdict: 'pass' | 'revision', notes: string) => void;
  }

  let { open = $bindable(false), song, onOpenChange, onSubmitVerdict }: Props = $props();

  let qcVerdict = $state<'pass' | 'revision'>('pass');
  let qcNotesInput = $state('');

  $effect(() => {
    if (song) {
      qcNotesInput = song.qcNotes || '';
      qcVerdict = 'pass';
    }
  });

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!song) return;

    onSubmitVerdict(song.id, qcVerdict, qcNotesInput.trim());
    onOpenChange(false);
  }
</script>

<Dialog bind:open onOpenChange={onOpenChange}>
  <DialogContent class="sm:max-w-xl w-full p-0 overflow-hidden border border-border/80 shadow-2xl rounded-2xl bg-card">
    <div class="p-5 sm:p-6 pb-4 border-b border-border/60 bg-gradient-to-b from-muted/40 to-card">
      <div class="flex items-center gap-2.5 mb-1.5">
        <div class="flex items-center justify-center w-8 h-8 rounded-xl bg-primary/10 text-primary border border-primary/20 shadow-xs shrink-0">
          <ShieldCheck class="w-4 h-4 text-primary" />
        </div>
        <div>
          <DialogTitle class="text-base font-bold text-foreground tracking-tight">
            {$tStore('studio_shows.qc_drawer_title')}
          </DialogTitle>
          <p class="text-xs text-muted-foreground mt-0.5">
            Stage Quality Assurance & Rehearsal Assessment
          </p>
        </div>
      </div>

      {#if song}
        <div class="mt-3.5 p-2.5 sm:p-3 rounded-xl bg-background/80 border border-border/70 flex items-center justify-between gap-3 text-xs shadow-xs">
          <div class="flex items-center gap-2 min-w-0 flex-1">
            <div class="w-2 h-2 rounded-full bg-primary shrink-0 animate-pulse"></div>
            <div class="truncate">
              <span class="text-muted-foreground font-medium">Song:</span>
              <span class="font-bold text-foreground ml-1">{song.title}</span>
              {#if song.genre}
                <span class="text-[11px] text-muted-foreground ml-1.5 px-1.5 py-0.5 rounded-md bg-muted font-normal inline-block">
                  {song.genre}
                </span>
              {/if}
            </div>
          </div>
          <div class="shrink-0 flex items-center gap-1.5 text-muted-foreground bg-muted/60 px-2.5 py-1 rounded-lg border border-border/50 text-[11px] sm:text-xs">
            <UserCheck class="w-3.5 h-3.5 text-primary" />
            <span class="font-semibold text-foreground">{song.qcReviewer}</span>
          </div>
        </div>
      {/if}
    </div>

    {#if song}
      <form onsubmit={handleSubmit} class="p-5 sm:p-6 pt-4 sm:pt-5 flex flex-col gap-4 sm:gap-5">
        <div class="flex flex-col gap-2">
          <div class="flex items-center justify-between">
            <Label class="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {$tStore('studio_shows.qc_verdict')}
            </Label>
            <span class="text-[11px] font-medium text-muted-foreground">Select decision</span>
          </div>

          <div class="grid grid-cols-2 gap-3">
            <button
              type="button"
              onclick={() => (qcVerdict = 'pass')}
              class="relative flex flex-col gap-2 p-3.5 rounded-xl border text-left transition-all duration-200 cursor-pointer {qcVerdict === 'pass'
                ? 'border-emerald-500/80 bg-emerald-500/10 shadow-sm ring-2 ring-emerald-500/20'
                : 'border-border/80 bg-card hover:bg-muted/40 hover:border-border'}"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center justify-center w-7 h-7 rounded-lg {qcVerdict === 'pass' ? 'bg-emerald-500 text-white' : 'bg-muted text-muted-foreground'} transition-colors">
                  <ThumbsUp class="w-3.5 h-3.5" />
                </div>
                {#if qcVerdict === 'pass'}
                  <span class="flex h-2 w-2 rounded-full bg-emerald-500"></span>
                {/if}
              </div>
              <div>
                <div class="text-xs font-bold {qcVerdict === 'pass' ? 'text-emerald-700 dark:text-emerald-400' : 'text-foreground'}">
                  {$tStore('studio_shows.qc_pass')}
                </div>
                <div class="text-[11px] text-muted-foreground mt-0.5 line-clamp-2">
                  Ready to advance to Stage Ready pipeline.
                </div>
              </div>
            </button>

            <button
              type="button"
              onclick={() => (qcVerdict = 'revision')}
              class="relative flex flex-col gap-2 p-3.5 rounded-xl border text-left transition-all duration-200 cursor-pointer {qcVerdict === 'revision'
                ? 'border-amber-500/80 bg-amber-500/10 shadow-sm ring-2 ring-amber-500/20'
                : 'border-border/80 bg-card hover:bg-muted/40 hover:border-border'}"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center justify-center w-7 h-7 rounded-lg {qcVerdict === 'revision' ? 'bg-amber-500 text-white' : 'bg-muted text-muted-foreground'} transition-colors">
                  <RefreshCw class="w-3.5 h-3.5" />
                </div>
                {#if qcVerdict === 'revision'}
                  <span class="flex h-2 w-2 rounded-full bg-amber-500"></span>
                {/if}
              </div>
              <div>
                <div class="text-xs font-bold {qcVerdict === 'revision' ? 'text-amber-700 dark:text-amber-400' : 'text-foreground'}">
                  {$tStore('studio_shows.qc_revision')}
                </div>
                <div class="text-[11px] text-muted-foreground mt-0.5 line-clamp-2">
                  Return to practice band with audit feedback.
                </div>
              </div>
            </button>
          </div>
        </div>

        <div class="flex flex-col gap-1.5">
          <div class="flex items-center justify-between">
            <Label for="qc-notes" class="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {$tStore('studio_shows.qc_notes')}
            </Label>
            <span class="text-[11px] text-muted-foreground">Required</span>
          </div>
          <textarea
            id="qc-notes"
            bind:value={qcNotesInput}
            rows="3"
            placeholder="Provide specific notes on vocal intonation, rhythm tightness, instrument balance..."
            required
            class="w-full p-3 text-xs leading-relaxed bg-background border border-input rounded-xl outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary transition-all shadow-xs resize-none placeholder:text-muted-foreground/60"
          ></textarea>
        </div>

        <DialogFooter class="flex items-center justify-end gap-2.5 pt-2 border-t border-border/50 mt-1">
          <Button
            variant="ghost"
            type="button"
            class="h-9 px-4 text-xs font-semibold rounded-xl text-muted-foreground hover:text-foreground"
            onclick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button
            type="submit"
            class="h-9 px-5 text-xs font-bold rounded-xl gap-1.5 shadow-sm transition-all {qcVerdict === 'pass' ? 'bg-emerald-600 hover:bg-emerald-700 text-white' : 'bg-primary hover:bg-primary/90 text-primary-foreground'}"
          >
            {#if qcVerdict === 'pass'}
              <CircleCheck class="w-3.5 h-3.5" />
            {:else}
              <RefreshCw class="w-3.5 h-3.5" />
            {/if}
            {$tStore('studio_shows.btn_submit_qc')}
          </Button>
        </DialogFooter>
      </form>
    {/if}
  </DialogContent>
</Dialog>
