<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { MailCheck, LoaderCircle } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import * as Dialog from '$lib/components/ui/dialog';
  import type { UserAccount } from '$lib/types/timetable';

  interface Props {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    onInvite: (payload: {
      email: string;
      full_name?: string;
      phone?: string;
      role: 'admin' | 'moderator' | 'member';
    }) => Promise<void>;
  }

  let { open = $bindable(false), onOpenChange, onInvite }: Props = $props();

  let inviteEmail = $state('');
  let inviteFullName = $state('');
  let invitePhone = $state('');
  let inviteRole = $state<'admin' | 'moderator' | 'member'>('member');
  let isInviting = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!inviteEmail.trim()) return;

    isInviting = true;
    try {
      await onInvite({
        email: inviteEmail.trim(),
        full_name: inviteFullName.trim() || undefined,
        phone: invitePhone.trim() || undefined,
        role: inviteRole,
      });
      inviteEmail = '';
      inviteFullName = '';
      invitePhone = '';
      inviteRole = 'member';
      onOpenChange(false);
    } finally {
      isInviting = false;
    }
  }
</script>

<Dialog.Root bind:open onOpenChange={onOpenChange}>
  <Dialog.Content class="sm:max-w-md">
    <Dialog.Header>
      <div class="flex items-center gap-2">
        <MailCheck class="w-5 h-5 text-primary" />
        <Dialog.Title class="text-base font-bold">
          {$tStore('admin_users.modal_invite_title')}
        </Dialog.Title>
      </div>
      <Dialog.Description class="text-xs text-muted-foreground">
        {$tStore('admin_users.modal_invite_desc')}
      </Dialog.Description>
    </Dialog.Header>

    <form onsubmit={handleSubmit} class="flex flex-col gap-3.5 py-2">
      <div class="flex flex-col gap-1.5">
        <Label for="invite-email" class="text-xs font-semibold">
          {$tStore('admin_users.label_email')} <span class="text-red-500">*</span>
        </Label>
        <Input
          id="invite-email"
          type="email"
          placeholder="member@university.edu"
          bind:value={inviteEmail}
          required
          class="text-xs h-9"
        />
      </div>

      <div class="flex flex-col gap-1.5">
        <Label for="invite-name" class="text-xs font-semibold">
          {$tStore('admin_users.label_fullname')} <span class="text-muted-foreground text-[10px]">({$tStore('admin_users.optional_label')})</span>
        </Label>
        <Input
          id="invite-name"
          type="text"
          placeholder="e.g. Nguyễn Văn A"
          bind:value={inviteFullName}
          class="text-xs h-9"
        />
      </div>

      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <Label for="invite-phone" class="text-xs font-semibold">
            {$tStore('admin_users.label_phone')} <span class="text-muted-foreground text-[10px]">({$tStore('admin_users.optional_label')})</span>
          </Label>
          <Input
            id="invite-phone"
            type="tel"
            placeholder="0901234567"
            bind:value={invitePhone}
            class="text-xs h-9"
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="invite-role" class="text-xs font-semibold">
            {$tStore('admin_users.label_role')}
          </Label>
          <select
            id="invite-role"
            bind:value={inviteRole}
            class="h-9 px-2.5 text-xs bg-card border border-border rounded-md outline-none cursor-pointer"
          >
            <option value="member">{$tStore('admin_users.role_member')}</option>
            <option value="moderator">{$tStore('admin_users.role_moderator')}</option>
            <option value="admin">{$tStore('admin_users.role_admin')}</option>
          </select>
        </div>
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
          disabled={isInviting || !inviteEmail.trim()}
          class="gap-1.5"
        >
          {#if isInviting}
            <LoaderCircle class="w-3.5 h-3.5 animate-spin" />
            <span>{$tStore('admin_users.btn_sending')}</span>
          {:else}
            <MailCheck class="w-3.5 h-3.5" />
            <span>{$tStore('admin_users.btn_send_invite')}</span>
          {/if}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
