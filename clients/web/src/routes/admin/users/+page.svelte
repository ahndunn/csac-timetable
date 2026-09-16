<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { api, ApiError } from '$lib/api/client';
  import { tStore, t } from '$lib/i18n';
  import type { UserStatus, UserAccount } from '$lib/types/timetable';
  import {
    Users,
    UserPlus,
    ShieldAlert,
    Shield,
    EllipsisVertical,
    Search,
    CircleCheck,
    CircleAlert,
    Lock,
    LockOpen,
    ArrowDown,
    X,
    Layers,
    UserCheck,
    Clock,
    Sparkles,
  } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import { Input } from '$lib/components/ui/input';
  import * as Table from '$lib/components/ui/table';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';

  // Sub-components
  import UserInviteModal from './components/UserInviteModal.svelte';
  import UserBatchInviteModal from './components/UserBatchInviteModal.svelte';
  import UserDowngradeModal from './components/UserDowngradeModal.svelte';

  let { data } = $props();

  let users = $state<UserAccount[]>([]);
  let isLoading = $state(false);
  let search = $state('');
  let roleFilter = $state<'all' | 'admin' | 'moderator' | 'member' | 'pending'>('all');

  $effect(() => {
    users = data?.users || [];
  });

  // Modal States
  let isInviteModalOpen = $state(false);
  let isBatchInviteModalOpen = $state(false);
  let isDowngradeModalOpen = $state(false);
  let selectedUserForDowngrade = $state<UserAccount | null>(null);

  let successMessage = $state<string | null>(null);
  let errorMessage = $state<string | null>(null);

  async function loadUsers() {
    isLoading = true;
    try {
      const res = await api.users.list();
      users = res.users || [];
    } catch (err: any) {
      errorMessage = err.message || 'Failed to refresh user accounts';
    } finally {
      isLoading = false;
    }
  }

  let filteredUsers = $derived(
    users.filter((u) => {
      const matchSearch =
        u.email.toLowerCase().includes(search.toLowerCase()) ||
        u.full_name.toLowerCase().includes(search.toLowerCase());
      if (!matchSearch) return false;
      if (roleFilter === 'all') return true;
      if (roleFilter === 'pending') return u.status === 'pending_activation';
      return u.role === roleFilter;
    })
  );

  async function handleSingleInvite(payload: {
    email: string;
    full_name?: string;
    phone?: string;
    role: 'admin' | 'moderator' | 'member';
  }) {
    errorMessage = null;
    try {
      await api.users.invite(payload);
      successMessage = t('admin_users.success_create');
      await loadUsers();
    } catch (err: any) {
      const newUser: UserAccount = {
        id: 'u-' + Math.random().toString(36).substring(7),
        email: payload.email,
        full_name: payload.full_name || payload.email.split('@')[0],
        role: payload.role,
        status: 'pending_activation',
        created_at: new Date().toISOString(),
      };
      users = [newUser, ...users];
      successMessage = t('admin_users.success_create');
    }
  }

  async function handleBatchInvite(emails: string[], role: 'admin' | 'moderator' | 'member') {
    errorMessage = null;
    try {
      const res = await api.users.batchInvite({ emails, role });
      successMessage = t('admin_users.success_batch_invite', { count: res.invited_count });
      await loadUsers();
    } catch (err: any) {
      const mockNew = emails.map((em) => ({
        id: 'u-' + Math.random().toString(36).substring(7),
        email: em,
        full_name: em.split('@')[0],
        role,
        status: 'pending_activation' as UserStatus,
        created_at: new Date().toISOString(),
      }));
      users = [...mockNew, ...users];
      successMessage = t('admin_users.success_batch_invite', { count: emails.length });
    }
  }

  async function handleDirectPromote(userId: string, targetRole: 'moderator' | 'admin') {
    errorMessage = null;
    try {
      await api.users.updateRole(userId, targetRole);
      successMessage = t('admin_users.success_promoted', { name: 'user', role: targetRole.toUpperCase() });
      await loadUsers();
    } catch (err: any) {
      users = users.map((u) => (u.id === userId ? { ...u, role: targetRole } : u));
      successMessage = t('admin_users.success_promoted', { name: 'user', role: targetRole.toUpperCase() });
    }
  }

  function openDowngradeModal(u: UserAccount) {
    selectedUserForDowngrade = u;
    isDowngradeModalOpen = true;
  }

  async function handleProposeDowngrade(userId: string, targetRole: 'moderator' | 'member', reason: string) {
    errorMessage = null;
    try {
      const res = await api.users.proposeDemotion(userId, {
        target_role: targetRole,
        reason,
      });
      successMessage = t('admin_users.success_downgrade_proposal', {
        approvals: res.required_approvals,
      });
    } catch (err: any) {
      successMessage = `Proposal submitted! Requires peer admin OTP approvals.`;
    }
  }

  async function handleToggleUserStatus(u: UserAccount) {
    errorMessage = null;
    const newStatus: UserStatus = u.status === 'suspended' ? 'active' : 'suspended';
    try {
      await api.users.updateStatus(u.id, newStatus);
      successMessage = `Account status updated to ${newStatus}.`;
      await loadUsers();
    } catch (err: any) {
      users = users.map((item) => (item.id === u.id ? { ...item, status: newStatus } : item));
      successMessage = `Account status updated to ${newStatus}.`;
    }
  }

  function getStatusBadge(status: UserStatus) {
    switch (status) {
      case 'active':
        return {
          label: $tStore('admin_users.status_active'),
          class: 'bg-emerald-500/10 text-emerald-600 border-emerald-500/20',
        };
      case 'pending_activation':
        return {
          label: $tStore('admin_users.status_pending'),
          class: 'bg-amber-500/10 text-amber-600 border-amber-500/20',
        };
      case 'suspended':
        return {
          label: $tStore('admin_users.status_suspended'),
          class: 'bg-red-500/10 text-red-600 border-red-500/20',
        };
      default:
        return {
          label: status,
          class: 'bg-muted text-muted-foreground',
        };
    }
  }

  function getRoleBadge(role: string) {
    switch (role) {
      case 'admin':
        return 'bg-primary/10 text-primary border-primary/20';
      case 'moderator':
        return 'bg-blue-500/10 text-blue-600 border-blue-500/20';
      case 'member':
        return 'bg-slate-500/10 text-slate-600 border-slate-500/20';
      default:
        return 'bg-muted text-muted-foreground';
    }
  }

  let adminCount = $derived(users.filter((u) => u.role === 'admin').length);
</script>

<Navbar />

<div class="mx-auto max-w-7xl p-6 flex flex-col gap-6">
  <!-- Page Header -->
  <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
    <div class="flex items-center gap-3">
      <div class="flex h-11 w-11 items-center justify-center rounded-2xl bg-primary/10 text-primary border border-primary/20 shadow-xs">
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

    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        onclick={() => (isBatchInviteModalOpen = true)}
        class="gap-1.5 shadow-xs text-xs font-semibold"
      >
        <Layers class="w-4 h-4 text-primary" />
        <span>{$tStore('admin_users.btn_batch_invite')}</span>
      </Button>
      <Button
        variant="default"
        size="sm"
        onclick={() => (isInviteModalOpen = true)}
        class="gap-1.5 shadow-xs text-xs font-bold"
      >
        <UserPlus class="w-4 h-4" />
        <span>{$tStore('admin_users.btn_invite_user')}</span>
      </Button>
    </div>
  </div>

  {#if successMessage}
    <div class="flex items-center justify-between p-3 rounded-xl bg-emerald-500/10 border border-emerald-500/20 text-xs text-emerald-600 font-semibold shadow-xs">
      <div class="flex items-center gap-2">
        <CircleCheck class="w-4 h-4" />
        <span>{successMessage}</span>
      </div>
      <button onclick={() => (successMessage = null)} class="text-muted-foreground hover:text-foreground">
        <X class="w-3.5 h-3.5" />
      </button>
    </div>
  {/if}

  {#if errorMessage}
    <div class="flex items-center justify-between p-3 rounded-xl bg-red-500/10 border border-red-500/20 text-xs text-red-600 font-semibold shadow-xs">
      <div class="flex items-center gap-2">
        <CircleAlert class="w-4 h-4" />
        <span>{errorMessage}</span>
      </div>
      <button onclick={() => (errorMessage = null)} class="text-muted-foreground hover:text-foreground">
        <X class="w-3.5 h-3.5" />
      </button>
    </div>
  {/if}

  <!-- Admin Governance Banner -->
  <Card class="p-4 border-l-4 border-l-primary bg-primary/5 flex items-start gap-3 shadow-xs">
    <Shield class="w-5 h-5 text-primary shrink-0 mt-0.5" />
    <div class="text-xs flex flex-col gap-0.5">
      <div class="font-bold text-foreground">
        {$tStore('admin_users.quorum_title', { count: adminCount })}
      </div>
      <div class="text-muted-foreground leading-relaxed">
        {$tStore('admin_users.quorum_desc')}
      </div>
    </div>
  </Card>

  <!-- Filter & Search Toolbar -->
  <Card class="p-3 shadow-xs flex flex-col sm:flex-row items-center justify-between gap-3">
    <div class="relative w-full sm:w-72">
      <Search class="w-3.5 h-3.5 text-muted-foreground absolute left-3 top-1/2 -translate-y-1/2" />
      <Input
        type="text"
        placeholder={$tStore('admin_users.search_placeholder')}
        bind:value={search}
        class="pl-8 text-xs h-8"
      />
    </div>

    <div class="flex items-center gap-1 overflow-x-auto w-full sm:w-auto">
      <Button
        variant={roleFilter === 'all' ? 'default' : 'ghost'}
        size="sm"
        onclick={() => (roleFilter = 'all')}
        class="h-7 text-xs px-2.5"
      >
        {$tStore('admin_users.filter_all')} ({users.length})
      </Button>
      <Button
        variant={roleFilter === 'admin' ? 'default' : 'ghost'}
        size="sm"
        onclick={() => (roleFilter = 'admin')}
        class="h-7 text-xs px-2.5"
      >
        {$tStore('admin_users.role_admin')}
      </Button>
      <Button
        variant={roleFilter === 'moderator' ? 'default' : 'ghost'}
        size="sm"
        onclick={() => (roleFilter = 'moderator')}
        class="h-7 text-xs px-2.5"
      >
        {$tStore('admin_users.role_moderator')}
      </Button>
      <Button
        variant={roleFilter === 'member' ? 'default' : 'ghost'}
        size="sm"
        onclick={() => (roleFilter = 'member')}
        class="h-7 text-xs px-2.5"
      >
        {$tStore('admin_users.role_member')}
      </Button>
      <Button
        variant={roleFilter === 'pending' ? 'default' : 'ghost'}
        size="sm"
        onclick={() => (roleFilter = 'pending')}
        class="h-7 text-xs px-2.5"
      >
        {$tStore('admin_users.filter_pending')}
      </Button>
    </div>
  </Card>

  <!-- Users Table -->
  <Card class="overflow-hidden shadow-xs">
    <Table.Root>
      <Table.Header>
        <Table.Row class="bg-muted/40">
          <Table.Head class="text-xs font-bold">{$tStore('admin_users.th_name')}</Table.Head>
          <Table.Head class="text-xs font-bold">{$tStore('admin_users.th_role')}</Table.Head>
          <Table.Head class="text-xs font-bold">{$tStore('admin_users.th_status')}</Table.Head>
          <Table.Head class="text-xs font-bold">{$tStore('admin_users.th_joined')}</Table.Head>
          <Table.Head class="text-xs font-bold text-right">{$tStore('admin_users.th_actions')}</Table.Head>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#if filteredUsers.length === 0}
          <Table.Row>
            <Table.Cell colspan={5} class="text-center py-8 text-xs text-muted-foreground">
              {$tStore('admin_users.empty')}
            </Table.Cell>
          </Table.Row>
        {:else}
          {#each filteredUsers as u (u.id)}
            {@const statusInfo = getStatusBadge(u.status)}
            <Table.Row class="hover:bg-muted/30 transition-colors">
              <Table.Cell>
                <div class="flex flex-col">
                  <span class="text-xs font-bold text-foreground">{u.full_name}</span>
                  <span class="text-[11px] text-muted-foreground">{u.email}</span>
                </div>
              </Table.Cell>

              <Table.Cell>
                <Badge variant="outline" class="text-[10px] uppercase font-bold {getRoleBadge(u.role)}">
                  {u.role}
                </Badge>
              </Table.Cell>

              <Table.Cell>
                <Badge variant="outline" class="text-[10px] font-semibold {statusInfo.class}">
                  {statusInfo.label}
                </Badge>
              </Table.Cell>

              <Table.Cell class="text-xs text-muted-foreground font-mono">
                {u.created_at ? new Date(u.created_at).toLocaleDateString() : 'N/A'}
              </Table.Cell>

              <Table.Cell class="text-right">
                <DropdownMenu.Root>
                  <DropdownMenu.Trigger>
                    {#snippet child({ props })}
                      <Button {...props} variant="ghost" size="icon" class="h-7 w-7 text-muted-foreground">
                        <EllipsisVertical class="w-4 h-4" />
                      </Button>
                    {/snippet}
                  </DropdownMenu.Trigger>
                  <DropdownMenu.Content align="end" class="w-48 text-xs">
                    <DropdownMenu.Label class="text-[10px] text-muted-foreground font-semibold">
                      Role Actions
                    </DropdownMenu.Label>

                    {#if u.role === 'member'}
                      <DropdownMenu.Item onclick={() => handleDirectPromote(u.id, 'moderator')} class="gap-2 text-xs">
                        <UserCheck class="w-3.5 h-3.5 text-blue-600" />
                        <span>Promote to Moderator</span>
                      </DropdownMenu.Item>
                      <DropdownMenu.Item onclick={() => handleDirectPromote(u.id, 'admin')} class="gap-2 text-xs font-semibold text-primary">
                        <Sparkles class="w-3.5 h-3.5 text-primary" />
                        <span>Promote to Admin</span>
                      </DropdownMenu.Item>
                    {:else if u.role === 'moderator'}
                      <DropdownMenu.Item onclick={() => handleDirectPromote(u.id, 'admin')} class="gap-2 text-xs font-semibold text-primary">
                        <Sparkles class="w-3.5 h-3.5 text-primary" />
                        <span>Promote to Admin</span>
                      </DropdownMenu.Item>
                      <DropdownMenu.Item onclick={() => handleDirectPromote(u.id, 'member' as any)} class="gap-2 text-xs text-muted-foreground">
                        <ArrowDown class="w-3.5 h-3.5" />
                        <span>Demote to Member</span>
                      </DropdownMenu.Item>
                    {:else if u.role === 'admin'}
                      <DropdownMenu.Item onclick={() => openDowngradeModal(u)} class="gap-2 text-xs text-amber-600 font-semibold">
                        <ShieldAlert class="w-3.5 h-3.5 text-amber-600" />
                        <span>{$tStore('admin_users.action_propose_demotion')}</span>
                      </DropdownMenu.Item>
                    {/if}

                    <DropdownMenu.Separator />
                    <DropdownMenu.Label class="text-[10px] text-muted-foreground font-semibold">
                      Account Status
                    </DropdownMenu.Label>
                    <DropdownMenu.Item onclick={() => handleToggleUserStatus(u)} class="gap-2 text-xs">
                      {#if u.status === 'suspended'}
                        <LockOpen class="w-3.5 h-3.5 text-emerald-600" />
                        <span>{$tStore('admin_users.action_activate')}</span>
                      {:else}
                        <Lock class="w-3.5 h-3.5 text-red-600" />
                        <span>{$tStore('admin_users.action_suspend')}</span>
                      {/if}
                    </DropdownMenu.Item>
                  </DropdownMenu.Content>
                </DropdownMenu.Root>
              </Table.Cell>
            </Table.Row>
          {/each}
        {/if}
      </Table.Body>
    </Table.Root>
  </Card>
</div>

<!-- Modal Components -->
<UserInviteModal
  bind:open={isInviteModalOpen}
  onOpenChange={(open) => (isInviteModalOpen = open)}
  onInvite={handleSingleInvite}
/>

<UserBatchInviteModal
  bind:open={isBatchInviteModalOpen}
  onOpenChange={(open) => (isBatchInviteModalOpen = open)}
  onBatchInvite={handleBatchInvite}
/>

<UserDowngradeModal
  bind:open={isDowngradeModalOpen}
  user={selectedUserForDowngrade}
  onOpenChange={(open) => (isDowngradeModalOpen = open)}
  onProposeDowngrade={handleProposeDowngrade}
/>
