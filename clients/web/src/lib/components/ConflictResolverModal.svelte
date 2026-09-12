<script lang="ts">
  import type { UnresolvedSong, ConflictItem, DayOfWeek, SongVoteData } from '../types/timetable';
  import { TriangleAlert, X, Check } from '@lucide/svelte';
  import { tStore, currentLocale } from '$lib/i18n';
  import { DAY_DISPLAY_LABELS } from '../constants/timetableDefaults';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card, CardHeader, CardTitle, CardContent } from '$lib/components/ui/card';
  import { cn } from '$lib/utils';

  interface Props {
    unresolved: UnresolvedSong[];
    conflicts: ConflictItem[];
    songs: SongVoteData[];
    onClose: () => void;
    onManualAssign: (songId: string, day: DayOfWeek, slot: string) => void;
  }

  let { unresolved, conflicts, songs, onClose, onManualAssign }: Props = $props();

  let songsMap = $derived(new Map(songs.map(s => [s.id, s])));
  let dayLabels = $derived(DAY_DISPLAY_LABELS[$currentLocale] || DAY_DISPLAY_LABELS.vi);
</script>

<Dialog.Root open={true} onOpenChange={(open) => { if (!open) onClose(); }}>
  <Dialog.Content class="max-w-2xl max-h-[90vh] overflow-y-auto">
    <Dialog.Header>
      <div class="flex items-center gap-2">
        <TriangleAlert size={20} class="text-amber-500" />
        <Dialog.Title class="text-base font-bold">{$tStore('conflict_resolver.title')}</Dialog.Title>
      </div>
      <Dialog.Description class="text-xs text-slate-500">
        {$tStore('conflict_modal.intro')}
      </Dialog.Description>
    </Dialog.Header>

    <div class="flex flex-col gap-4 py-3">
      <!-- Unresolved Songs Section -->
      {#each unresolved as item (item.songId)}
        {@const song = songsMap.get(item.songId)}
        {@const color = song?.color}

        <Card class="border border-slate-200 bg-slate-50/50 p-4 dark:border-slate-800 dark:bg-card">
          <div class="flex items-center justify-between gap-2 mb-2">
            <div class="flex items-center gap-2">
              <div
                class="h-3 w-3 rounded-full"
                style="background-color: {color?.border || '#f59e0b'};"
              ></div>
              <strong class="text-sm font-bold text-slate-900 dark:text-slate-100">
                {item.songName}
              </strong>
            </div>
            <Badge variant="destructive" class="text-[11px]">
              {$tStore('conflict_modal.assigned_ratio', { assigned: item.assignedSessions, target: item.targetSessions })}
            </Badge>
          </div>

          <div class="text-xs text-amber-700 dark:text-amber-400 mb-3 space-y-0.5">
            {#each item.reasons as r, i (i)}
              <div>• {r}</div>
            {/each}
          </div>

          <div class="text-xs font-bold text-slate-800 dark:text-slate-200 mb-2">
            {$tStore('conflict_modal.best_suggestions')}
          </div>

          <div class="flex flex-col gap-2">
            {#each item.candidates.slice(0, 5) as cand, cIdx (cIdx)}
              {@const hasDoubleBook = cand.conflictingMembers.length > 0}
              {@const isAllFree = cand.absentMembers.length === 0}
              {@const dayName = dayLabels[cand.day]?.full || cand.day}

              <div class="flex items-center justify-between gap-3 rounded-lg border border-slate-200 bg-white p-2.5 dark:border-slate-800 dark:bg-slate-900">
                <div class="flex flex-col gap-0.5">
                  <div class="flex items-center gap-2">
                    <strong class="text-xs font-semibold text-slate-800 dark:text-slate-100">{dayName}</strong>
                    <span class="text-xs text-slate-500">({cand.slot})</span>
                    <Badge variant={isAllFree ? 'default' : 'secondary'} class="text-[10px] py-0">
                      {$tStore('conflict_modal.members_free', { available: cand.availableCount, total: cand.totalCount })}
                    </Badge>
                  </div>

                  <div class="text-[11px] text-slate-500">
                    {#if hasDoubleBook}
                      <span class="text-rose-600 dark:text-rose-400 font-medium">
                        {$tStore('conflict_modal.conflict_with_song', {
                          members: cand.conflictingMembers.join(', '),
                          songs: cand.conflictingSongs.join(', ')
                        })}
                      </span>
                    {:else if !isAllFree}
                      <span class="text-amber-600 dark:text-amber-400">
                        {$tStore('conflict_modal.absent_by_vote', { members: cand.absentMembers.join(', ') })}
                      </span>
                    {:else}
                      <span class="text-emerald-600 dark:text-emerald-400 font-medium">
                        {$tStore('conflict_modal.all_free_no_conflict')}
                      </span>
                    {/if}
                  </div>
                </div>

                <Button
                  variant={isAllFree ? 'default' : 'outline'}
                  size="sm"
                  onclick={() => onManualAssign(item.songId, cand.day, cand.slot)}
                >
                  <Check size={13} class="mr-1" />
                  <span>{$tStore('conflict_modal.choose_slot')}</span>
                </Button>
              </div>
            {/each}
          </div>
        </Card>
      {/each}

      {#if unresolved.length === 0 && conflicts.length === 0}
        <div class="p-6 text-center text-sm font-semibold text-emerald-600">
          {$tStore('conflict_modal.no_conflicts_msg')}
        </div>
      {/if}
    </div>

    <Dialog.Footer>
      <Button variant="default" onclick={onClose}>
        {$tStore('conflict_modal.close')}
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
