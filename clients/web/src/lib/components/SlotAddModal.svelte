<script lang="ts">
  import type { SongVoteData, ScheduledSession, DayOfWeek } from '../types/timetable';
  import { getSlotAttendance } from '../engine/scheduler';
  import { Plus, X, AlertCircle } from '@lucide/svelte';
  import { tStore, currentLocale } from '$lib/i18n';
  import { DAY_DISPLAY_LABELS } from '../constants/timetableDefaults';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';

  interface Props {
    day: DayOfWeek | null;
    slot: string | null;
    songs: SongVoteData[];
    schedule: ScheduledSession[];
    onClose: () => void;
    onAssignSong: (song: SongVoteData, day: DayOfWeek, slot: string) => void;
  }

  let { day, slot, songs, schedule, onClose, onAssignSong }: Props = $props();

  let existingInSlot = $derived(
    day && slot ? schedule.filter(s => s.day === day && s.slot === slot) : []
  );

  let dayLabels = $derived(DAY_DISPLAY_LABELS[$currentLocale] || DAY_DISPLAY_LABELS.vi);
  let localizedDay = $derived(day ? (dayLabels[day]?.full || day) : '');
</script>

{#if day && slot}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) onClose(); }}>
    <Dialog.Content class="max-w-xl max-h-[85vh] overflow-y-auto">
      <Dialog.Header>
        <div class="flex items-center gap-2">
          <Plus size={18} class="text-primary" />
          <Dialog.Title class="text-base font-bold">
            {$tStore('slot_add_modal.title', { day: localizedDay, slot })}
          </Dialog.Title>
        </div>
        <Dialog.Description class="text-xs text-muted-foreground">
          {$tStore('slot_add_modal.intro')}
        </Dialog.Description>
      </Dialog.Header>

      <div class="flex flex-col gap-2.5 py-2">
        {#each songs as song (song.id)}
          {@const attendance = getSlotAttendance(song, day, slot)}
          {@const doubleBookedMembers = existingInSlot.flatMap(existing => 
            song.members.filter(m => existing.allMembers.includes(m)).map(m => 
              $tStore('slot_add_modal.busy_in_song', { member: m, song: existing.songName })
            )
          )}
          {@const isAlreadyInSlot = existingInSlot.some(s => s.songId === song.id)}

          <Card class="flex items-center justify-between p-3 border border-slate-200 bg-white dark:border-slate-800 dark:bg-card {isAlreadyInSlot ? 'opacity-60 bg-slate-50' : ''}">
            <div class="flex flex-col gap-1">
              <div class="flex items-center gap-2">
                <div
                  class="h-3 w-3 rounded-full"
                  style="background-color: {song.color.border};"
                ></div>
                <strong class="text-xs font-bold text-slate-900 dark:text-slate-100">
                  {song.name}
                </strong>
                <Badge variant={attendance.is100Percent ? 'default' : 'destructive'} class="text-[10px] py-0">
                  {$tStore('slot_add_modal.members_free', { available: attendance.availableMembers.length, total: song.members.length })}
                </Badge>
              </div>

              {#if doubleBookedMembers.length > 0}
                <div class="flex items-center gap-1 text-[11px] text-rose-600 dark:text-rose-400">
                  <AlertCircle size={12} />
                  <span>{$tStore('slot_add_modal.overlap_warning', { members: doubleBookedMembers.join(', ') })}</span>
                </div>
              {/if}

              {#if attendance.absentMembers.length > 0 && doubleBookedMembers.length === 0}
                <div class="text-[11px] text-amber-600 dark:text-amber-400">
                  {$tStore('slot_add_modal.absent_by_vote', { members: attendance.absentMembers.join(', ') })}
                </div>
              {/if}
            </div>

            <Button
              variant={isAlreadyInSlot ? 'outline' : 'default'}
              size="sm"
              disabled={isAlreadyInSlot}
              onclick={() => {
                onAssignSong(song, day, slot);
                onClose();
              }}
            >
              {isAlreadyInSlot ? $tStore('slot_add_modal.already_assigned') : $tStore('slot_add_modal.assign_btn')}
            </Button>
          </Card>
        {/each}
      </div>

      <Dialog.Footer>
        <Button variant="outline" size="sm" onclick={onClose}>
          {$tStore('slot_add_modal.close')}
        </Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
{/if}
