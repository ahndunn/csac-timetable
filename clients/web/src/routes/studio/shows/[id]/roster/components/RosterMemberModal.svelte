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
  import { canManageShowRoster } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';
  import type { ShowRosterMember } from './RosterCard.svelte';

  export interface BandRoleDef {
    id: string;
    key: string;
    icon: any;
  }

  interface Props {
    open: boolean;
    member: ShowRosterMember | null;
    userRole: UserRole;
    availableBandRoles: BandRoleDef[];
    onOpenChange: (open: boolean) => void;
    onSave: (payload: {
      fullName: string;
      email: string;
      phone?: string;
      showRole: ShowRosterMember['showRole'];
      primaryInstrument: string;
      secondaryInstruments: string[];
      practiceHours: number;
    }) => void;
  }

  let {
    open = $bindable(false),
    member,
    userRole,
    availableBandRoles,
    onOpenChange,
    onSave,
  }: Props = $props();

  let formFullName = $state('');
  let formEmail = $state('');
  let formPhone = $state('');
  let formShowRole = $state<ShowRosterMember['showRole']>('Performer');
  let formPrimaryInst = $state('vocals');
  let formSecondaryInst = $state<string[]>([]);
  let formPracticeHours = $state(0);

  $effect(() => {
    if (member) {
      formFullName = member.fullName;
      formEmail = member.email;
      formPhone = member.phone || '';
      formShowRole = member.showRole;
      formPrimaryInst = member.primaryInstrument;
      formSecondaryInst = [...member.secondaryInstruments];
      formPracticeHours = member.practiceHours;
    } else {
      formFullName = '';
      formEmail = '';
      formPhone = '';
      formShowRole = 'Performer';
      formPrimaryInst = 'vocals';
      formSecondaryInst = [];
      formPracticeHours = 0;
    }
  });

  function toggleSecondaryInstrument(instId: string) {
    if (formSecondaryInst.includes(instId)) {
      formSecondaryInst = formSecondaryInst.filter((id) => id !== instId);
    } else {
      formSecondaryInst = [...formSecondaryInst, instId];
    }
  }

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!formFullName.trim() || !formEmail.trim()) return;

    onSave({
      fullName: formFullName.trim(),
      email: formEmail.trim(),
      phone: formPhone.trim() || undefined,
      showRole: formShowRole,
      primaryInstrument: formPrimaryInst,
      secondaryInstruments: formSecondaryInst,
      practiceHours: formPracticeHours,
    });

    onOpenChange(false);
  }
</script>

<Dialog bind:open onOpenChange={onOpenChange}>
  <DialogContent class="max-w-lg">
    <DialogHeader>
      <DialogTitle>
        {member ? $tStore('show_mgmt.roster_page.btn_edit_profile') : $tStore('show_mgmt.roster_modal.title')}
      </DialogTitle>
      <DialogDescription>
        {member
          ? 'Update performer contact details, instruments, and practice allocation.'
          : $tStore('show_mgmt.roster_modal.desc')}
      </DialogDescription>
    </DialogHeader>

    <form onsubmit={handleSubmit} class="flex flex-col gap-4 mt-2">
      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <Label for="member-name">{$tStore('show_mgmt.roster_modal.label_name')} *</Label>
          <Input id="member-name" type="text" bind:value={formFullName} required />
        </div>
        <div class="flex flex-col gap-1.5">
          <Label for="member-email">{$tStore('show_mgmt.roster_modal.label_email')} *</Label>
          <Input id="member-email" type="email" bind:value={formEmail} required />
        </div>
      </div>

      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <Label for="member-show-role">{$tStore('show_mgmt.roster_modal.select_role')}</Label>
          <select
            id="member-show-role"
            bind:value={formShowRole}
            class="h-9 px-3 text-xs bg-card text-foreground border border-border rounded-md outline-none focus:ring-1 focus:ring-primary/40 disabled:opacity-50"
            disabled={!canManageShowRoster(userRole)}
          >
            <option value="DM">Delivery Manager (DM)</option>
            <option value="PM">Performance Manager (PM)</option>
            <option value="QC">Quality Reviewer (QC)</option>
            <option value="Performer">Performer / Musician</option>
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="member-phone">Phone Number</Label>
          <Input id="member-phone" type="text" bind:value={formPhone} placeholder="090-xxx-xxxx" />
        </div>
      </div>

      <div class="flex flex-col gap-1.5">
        <Label for="member-primary-inst">{$tStore('show_mgmt.roster_modal.select_instrument')} *</Label>
        <select id="member-primary-inst" bind:value={formPrimaryInst} class="h-9 px-3 text-xs bg-card text-foreground border border-border rounded-md outline-none focus:ring-1 focus:ring-primary/40">
          {#each availableBandRoles as role}
            <option value={role.id}>{$tStore(role.key)}</option>
          {/each}
        </select>
      </div>

      <div class="flex flex-col gap-1.5">
        <Label>{$tStore('show_mgmt.roster_modal.select_secondary_inst')}</Label>
        <div class="flex flex-wrap gap-1.5 p-2 bg-slate-50 dark:bg-slate-900/60 rounded-lg border border-slate-200 dark:border-slate-800">
          {#each availableBandRoles as role}
            {@const RoleIcon = role.icon}
            {#if role.id !== formPrimaryInst}
              <button
                type="button"
                class="inline-flex items-center gap-1 text-[11px] px-2 py-1 rounded border transition-colors cursor-pointer {formSecondaryInst.includes(role.id) ? 'bg-primary text-primary-foreground border-primary' : 'bg-card text-foreground border-border hover:bg-muted/50'}"
                onclick={() => toggleSecondaryInstrument(role.id)}
              >
                <RoleIcon class="w-3 h-3" />
                <span>{$tStore(role.key)}</span>
              </button>
            {/if}
          {/each}
        </div>
      </div>

      <DialogFooter class="mt-4">
        <Button
          type="button"
          variant="outline"
          onclick={() => onOpenChange(false)}
        >
          {$tStore('show_mgmt.roster_page.btn_cancel')}
        </Button>
        <Button type="submit">
          {$tStore('show_mgmt.roster_modal.btn_submit')}
        </Button>
      </DialogFooter>
    </form>
  </DialogContent>
</Dialog>
