<script lang="ts">
  import type { SongVoteData, DayOfWeek } from '../types/timetable';
  import { DAYS_OF_WEEK, DEFAULT_TIME_SLOTS, DAY_DISPLAY_LABELS } from '../constants/timetableDefaults';
  import { X, CheckSquare, Square } from '@lucide/svelte';
  import { tStore, currentLocale } from '$lib/i18n';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';

  interface Props {
    song: SongVoteData | null;
    onClose: () => void;
    onToggleVote: (songId: string, day: DayOfWeek, slot: string, member: string) => void;
  }

  let { song, onClose, onToggleVote }: Props = $props();

  const days = DAYS_OF_WEEK;
  const slots = DEFAULT_TIME_SLOTS;

  let dayLabels = $derived(DAY_DISPLAY_LABELS[$currentLocale] || DAY_DISPLAY_LABELS.vi);
</script>

{#if song}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) onClose(); }}>
    <Dialog.Content class="max-w-4xl max-h-[90vh] overflow-y-auto">
      <Dialog.Header>
        <div class="flex items-center gap-2">
          <div
            class="h-3.5 w-3.5 rounded-full"
            style="background-color: {song.color.border};"
          ></div>
          <Dialog.Title class="text-base font-bold">
            {$tStore('raw_vote_modal.title', { song: song.name })}
          </Dialog.Title>
        </div>
        <Dialog.Description class="text-xs text-slate-500">
          {$tStore('raw_vote_modal.hint')}
        </Dialog.Description>
      </Dialog.Header>

      <div class="overflow-x-auto py-2">
        <table class="w-full border-collapse border border-slate-200 text-center text-xs dark:border-slate-800">
          <thead>
            <!-- Sheet Title Row -->
            <tr>
              <th
                colspan={3 + song.members.length}
                class="bg-primary/10 p-2 text-xs font-bold text-primary border-b border-border"
              >
                {song.weekTitle || $tStore('raw_vote_modal.default_title')}
              </th>
            </tr>

            <!-- Header row 1 -->
            <tr class="bg-slate-50 dark:bg-slate-800">
              <th rowspan={2} class="border border-slate-200 p-2 font-bold text-slate-700 dark:border-slate-700 dark:text-slate-200 w-20">{$tStore('raw_vote_modal.th_day')}</th>
              <th rowspan={2} class="border border-slate-200 p-2 font-bold text-slate-700 dark:border-slate-700 dark:text-slate-200 w-24">{$tStore('raw_vote_modal.th_time')}</th>
              <th colspan={song.members.length} class="border border-slate-200 p-2 font-bold text-slate-700 dark:border-slate-700 dark:text-slate-200">{$tStore('raw_vote_modal.th_members')}</th>
              <th rowspan={2} class="border border-slate-200 p-2 font-bold text-slate-700 dark:border-slate-700 dark:text-slate-200 w-28">{$tStore('raw_vote_modal.th_notes')}</th>
            </tr>

            <!-- Header row 2: Member names -->
            <tr class="bg-slate-50 dark:bg-slate-800">
              {#each song.members as m (m)}
                <th class="border border-slate-200 p-1.5 font-semibold text-slate-700 dark:border-slate-700 dark:text-slate-300 min-w-[70px]">{m}</th>
              {/each}
            </tr>
          </thead>

          <tbody>
            {#each days as day}
              {@const localizedDay = dayLabels[day]?.full || day}
              {#each slots as slot, slotIdx}
                {@const noteKey = `${day}__${slot}`}
                {@const note = song.notes[noteKey] || ''}

                <tr class="hover:bg-slate-50/50 dark:hover:bg-slate-800/50">
                  {#if slotIdx === 0}
                    <td rowspan={slots.length} class="border border-slate-200 p-2 font-bold text-slate-800 align-middle bg-slate-50/30 dark:border-slate-800 dark:text-slate-200 dark:bg-slate-900/40">
                      {localizedDay}
                    </td>
                  {/if}

                  <td class="border border-slate-200 p-1.5 text-[11px] text-slate-600 dark:border-slate-800 dark:text-slate-400">{slot}</td>

                  {#each song.members as member (member)}
                    {@const key = `${day}__${slot}__${member}`}
                    {@const isChecked = !!song.availability[key]}

                    <td
                      class="border border-slate-200 p-1.5 cursor-pointer transition-colors dark:border-slate-800 {isChecked ? 'bg-emerald-50/60 dark:bg-emerald-950/20' : ''}"
                      onclick={() => onToggleVote(song.id, day, slot, member)}
                      title={$tStore('raw_vote_modal.toggle_tooltip', { member })}
                    >
                      <div class="flex items-center justify-center">
                        {#if isChecked}
                          <CheckSquare size={16} class="text-emerald-600" />
                        {:else}
                          <Square size={16} class="text-slate-300 dark:text-slate-600" />
                        {/if}
                      </div>
                    </td>
                  {/each}

                  <td class="border border-slate-200 p-1.5 text-left text-[11px] text-slate-600 dark:border-slate-800 dark:text-slate-400">
                    {note}
                  </td>
                </tr>
              {/each}
            {/each}
          </tbody>
        </table>
      </div>

      <Dialog.Footer>
        <Button variant="default" size="sm" onclick={onClose}>
          {$tStore('raw_vote_modal.close')}
        </Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
{/if}
