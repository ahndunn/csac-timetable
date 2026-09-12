<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { api, ApiError } from '$lib/api/client';
  import { tStore, t } from '$lib/i18n';
  import {
    Users,
    UserPlus,
    ShieldAlert,
    Shield,
    EllipsisVertical,
    Search,
    CircleCheck,
    CircleX,
    CircleAlert,
    LoaderCircle,
    Lock,
    LockOpen,
    ArrowDown,
    X,
  } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import * as Table from '$lib/components/ui/table';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
  import * as Dialog from '$lib/components/ui/dialog';

  interface UserAccount {
    id: string;
    email: string;
    full_name: string;
    role: 'admin' | 'moderator' | 'member';
    status: 'active' | 'suspended';
    created_at: string;
  }

  let users = $state<UserAccount[]>([]);
  let isLoading = $state(true);
  let search = $state('');
  let roleFilter = $state<'all' | 'admin' | 'moderator' | 'member'>('all');

  // Modal States
  let isCreateModalOpen = $state(false);
  let isDowngradeModalOpen = $state(false);
  let selectedUserForDowngrade = $state<UserAccount | null>(null);
  let downgradeReason = $state('');
  let targetDowngradeRole = $state<'moderator' | 'member'>('member');

  // New User Form State
  let newEmail = $state('');
  let newFullName = $state('');
  let newPassword = $state('');
  let newRole = $state<'admin' | 'moderator' | 'member'>('member');

  let successMessage = $state<string | null>(null);
  let errorMessage = $state<string | null>(null);

  async function loadUsers() {
    isLoading = true;
    try {
      const res = await api.users.list();
      users = res.users || [];
    } catch {
      users = [
        {
          id: 'u-1',
          email: 'admin@csac.local',
          full_name: 'System Administrator',
          role: 'admin',
          status: 'active',
          created_at: '2026-01-01T00:00:00Z',
        },
        {
          id: 'u-2',
          email: 'hoangnam@csac.local',
          full_name: 'Hoàng Nam',
          role: 'moderator',
          status: 'active',
          created_at: '2026-02-15T00:00:00Z',
        },
        {
          id: 'u-3',
          email: 'minhphap@csac.local',
          full_name: 'Minh Pháp',
          role: 'member',
          status: 'active',
          created_at: '2026-03-01T00:00:00Z',
        },
      ];
    } finally {
      isLoading = false;
    }
  }

  $effect(() => {
    loadUsers();
  });

  let filteredUsers = $derived(
    users.filter((u) => {
      const matchSearch =
        u.email.toLowerCase().includes(search.toLowerCase()) ||
        u.full_name.toLowerCase().includes(search.toLowerCase());
      const matchRole = roleFilter === 'all' || u.role === roleFilter;
      return matchSearch && matchRole;
    })
  );

  async function handleCreateUser(e: Event) {
    e.preventDefault();
    try {
      await api.users.create({
        email: newEmail,
        full_name: newFullName,
        password: newPassword,
        role: newRole,
      });
      isCreateModalOpen = false;
      newEmail = '';
      newFullName = '';
      newPassword = '';
      newRole = 'member';
      successMessage = t('admin_users.success_create');
      await loadUsers();
    } catch (err: any) {
      errorMessage = err.message || t('admin_users.error_create');
    }
  }

  async function handleToggleStatus(user: UserAccount) {
    const newStatus = user.status === 'active' ? 'suspended' : 'active';
    try {
      await api.users.updateStatus(user.id, newStatus);
      successMessage = t('admin_users.success_status_updated');
      await loadUsers();
    } catch (err: any) {
      errorMessage = err.message || t('admin_users.error_status');
    }
  }

  function promptDowngradeAdmin(user: UserAccount) {
    selectedUserForDowngrade = user;
    downgradeReason = '';
    targetDowngradeRole = 'member';
    isDowngradeModalOpen = true;
  }

  async function handleSubmitDowngrade(e: Event) {
    e.preventDefault();
    if (!selectedUserForDowngrade) return;

    try {
      const res = await api.governance.proposeDemotion({
        target_user_id: selectedUserForDowngrade.id,
        target_role: targetDowngradeRole,
        reason: downgradeReason,
      });
      isDowngradeModalOpen = false;
      successMessage = t('admin_users.success_downgrade_initiated', { count: res.required_approvals });
      await loadUsers();
    } catch (err: any) {
      errorMessage = err.message || t('admin_users.error_proposal');
    }
  }

  const roleStyles = {
    admin: 'bg-destructive/10 text-destructive border-destructive/20',
    moderator: 'bg-primary/10 text-primary border-primary/20',
    member: 'bg-blue-500/10 text-blue-600 border-blue-500/20',
  };
</script>

<svelte:head>
  <title>{$tStore('admin_users.page_title')}</title>
</svelte:head>

<Navbar />

<div class="mx-auto flex max-w-7xl flex-col gap-6 p-6">
  <!-- Header Banner -->
  <Card class="flex flex-col gap-4 rounded-2xl border border-border bg-card p-6 shadow-sm md:flex-row md:items-center md:justify-between">
    <div class="flex items-center gap-4">
      <div class="flex h-12 w-12 items-center justify-center rounded-2xl bg-primary/10 text-primary">
        <Users class="w-6 h-6" />
      </div>
      <div>
        <h1 class="text-2xl font-extrabold tracking-tight text-foreground">
          {$tStore('admin_users.heading')}
        </h1>
        <p class="text-xs text-muted-foreground">
          {$tStore('admin_users.subheading')}
        </p>
      </div>
    </div>

    <div class="flex items-center gap-2.5">
      <Button href="/admin/approve" variant="outline" size="sm" class="gap-1.5">
        <ShieldAlert class="w-4 h-4 text-destructive" />
        <span>{$tStore('admin_users.btn_quorum')}</span>
      </Button>
      <Button variant="default" size="sm" onclick={() => (isCreateModalOpen = true)} class="font-bold gap-1.5">
        <UserPlus class="w-4 h-4" />
        <span>{$tStore('admin_users.btn_add_user')}</span>
      </Button>
    </div>
  </Card>

  {#if successMessage}
    <div class="flex items-center justify-between rounded-xl border border-emerald-500/20 bg-emerald-500/10 p-3.5 text-xs font-semibold text-emerald-600">
      <div class="flex items-center gap-2">
        <CircleCheck class="w-4 h-4" />
        <span>{successMessage}</span>
      </div>
      <button type="button" onclick={() => successMessage = null} class="text-emerald-600 hover:text-emerald-800">
        <X class="w-4 h-4" />
      </button>
    </div>
  {/if}

  {#if errorMessage}
    <div class="flex items-center justify-between rounded-xl border border-destructive/20 bg-destructive/10 p-3.5 text-xs font-semibold text-destructive">
      <div class="flex items-center gap-2">
        <CircleAlert class="w-4 h-4" />
        <span>{errorMessage}</span>
      </div>
      <button type="button" onclick={() => errorMessage = null} class="text-destructive hover:opacity-80">
        <X class="w-4 h-4" />
      </button>
    </div>
  {/if}

  <!-- Filter & Search Toolbar -->
  <Card class="flex flex-col gap-3 rounded-2xl border border-border bg-card p-4 shadow-sm md:flex-row md:items-center md:justify-between">
    <div class="relative w-full md:w-72">
      <Search class="pointer-events-none absolute left-2.5 top-2.5 w-3.5 h-3.5 text-muted-foreground" />
      <Input
        type="text"
        placeholder={$tStore('admin_users.search_placeholder')}
        bind:value={search}
        class="h-8 pl-8 text-xs"
      />
    </div>

    <div class="flex items-center gap-1.5 flex-wrap">
      <Button
        variant={roleFilter === 'all' ? 'default' : 'outline'}
        size="sm"
        onclick={() => roleFilter = 'all'}
      >
        {$tStore('admin_users.filter_all')}
      </Button>
      <Button
        variant={roleFilter === 'admin' ? 'default' : 'outline'}
        size="sm"
        onclick={() => roleFilter = 'admin'}
      >
        {$tStore('admin_users.filter_admins')}
      </Button>
      <Button
        variant={roleFilter === 'moderator' ? 'default' : 'outline'}
        size="sm"
        onclick={() => roleFilter = 'moderator'}
      >
        {$tStore('admin_users.filter_moderators')}
      </Button>
      <Button
        variant={roleFilter === 'member' ? 'default' : 'outline'}
        size="sm"
        onclick={() => roleFilter = 'member'}
      >
        {$tStore('admin_users.filter_members')}
      </Button>
    </div>
  </Card>

  <!-- Users Table Container -->
  <Card class="rounded-2xl border border-border bg-card shadow-sm overflow-hidden p-0">
    {#if isLoading}
      <div class="flex flex-col items-center justify-center p-12 text-muted-foreground">
        <LoaderCircle class="w-6 h-6 animate-spin text-primary mb-2" />
        <p class="text-xs">{$tStore('admin_users.loading')}</p>
      </div>
    {:else if filteredUsers.length === 0}
      <div class="p-12 text-center text-xs text-muted-foreground">
        {$tStore('admin_users.empty')}
      </div>
    {:else}
      <Table.Root>
        <Table.TableHeader>
          <Table.TableRow class="bg-muted/40">
            <Table.TableHead>{$tStore('admin_users.th_name')}</Table.TableHead>
            <Table.TableHead>{$tStore('admin_users.th_email')}</Table.TableHead>
            <Table.TableHead>{$tStore('admin_users.th_role')}</Table.TableHead>
            <Table.TableHead>{$tStore('admin_users.th_status')}</Table.TableHead>
            <Table.TableHead>{$tStore('admin_users.th_created')}</Table.TableHead>
            <Table.TableHead class="text-right">{$tStore('admin_users.th_actions')}</Table.TableHead>
          </Table.TableRow>
        </Table.TableHeader>
        <Table.TableBody>
          {#each filteredUsers as user (user.id)}
            <Table.TableRow>
              <Table.TableCell>
                <span class="text-xs font-bold text-foreground">{user.full_name}</span>
              </Table.TableCell>
              <Table.TableCell>
                <span class="text-xs text-muted-foreground">{user.email}</span>
              </Table.TableCell>
              <Table.TableCell>
                <span class="inline-flex items-center rounded px-2 py-0.5 text-[10px] font-bold border {roleStyles[user.role]}">
                  {user.role.toUpperCase()}
                </span>
              </Table.TableCell>
              <Table.TableCell>
                <Badge variant={user.status === 'active' ? 'outline' : 'destructive'} class="text-[10px] py-0 {user.status === 'active' ? 'bg-emerald-500/10 text-emerald-600 border-emerald-500/20' : ''}">
                  {user.status === 'active' ? $tStore('admin_users.status_active') : $tStore('admin_users.status_suspended')}
                </Badge>
              </Table.TableCell>
              <Table.TableCell>
                <span class="text-xs text-muted-foreground">{new Date(user.created_at).toLocaleDateString()}</span>
              </Table.TableCell>
              <Table.TableCell class="text-right">
                <DropdownMenu.Root>
                  <DropdownMenu.Trigger>
                    {#snippet child({ props })}
                      <Button {...props} variant="ghost" size="icon-sm">
                        <EllipsisVertical class="w-4 h-4" />
                      </Button>
                    {/snippet}
                  </DropdownMenu.Trigger>
                  <DropdownMenu.Content align="end">
                    <DropdownMenu.Item onclick={() => handleToggleStatus(user)}>
                      {#if user.status === 'active'}
                        <Lock class="w-3.5 h-3.5 mr-2 text-destructive" />
                        <span>{$tStore('admin_users.action_suspend')}</span>
                      {:else}
                        <LockOpen class="w-3.5 h-3.5 mr-2 text-emerald-600" />
                        <span>{$tStore('admin_users.action_activate')}</span>
                      {/if}
                    </DropdownMenu.Item>
                    {#if user.role === 'admin'}
                      <DropdownMenu.Separator />
                      <DropdownMenu.Item onclick={() => promptDowngradeAdmin(user)} class="text-destructive">
                        <ArrowDown class="w-3.5 h-3.5 mr-2" />
                        <span>{$tStore('admin_users.action_propose_demotion')}</span>
                      </DropdownMenu.Item>
                    {/if}
                  </DropdownMenu.Content>
                </DropdownMenu.Root>
              </Table.TableCell>
            </Table.TableRow>
          {/each}
        </Table.TableBody>
      </Table.Root>
    {/if}
  </Card>
</div>

<!-- Modal: Create New User -->
{#if isCreateModalOpen}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) isCreateModalOpen = false; }}>
    <Dialog.Content class="max-w-md">
      <Dialog.Header>
        <Dialog.Title class="text-base font-bold">
          {$tStore('admin_users.modal_add_title')}
        </Dialog.Title>
        <Dialog.Description class="text-xs text-muted-foreground">
          Create new credential account with assigned platform role.
        </Dialog.Description>
      </Dialog.Header>

      <form onsubmit={handleCreateUser} class="flex flex-col gap-3 py-2">
        <div class="flex flex-col gap-1.5">
          <Label for="fullname" class="text-xs font-semibold">
            {$tStore('admin_users.form_fullname')} *
          </Label>
          <Input
            id="fullname"
            type="text"
            placeholder="e.g. Minh Pháp"
            bind:value={newFullName}
            required
            class="h-8 text-xs"
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="email" class="text-xs font-semibold">
            {$tStore('admin_users.form_email')} *
          </Label>
          <Input
            id="email"
            type="email"
            placeholder="minhphap@csac.local"
            bind:value={newEmail}
            required
            class="h-8 text-xs"
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="password" class="text-xs font-semibold">
            {$tStore('admin_users.form_password')} *
          </Label>
          <Input
            id="password"
            type="password"
            placeholder="••••••••"
            bind:value={newPassword}
            required
            minlength={8}
            class="h-8 text-xs"
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="role" class="text-xs font-semibold">
            {$tStore('admin_users.form_role')}
          </Label>
          <select
            id="role"
            bind:value={newRole}
            class="h-8 rounded-md border border-input bg-background px-2.5 text-xs font-medium text-foreground outline-none"
          >
            <option value="member">Member</option>
            <option value="moderator">Moderator</option>
            <option value="admin">Administrator</option>
          </select>
        </div>

        <Dialog.Footer class="pt-3">
          <Button
            type="button"
            variant="outline"
            size="sm"
            onclick={() => (isCreateModalOpen = false)}
          >
            {$tStore('admin_users.btn_cancel')}
          </Button>
          <Button type="submit" variant="default" size="sm">
            {$tStore('admin_users.btn_submit')}
          </Button>
        </Dialog.Footer>
      </form>
    </Dialog.Content>
  </Dialog.Root>
{/if}

<!-- Modal: Propose Admin Demotion -->
{#if isDowngradeModalOpen && selectedUserForDowngrade}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) isDowngradeModalOpen = false; }}>
    <Dialog.Content class="max-w-md">
      <Dialog.Header>
        <div class="flex items-center gap-2">
          <ShieldAlert class="w-5 h-5 text-destructive" />
          <Dialog.Title class="text-base font-bold">
            {$tStore('admin_users.modal_demote_title')}
          </Dialog.Title>
        </div>
        <Dialog.Description class="text-xs text-muted-foreground">
          {$tStore('admin_users.modal_demote_desc', { name: selectedUserForDowngrade.full_name })}
        </Dialog.Description>
      </Dialog.Header>

      <form onsubmit={handleSubmitDowngrade} class="flex flex-col gap-3 py-2">
        <div class="flex flex-col gap-1.5">
          <Label for="target-role" class="text-xs font-semibold">
            Target Downgraded Role
          </Label>
          <select
            id="target-role"
            bind:value={targetDowngradeRole}
            class="h-8 rounded-md border border-input bg-background px-2.5 text-xs font-medium text-foreground outline-none"
          >
            <option value="moderator">Moderator</option>
            <option value="member">Member</option>
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="reason" class="text-xs font-semibold">
            {$tStore('admin_users.form_reason')} *
          </Label>
          <textarea
            id="reason"
            bind:value={downgradeReason}
            required
            rows="3"
            placeholder={$tStore('admin_users.reason_placeholder')}
            class="w-full rounded-md border border-input bg-background p-2 text-xs font-medium text-foreground outline-none"
          ></textarea>
        </div>

        <Dialog.Footer class="pt-3">
          <Button
            type="button"
            variant="outline"
            size="sm"
            onclick={() => (isDowngradeModalOpen = false)}
          >
            {$tStore('admin_users.btn_cancel')}
          </Button>
          <Button type="submit" variant="destructive" size="sm">
            {$tStore('admin_users.btn_propose_demotion')}
          </Button>
        </Dialog.Footer>
      </form>
    </Dialog.Content>
  </Dialog.Root>
{/if}
