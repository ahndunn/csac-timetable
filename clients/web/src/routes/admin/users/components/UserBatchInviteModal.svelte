<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { Layers, LoaderCircle } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Label } from '$lib/components/ui/label';
  import * as Dialog from '$lib/components/ui/dialog';

  interface Props {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    onBatchInvite: (emails: string[], role: 'admin' | 'moderator' | 'member') => Promise<void>;
  }

  let { open = $bindable(false), onOpenChange, onBatchInvite }: Props = $props();

  let batchEmails = $state('');
  let batchRole = $state<'admin' | 'moderator' | 'member'>('member');
  let isBatchInviting = $state(false);
  let localError = $state<string | null>(null);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    localError = null;

    const rawList = batchEmails
      .split(/[\n,;]+/)
      .map((s) => s.trim())
      .filter((s) => s.length > 0 && s.includes('@'));

    if (rawList.length === 0) {
      localError = 'Please enter at least one valid email address.';
      return;
    }

    isBatchInviting = true;
    try {
      await onBatchInvite(rawList, batchRole);
      batchEmails = '';
      batchRole = 'member';
      onOpenChange(false);
    } catch (err: any) {
      localError = err.message || 'Failed to dispatch batch invitations';
    } finally {
      isBatchInviting = false;
    }
  }
</script>

<Dialog.Root bind:open onOpenChange={onOpenChange}>
  <Dialog.Content class="sm:max-w-md">
    <Dialog.Header>
      <div class="flex items-center gap-2">
        <Layers class="w-5 h-5 text-primary" />
        <Dialog.Title class="text-base font-bold">
          {$tStore('admin_users.modal_batch_title')}
        </Dialog.Title>
      </div>
      <Dialog.Description class="text-xs text-muted-foreground">
        {$tStore('admin_users.modal_batch_desc')}
      </Dialog.Description>
    </Dialog.Header>

    {#if localError}
      <div class="p-2.5 text-xs text-red-600 bg-red-50 dark:bg-red-950/40 border border-red-200 dark:border-red-800 rounded-lg">
        {localError}
      </div>
    {/if}

    <form onsubmit={handleSubmit} class="flex flex-col gap-3.5 py-2">
      <div class="flex flex-col gap-1.5">
        <Label for="batch-emails" class="text-xs font-semibold">
          {$tStore('admin_users.label_batch_emails')} <span class="text-red-500">*</span>
        </Label>
        <textarea
          id="batch-emails"
          rows="5"
          placeholder="guitarist@csac.vn&#10;drummer@csac.vn&#10;vocalist@csac.vn"
          bind:value={batchEmails}
          required
          class="text-xs p-2.5 rounded-md border border-input bg-card font-mono resize-none focus:outline-hidden focus:ring-1 focus:ring-ring"
        ></textarea>
        <span class="text-[10px] text-muted-foreground">
          {$tStore('admin_users.batch_emails_hint')}
        </span>
      </div>

      <div class="flex flex-col gap-1.5">
        <Label for="batch-role" class="text-xs font-semibold">
          {$tStore('admin_users.label_role')}
        </Label>
        <select
          id="batch-role"
          bind:value={batchRole}
          class="h-9 px-2.5 text-xs bg-card border border-border rounded-md outline-none cursor-pointer"
        >
          <option value="member">{$tStore('admin_users.role_member')}</option>
          <option value="moderator">{$tStore('admin_users.role_moderator')}</option>
          <option value="admin">{$tStore('admin_users.role_admin')}</option>
        </select>
      </div>

      <Dialog.Footer class="mt-4 gap-2">
        <Button
          type="button"
          variant="outline"
          size="sm"
          onclick={() => onOpenChange(false)}
        >
          {$tStore('admin_users.btn_cancel')}
        </Button>
        <Button
          type="submit"
          size="sm"
          disabled={isBatchInviting || !batchEmails.trim()}
          class="gap-1.5"
        >
          {#if isBatchInviting}
            <LoaderCircle class="w-3.5 h-3.5 animate-spin" />
            <span>{$tStore('admin_users.btn_sending')}</span>
          {:else}
            <Layers class="w-3.5 h-3.5" />
            <span>{$tStore('admin_users.btn_send_batch')}</span>
          {/if}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
