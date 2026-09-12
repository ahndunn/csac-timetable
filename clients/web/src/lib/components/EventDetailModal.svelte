<script lang="ts">
  import type { ScheduledSession, SongVoteData } from '../types/timetable';
  import { Clock, MapPin, Users, FileText, Trash2, Check, CircleAlert } from '@lucide/svelte';
  import { tStore, currentLocale } from '$lib/i18n';
  import { DAY_DISPLAY_LABELS } from '../constants/timetableDefaults';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';

  interface Props {
    session: ScheduledSession | null;
    songData?: SongVoteData;
    onClose: () => void;
    onDeleteSession: (sessionId: string) => void;
  }

  let { session, songData, onClose, onDeleteSession }: Props = $props();

  let isPerfect = $derived(session ? session.absentMembers.length === 0 : true);
  let dayLabels = $derived(DAY_DISPLAY_LABELS[$currentLocale] || DAY_DISPLAY_LABELS.vi);
  let localizedDay = $derived(session ? (dayLabels[session.day]?.full || session.day) : '');
</script>

{#if session}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) onClose(); }}>
    <Dialog.Content class="max-w-lg">
      <Dialog.Header>
        <div class="flex items-center gap-2">
          <div
            class="h-3.5 w-3.5 rounded-full"
            style="background-color: {session.color.border};"
          ></div>
          <Dialog.Title class="text-base font-bold">
            {session.songName}
          </Dialog.Title>
        </div>
      </Dialog.Header>

      <div class="flex flex-col gap-3 py-2">
        <div class="flex flex-col gap-2">
          <div class="flex items-center gap-2 text-xs font-semibold text-foreground">
            <Clock size={15} class="text-primary" />
            <span>
              <strong>{localizedDay}</strong>, {session.slot}
            </span>
          </div>

          <div class="flex items-center gap-2 text-xs font-semibold text-foreground">
            <MapPin size={15} class="text-primary" />
            <span>{$tStore('event_detail_modal.room_label', { room: session.room })}</span>
          </div>
        </div>

        <Card class="border border-border bg-muted/30 p-3.5">
          <div class="flex items-center justify-between mb-2.5">
            <span class="flex items-center gap-1.5 text-xs font-bold text-foreground">
              <Users size={14} class="text-primary" />
              <span>{$tStore('event_detail_modal.members_header', { present: session.availableMembers.length, total: session.allMembers.length })}</span>
            </span>

            <Badge variant={isPerfect ? 'default' : 'destructive'} class="text-[10px]">
              {isPerfect ? $tStore('event_detail_modal.full_attendance') : $tStore('event_detail_modal.absent_count', { count: session.absentMembers.length })}
            </Badge>
          </div>

          <div class="grid grid-cols-2 gap-2 rounded-lg border border-border bg-card p-2.5">
            {#each session.allMembers as m}
              {@const isAvail = session.availableMembers.includes(m)}
              <div class="flex items-center gap-1.5 text-xs {isAvail ? 'text-foreground' : 'text-muted-foreground'}">
                {#if isAvail}
                  <Check size={13} class="text-emerald-600" />
                {:else}
                  <CircleAlert size={13} class="text-destructive" />
                {/if}
                <span class={isAvail ? 'font-medium' : 'line-through'}>
                  {m}
                </span>
                {#if !isAvail}
                  <span class="text-[10px] font-bold text-destructive">
                    {$tStore('event_detail_modal.busy_label')}
                  </span>
                {/if}
              </div>
            {/each}
          </div>
        </Card>

        {#if session.note}
          <div class="rounded-lg border border-border bg-muted/30 p-2.5">
            <div class="flex items-center gap-1.5 text-xs font-bold text-muted-foreground mb-1">
              <FileText size={13} class="text-primary" />
              <span>{$tStore('event_detail_modal.notes_title')}</span>
            </div>
            <div class="text-xs text-foreground">
              {session.note}
            </div>
          </div>
        {/if}
      </div>

      <Dialog.Footer class="flex items-center justify-between gap-2 sm:justify-between">
        <Button
          variant="destructive"
          size="sm"
          onclick={() => onDeleteSession(session.id)}
        >
          <Trash2 size={14} class="mr-1.5" />
          <span>{$tStore('event_detail_modal.delete_btn')}</span>
        </Button>
        <Button variant="default" size="sm" onclick={onClose}>
          {$tStore('event_detail_modal.close')}
        </Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
{/if}
