<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import { Label } from '$lib/components/ui/label';
  import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogDescription,
    DialogFooter,
  } from '$lib/components/ui/dialog';
  import type { SongNumber } from './NumberCard.svelte';

  interface Props {
    open: boolean;
    song: SongNumber | null;
    availableRoster: string[];
    onOpenChange: (open: boolean) => void;
    onSaveLineup: (songId: string, lineup: NonNullable<SongNumber['lineup']>) => void;
  }

  let { open = $bindable(false), song, availableRoster, onOpenChange, onSaveLineup }: Props = $props();

  let formVocalLead = $state('');
  let formGuitarLead = $state('');
  let formBass = $state('');
  let formDrums = $state('');
  let formKeys = $state('');

  $effect(() => {
    if (song) {
      formVocalLead = song.lineup?.vocalLead || '';
      formGuitarLead = song.lineup?.guitarLead || '';
      formBass = song.lineup?.bass || '';
      formDrums = song.lineup?.drums || '';
      formKeys = song.lineup?.keys || '';
    }
  });

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!song) return;

    onSaveLineup(song.id, {
      vocalLead: formVocalLead || undefined,
      guitarLead: formGuitarLead || undefined,
      bass: formBass || undefined,
      drums: formDrums || undefined,
      keys: formKeys || undefined,
    });

    onOpenChange(false);
  }
</script>

<Dialog bind:open onOpenChange={onOpenChange}>
  <DialogContent class="max-w-md">
    <DialogHeader>
      <DialogTitle>Assign Band Lineup</DialogTitle>
      {#if song}
        <DialogDescription>
          Song: <strong class="text-foreground">{song.title}</strong> ({song.genre})
        </DialogDescription>
      {/if}
    </DialogHeader>

    {#if song}
      <form onsubmit={handleSubmit} class="flex flex-col gap-3.5 mt-2">
        <div class="flex flex-col gap-1.5">
          <Label for="role-vocal">Vocal Lead</Label>
          <select id="role-vocal" bind:value={formVocalLead} class="h-9 px-3 text-xs bg-card text-foreground border border-border rounded-md outline-none focus:ring-1 focus:ring-primary/40">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="role-guitar">Guitar Lead / Solo</Label>
          <select id="role-guitar" bind:value={formGuitarLead} class="h-9 px-3 text-xs bg-card text-foreground border border-border rounded-md outline-none focus:ring-1 focus:ring-primary/40">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="role-bass">Bass Guitar</Label>
          <select id="role-bass" bind:value={formBass} class="h-9 px-3 text-xs bg-card text-foreground border border-border rounded-md outline-none focus:ring-1 focus:ring-primary/40">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="role-drums">Drum Kit</Label>
          <select id="role-drums" bind:value={formDrums} class="h-9 px-3 text-xs bg-card text-foreground border border-border rounded-md outline-none focus:ring-1 focus:ring-primary/40">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="role-keys">Keyboard / Synthesizer</Label>
          <select id="role-keys" bind:value={formKeys} class="h-9 px-3 text-xs bg-card text-foreground border border-border rounded-md outline-none focus:ring-1 focus:ring-primary/40">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <DialogFooter class="mt-4">
          <Button variant="outline" type="button" onclick={() => onOpenChange(false)}>Cancel</Button>
          <Button type="submit">
            Save Lineup Allocation
          </Button>
        </DialogFooter>
      </form>
    {/if}
  </DialogContent>
</Dialog>
