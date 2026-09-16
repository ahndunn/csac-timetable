<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { ShieldAlert, ArrowDown } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Label } from '$lib/components/ui/label';
  import * as Dialog from '$lib/components/ui/dialog';
  import type { UserAccount } from '$lib/types/timetable';

  interface Props {
    open: boolean;
    user: UserAccount | null;
    onOpenChange: (open: boolean) => void;
    onProposeDowngrade: (userId: string, targetRole: 'moderator' | 'member', reason: string) => Promise<void>;
  }

  let { open = $bindable(false), user, onOpenChange, onProposeDowngrade }: Props = $props();

  let targetRole = $state<'moderator' | 'member'>('member');
  let reason = $state('');

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!user || !reason.trim()) return;

    onProposeDowngrade(user.id, targetRole, reason.trim());
    reason = '';
    onOpenChange(false);
  }
</script>

<Dialog.Root bind:open onOpenChange={onOpenChange}>
  <Dialog.Content class="sm:max-w-md">
    <Dialog.Header>
      <div class="flex items-center gap-2">
        <ShieldAlert class="w-5 h-5 text-amber-600" />
        <Dialog.Title class="text-base font-bold">
          {$tStore('admin_users.downgrade_modal_title')}
        </Dialog.Title>
      </div>
      <Dialog.Description class="text-xs text-muted-foreground">
        {$tStore('admin_users.downgrade_modal_desc')}
      </Dialog.Description>
    </Dialog.Header>

    {#if user}
      <form onsubmit={handleSubmit} class="flex flex-col gap-3.5 py-2">
        <div class="p-2.5 rounded-lg bg-muted text-xs flex flex-col gap-1">
          <div><span class="text-muted-foreground">Target Admin:</span> <strong>{user.full_name}</strong></div>
          <div><span class="text-muted-foreground">Email:</span> {user.email}</div>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="downgrade-target-role" class="text-xs font-semibold">
            {$tStore('admin_users.downgrade_target_role')}
          </Label>
          <select
            id="downgrade-target-role"
            bind:value={targetRole}
            class="h-9 px-2.5 text-xs bg-card border border-border rounded-md outline-none cursor-pointer"
          >
            <option value="moderator">{$tStore('admin_users.role_moderator')}</option>
            <option value="member">{$tStore('admin_users.role_member')}</option>
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="downgrade-reason" class="text-xs font-semibold">
            {$tStore('admin_users.downgrade_reason')} <span class="text-red-500">*</span>
          </Label>
          <textarea
            id="downgrade-reason"
            rows="3"
            placeholder="Explain why this admin role transition is needed..."
            bind:value={reason}
            required
            class="text-xs p-2.5 rounded-md border border-input bg-card resize-none focus:outline-hidden focus:ring-1 focus:ring-ring"
          ></textarea>
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
            variant="destructive"
            size="sm"
            disabled={!reason.trim()}
            class="gap-1.5"
          >
            <ArrowDown class="w-3.5 h-3.5" />
            <span>{$tStore('admin_users.btn_submit_proposal')}</span>
          </Button>
        </Dialog.Footer>
      </form>
    {/if}
  </Dialog.Content>
</Dialog.Root>
