<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { TriangleAlert } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogDescription,
    DialogFooter,
  } from '$lib/components/ui/dialog';
  import type { ShowRosterMember } from './RosterCard.svelte';

  interface Props {
    open: boolean;
    member: ShowRosterMember | null;
    onOpenChange: (open: boolean) => void;
    onConfirm: (memberId: string) => void;
  }

  let { open = $bindable(false), member, onOpenChange, onConfirm }: Props = $props();

  function handleConfirm() {
    if (member) {
      onConfirm(member.id);
      onOpenChange(false);
    }
  }
</script>

<Dialog bind:open onOpenChange={onOpenChange}>
  <DialogContent class="max-w-md">
    <DialogHeader>
      <DialogTitle class="text-red-600 flex items-center gap-2">
        <TriangleAlert class="w-5 h-5 text-red-600" />
        <span>{$tStore('show_mgmt.roster_page.confirm_remove_title')}</span>
      </DialogTitle>
      {#if member}
        <DialogDescription>
          {$tStore('show_mgmt.roster_page.confirm_remove_msg', { name: member.fullName })}
        </DialogDescription>
      {/if}
    </DialogHeader>

    <DialogFooter class="mt-4">
      <Button
        type="button"
        variant="outline"
        onclick={() => onOpenChange(false)}
      >
        {$tStore('show_mgmt.roster_page.btn_cancel')}
      </Button>
      <Button
        type="button"
        variant="destructive"
        onclick={handleConfirm}
      >
        {$tStore('show_mgmt.roster_page.btn_confirm_remove')}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
