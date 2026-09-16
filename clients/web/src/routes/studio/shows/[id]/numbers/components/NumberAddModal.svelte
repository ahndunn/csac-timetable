<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogDescription,
    DialogFooter,
  } from '$lib/components/ui/dialog';

  interface Props {
    open: boolean;
    availableRoster: string[];
    onOpenChange: (open: boolean) => void;
    onCreate: (payload: { title: string; genre: string; pm_name: string; qc_reviewer: string }) => void;
  }

  let { open = $bindable(false), availableRoster, onOpenChange, onCreate }: Props = $props();

  let newTitle = $state('');
  let newGenre = $state('');
  let newPm = $state('');
  let newQcReviewer = $state('');

  $effect(() => {
    if (availableRoster.length > 0) {
      if (!newPm) newPm = availableRoster[0];
      if (!newQcReviewer) newQcReviewer = availableRoster[1] || availableRoster[0];
    }
  });

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!newTitle.trim()) return;

    onCreate({
      title: newTitle.trim(),
      genre: newGenre.trim() || 'General Performance',
      pm_name: newPm || availableRoster[0] || 'Leader',
      qc_reviewer: newQcReviewer || availableRoster[1] || availableRoster[0] || 'Reviewer',
    });

    newTitle = '';
    newGenre = '';
    onOpenChange(false);
  }
</script>

<Dialog bind:open onOpenChange={onOpenChange}>
  <DialogContent class="max-w-md">
    <DialogHeader>
      <DialogTitle>{$tStore('studio_shows.modal_add_title')}</DialogTitle>
      <DialogDescription>{$tStore('studio_shows.modal_add_desc')}</DialogDescription>
    </DialogHeader>

    <form onsubmit={handleSubmit} class="flex flex-col gap-4 mt-2">
      <div class="flex flex-col gap-1.5">
        <Label for="new-song-title">{$tStore('studio_shows.label_song_title')}</Label>
        <Input
          id="new-song-title"
          type="text"
          bind:value={newTitle}
          placeholder="e.g. Diễm Xưa, Đi Về Nhà..."
          required
        />
      </div>

      <div class="flex flex-col gap-1.5">
        <Label for="new-song-genre">{$tStore('studio_shows.label_genre')}</Label>
        <Input
          id="new-song-genre"
          type="text"
          bind:value={newGenre}
          placeholder="e.g. Pop Rock, Acoustic Ballad, Jazz Fusion..."
        />
      </div>

      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <Label for="new-song-pm">{$tStore('studio_shows.label_pm')}</Label>
          <select id="new-song-pm" bind:value={newPm} class="h-9 px-3 text-xs bg-card text-foreground border border-border rounded-md outline-none focus:ring-1 focus:ring-primary/40">
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="new-song-qc">{$tStore('studio_shows.label_qc_reviewer')}</Label>
          <select id="new-song-qc" bind:value={newQcReviewer} class="h-9 px-3 text-xs bg-card text-foreground border border-border rounded-md outline-none focus:ring-1 focus:ring-primary/40">
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>
      </div>

      <DialogFooter class="mt-4">
        <Button variant="outline" type="button" onclick={() => onOpenChange(false)}>
          Cancel
        </Button>
        <Button type="submit">
          {$tStore('studio_shows.btn_create_number')}
        </Button>
      </DialogFooter>
    </form>
  </DialogContent>
</Dialog>
