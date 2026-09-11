<script lang="ts">
  import { onMount } from 'svelte';
  import { api, ApiError } from '$lib/api/client';
  import { auth } from '$lib/stores/auth.svelte';
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore, t } from '$lib/i18n';
  import {
    Users,
    UserPlus,
    Shield,
    ShieldAlert,
    ShieldCheck,
    Search,
    AlertCircle,
    CheckCircle2,
    X,
    Key,
    Mail,
  } from '@lucide/svelte';

  interface UserItem {
    id: string;
    email: string;
    full_name: string;
    role: 'admin' | 'moderator' | 'member';
    status: 'active' | 'suspended';
    created_at: string;
  }

  let users = $state<UserItem[]>([]);
  let searchQuery = $state('');
  let roleFilter = $state<string>('all');
  let isLoading = $state(true);
  let errorMessage = $state<string | null>(null);
  let successMessage = $state<string | null>(null);

  // Modals
  let isCreateModalOpen = $state(false);
  let newUserEmail = $state('');
  let newUserName = $state('');
  let newUserRole = $state<'admin' | 'moderator' | 'member'>('member');
  let createdCredentials = $state<{ user: UserItem; initial_password?: string } | null>(null);

  let isDowngradeModalOpen = $state(false);
  let downgradeTarget = $state<UserItem | null>(null);
  let downgradeTargetRole = $state<'moderator' | 'member'>('moderator');
  let downgradeReason = $state('');

  async function loadUsers() {
    isLoading = true;
    errorMessage = null;
    try {
      users = await api.admin.listUsers();
    } catch (err: any) {
      errorMessage = err.message || t('admin_users.error_load');
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    loadUsers();
  });

  const filteredUsers = $derived(
    users.filter((u) => {
      const matchSearch =
        u.email.toLowerCase().includes(searchQuery.toLowerCase()) ||
        u.full_name.toLowerCase().includes(searchQuery.toLowerCase());
      const matchRole = roleFilter === 'all' || u.role === roleFilter;
      return matchSearch && matchRole;
    })
  );

  async function handleCreateUser(e: SubmitEvent) {
    e.preventDefault();
    errorMessage = null;
    try {
      const res = await api.admin.createUser({
        email: newUserEmail,
        full_name: newUserName,
        role: newUserRole,
      });
      createdCredentials = res;
      successMessage = t('admin_users.success_created', { email: newUserEmail });
      newUserEmail = '';
      newUserName = '';
      newUserRole = 'member';
      await loadUsers();
    } catch (err: any) {
      errorMessage = err.message || t('admin_users.error_create');
    }
  }

  async function handleRoleChange(user: UserItem, newRole: 'admin' | 'moderator' | 'member') {
    errorMessage = null;
    if (user.role === 'admin' && newRole !== 'admin') {
      downgradeTarget = user;
      downgradeTargetRole = newRole;
      downgradeReason = '';
      isDowngradeModalOpen = true;
      return;
    }

    try {
      await api.admin.updateRole(user.id, newRole);
      successMessage = t('admin_users.success_promoted', { name: user.full_name, role: newRole.toUpperCase() });
      await loadUsers();
    } catch (err: any) {
      errorMessage = err.message || t('admin_users.error_update_role');
    }
  }

  async function handleSubmitDowngradeProposal(e: SubmitEvent) {
    e.preventDefault();
    if (!downgradeTarget) return;

    try {
      const res = await api.admin.createDowngradeProposal(downgradeTarget.id, {
        target_role: downgradeTargetRole,
        reason: downgradeReason,
      });
      isDowngradeModalOpen = false;
      successMessage = t('admin_users.success_downgrade_initiated', { count: res.required_approvals });
      await loadUsers();
    } catch (err: any) {
      errorMessage = err.message || t('admin_users.error_proposal');
    }
  }
</script>

<svelte:head>
  <title>{$tStore('admin_users.page_title')}</title>
</svelte:head>

<Navbar />

<div class="admin-container">
  <div class="header-card">
    <div class="header-main">
      <div class="icon-box">
        <Users size={28} class="text-orange" />
      </div>
      <div>
        <h1 class="page-title">{$tStore('admin_users.heading')}</h1>
        <p class="page-desc">{$tStore('admin_users.subheading')}</p>
      </div>
    </div>

    <div class="header-actions">
      <a href="/admin/approve" class="quorum-link">
        <ShieldAlert size={18} />
        <span>{$tStore('admin_users.btn_quorum')}</span>
      </a>
      <button class="primary-btn" onclick={() => isCreateModalOpen = true}>
        <UserPlus size={18} />
        <span>{$tStore('admin_users.btn_add_user')}</span>
      </button>
    </div>
  </div>

  {#if successMessage}
    <div class="alert-box alert-success">
      <CheckCircle2 size={18} />
      <span>{successMessage}</span>
      <button class="alert-close" onclick={() => successMessage = null}><X size={16} /></button>
    </div>
  {/if}

  {#if errorMessage}
    <div class="alert-box alert-error">
      <AlertCircle size={18} />
      <span>{errorMessage}</span>
      <button class="alert-close" onclick={() => errorMessage = null}><X size={16} /></button>
    </div>
  {/if}

  <div class="table-card">
    <div class="filter-bar">
      <div class="search-input-wrap">
        <Search size={18} class="search-icon" />
        <input
          type="text"
          placeholder={$tStore('admin_users.search_placeholder')}
          bind:value={searchQuery}
        />
      </div>

      <div class="role-pills">
        <button class="pill {roleFilter === 'all' ? 'active' : ''}" onclick={() => roleFilter = 'all'}>{$tStore('admin_users.filter_all', { count: users.length })}</button>
        <button class="pill {roleFilter === 'admin' ? 'active' : ''}" onclick={() => roleFilter = 'admin'}>{$tStore('admin_users.filter_admins')}</button>
        <button class="pill {roleFilter === 'moderator' ? 'active' : ''}" onclick={() => roleFilter = 'moderator'}>{$tStore('admin_users.filter_moderators')}</button>
        <button class="pill {roleFilter === 'member' ? 'active' : ''}" onclick={() => roleFilter = 'member'}>{$tStore('admin_users.filter_members')}</button>
      </div>
    </div>

    {#if isLoading}
      <div class="loading-wrap">
        <div class="spinner"></div>
        <p>{$tStore('admin_users.loading')}</p>
      </div>
    {:else if filteredUsers.length === 0}
      <div class="empty-wrap">
        <p>{$tStore('admin_users.empty')}</p>
      </div>
    {:else}
      <div class="table-responsive">
        <table class="user-table">
          <thead>
            <tr>
              <th>{$tStore('admin_users.th_name')}</th>
              <th>{$tStore('admin_users.th_email')}</th>
              <th>{$tStore('admin_users.th_role')}</th>
              <th>{$tStore('admin_users.th_status')}</th>
              <th>{$tStore('admin_users.th_joined')}</th>
              <th style="text-align: right;">{$tStore('admin_users.th_governance')}</th>
            </tr>
          </thead>
          <tbody>
            {#each filteredUsers as user (user.id)}
              <tr>
                <td class="name-cell">
                  <strong>{user.full_name}</strong>
                </td>
                <td class="email-cell">{user.email}</td>
                <td>
                  <span class="role-badge role-{user.role}">
                    {#if user.role === 'admin'}
                      <ShieldCheck size={14} />
                    {:else if user.role === 'moderator'}
                      <Shield size={14} />
                    {/if}
                    {user.role === 'admin' ? $tStore('admin_users.role_admin') : user.role === 'moderator' ? $tStore('admin_users.role_moderator') : $tStore('admin_users.role_member')}
                  </span>
                </td>
                <td>
                  <span class="status-badge status-{user.status}">
                    {user.status === 'active' ? $tStore('admin_users.status_active') : $tStore('admin_users.status_suspended')}
                  </span>
                </td>
                <td class="date-cell">
                  {new Date(user.created_at).toLocaleDateString()}
                </td>
                <td style="text-align: right;">
                  <select
                    class="role-select"
                    value={user.role}
                    onchange={(e) => handleRoleChange(user, (e.target as HTMLSelectElement).value as any)}
                  >
                    <option value="member">{$tStore('admin_users.role_member')}</option>
                    <option value="moderator">{$tStore('admin_users.role_moderator')}</option>
                    <option value="admin">{$tStore('admin_users.role_admin')}</option>
                  </select>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
</div>

<!-- Modal: Create User -->
{#if isCreateModalOpen}
  <div class="modal-overlay">
    <div class="modal-card">
      <div class="modal-header">
        <div class="modal-title-wrap">
          <UserPlus size={22} class="text-orange" />
          <h2>{$tStore('admin_users.modal_create_title')}</h2>
        </div>
        <button class="close-btn" onclick={() => { isCreateModalOpen = false; createdCredentials = null; }}><X size={20} /></button>
      </div>

      {#if createdCredentials}
        <div class="credentials-box">
          <div class="cred-title">
            <Key size={18} class="text-orange" />
            <h3>{$tStore('admin_users.modal_cred_success')}</h3>
          </div>
          <p class="cred-desc">{$tStore('admin_users.modal_cred_desc')}</p>
          <div class="cred-info">
            <div><strong>{$tStore('admin_users.modal_cred_email')}</strong> {createdCredentials.user.email}</div>
            <div><strong>{$tStore('admin_users.modal_cred_password')}</strong> <code class="cred-pwd">{createdCredentials.initial_password}</code></div>
          </div>
          <button class="primary-btn" style="width: 100%; margin-top: 1rem;" onclick={() => { isCreateModalOpen = false; createdCredentials = null; }}>
            {$tStore('admin_users.modal_cred_done')}
          </button>
        </div>
      {:else}
        <form onsubmit={handleCreateUser} class="modal-form">
          <div class="form-group">
            <label for="name">{$tStore('admin_users.modal_name_label')}</label>
            <input id="name" type="text" placeholder={$tStore('admin_users.modal_name_placeholder')} bind:value={newUserName} required />
          </div>

          <div class="form-group">
            <label for="email">{$tStore('admin_users.modal_email_label')}</label>
            <input id="email" type="email" placeholder={$tStore('admin_users.modal_email_placeholder')} bind:value={newUserEmail} required />
          </div>

          <div class="form-group">
            <label for="role">{$tStore('admin_users.modal_role_label')}</label>
            <select id="role" bind:value={newUserRole}>
              <option value="member">{$tStore('admin_users.modal_role_desc_member')}</option>
              <option value="moderator">{$tStore('admin_users.modal_role_desc_mod')}</option>
              <option value="admin">{$tStore('admin_users.modal_role_desc_admin')}</option>
            </select>
          </div>

          <div class="modal-footer">
            <button type="button" class="btn-cancel" onclick={() => isCreateModalOpen = false}>{$tStore('admin_users.modal_btn_cancel')}</button>
            <button type="submit" class="primary-btn">{$tStore('admin_users.modal_btn_submit')}</button>
          </div>
        </form>
      {/if}
    </div>
  </div>
{/if}

<!-- Modal: Admin Downgrade Proposal -->
{#if isDowngradeModalOpen && downgradeTarget}
  <div class="modal-overlay">
    <div class="modal-card">
      <div class="modal-header">
        <div class="modal-title-wrap">
          <ShieldAlert size={22} class="text-warning" />
          <h2>{$tStore('admin_users.modal_downgrade_title')}</h2>
        </div>
        <button class="close-btn" onclick={() => isDowngradeModalOpen = false}><X size={20} /></button>
      </div>

      <div class="warning-callout">
        {$tStore('admin_users.modal_downgrade_callout')}
      </div>

      <form onsubmit={handleSubmitDowngradeProposal} class="modal-form">
        <div class="form-group">
          <label>{$tStore('admin_users.modal_downgrade_target')}</label>
          <input type="text" value="{downgradeTarget.full_name} ({downgradeTarget.email})" disabled />
        </div>

        <div class="form-group">
          <label for="target-role">{$tStore('admin_users.modal_downgrade_role')}</label>
          <select id="target-role" bind:value={downgradeTargetRole}>
            <option value="moderator">{$tStore('admin_users.role_moderator')}</option>
            <option value="member">{$tStore('admin_users.role_member')}</option>
          </select>
        </div>

        <div class="form-group">
          <label for="reason">{$tStore('admin_users.modal_downgrade_reason')}</label>
          <textarea id="reason" rows="3" placeholder={$tStore('admin_users.modal_downgrade_reason_placeholder')} bind:value={downgradeReason} required></textarea>
        </div>

        <div class="modal-footer">
          <button type="button" class="btn-cancel" onclick={() => isDowngradeModalOpen = false}>{$tStore('admin_users.modal_btn_cancel')}</button>
          <button type="submit" class="danger-btn">{$tStore('admin_users.modal_downgrade_submit')}</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .admin-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .header-card {
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 20px;
    padding: 1.75rem 2rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.03);
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 1.25rem;
  }

  .header-main {
    display: flex;
    align-items: center;
    gap: 1.25rem;
  }

  .icon-box {
    width: 54px;
    height: 54px;
    border-radius: 16px;
    background: rgba(255, 107, 0, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .text-orange { color: #ff6b00; }
  .text-warning { color: #f59e0b; }

  .page-title {
    font-size: 1.45rem;
    font-weight: 700;
    color: #0f172a;
    margin: 0 0 0.25rem 0;
  }

  .page-desc {
    font-size: 0.9rem;
    color: #64748b;
    margin: 0;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .primary-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: #ff6b00;
    color: #ffffff;
    padding: 0.65rem 1.25rem;
    border-radius: 12px;
    font-weight: 600;
    font-size: 0.9rem;
    border: none;
    cursor: pointer;
    transition: all 0.2s;
  }

  .primary-btn:hover {
    background: #e65c00;
  }

  .danger-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: #ef4444;
    color: #ffffff;
    padding: 0.65rem 1.25rem;
    border-radius: 12px;
    font-weight: 600;
    border: none;
    cursor: pointer;
  }

  .quorum-link {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    color: #334155;
    padding: 0.65rem 1.15rem;
    border-radius: 12px;
    font-weight: 600;
    font-size: 0.9rem;
    text-decoration: none;
    transition: all 0.2s;
  }

  .quorum-link:hover {
    background: #f1f5f9;
    border-color: #cbd5e1;
  }

  .alert-box {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.85rem 1.25rem;
    border-radius: 12px;
    font-size: 0.9rem;
  }

  .alert-success { background: #f0fdf4; border: 1px solid #bbf7d0; color: #16a34a; }
  .alert-error { background: #fef2f2; border: 1px solid #fee2e2; color: #ef4444; }

  .alert-close {
    margin-left: auto;
    background: none;
    border: none;
    cursor: pointer;
    color: inherit;
  }

  .table-card {
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 20px;
    padding: 1.5rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.03);
  }

  .filter-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 1.25rem;
  }

  .search-input-wrap {
    position: relative;
    display: flex;
    align-items: center;
    flex: 1;
    min-width: 260px;
    max-width: 400px;
  }

  :global(.search-icon) {
    position: absolute;
    left: 0.85rem;
    color: #94a3b8;
  }

  .search-input-wrap input {
    width: 100%;
    padding: 0.65rem 1rem 0.65rem 2.4rem;
    border: 1px solid #e2e8f0;
    border-radius: 12px;
    background: #f8fafc;
    font-size: 0.9rem;
  }

  .role-pills {
    display: flex;
    gap: 0.4rem;
  }

  .pill {
    padding: 0.45rem 0.85rem;
    border-radius: 10px;
    border: 1px solid #e2e8f0;
    background: #f8fafc;
    color: #475569;
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
  }

  .pill.active {
    background: #ff6b00;
    color: #ffffff;
    border-color: #ff6b00;
  }

  .table-responsive {
    overflow-x: auto;
  }

  .user-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.9rem;
  }

  .user-table th {
    text-align: left;
    padding: 0.85rem 1rem;
    color: #64748b;
    font-weight: 600;
    border-bottom: 1px solid #e2e8f0;
    font-size: 0.8rem;
    text-transform: uppercase;
  }

  .user-table td {
    padding: 0.95rem 1rem;
    border-bottom: 1px solid #f1f5f9;
    color: #334155;
  }

  .role-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.75rem;
    font-weight: 700;
    padding: 0.25rem 0.65rem;
    border-radius: 8px;
  }

  .role-admin { background: #faf5ff; color: #9333ea; border: 1px solid #f3e8ff; }
  .role-moderator { background: #eff6ff; color: #2563eb; border: 1px solid #dbeafe; }
  .role-member { background: #f8fafc; color: #64748b; border: 1px solid #e2e8f0; }

  .status-badge {
    font-size: 0.75rem;
    font-weight: 600;
    padding: 0.2rem 0.5rem;
    border-radius: 6px;
  }

  .status-active { background: #f0fdf4; color: #16a34a; }
  .status-suspended { background: #fef2f2; color: #ef4444; }

  .role-select {
    padding: 0.4rem 0.75rem;
    border-radius: 8px;
    border: 1px solid #e2e8f0;
    background: #f8fafc;
    font-size: 0.85rem;
    color: #1e293b;
  }

  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(15, 23, 42, 0.4);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
    padding: 1.5rem;
  }

  .modal-card {
    background: #ffffff;
    border-radius: 20px;
    padding: 2rem;
    width: 100%;
    max-width: 480px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1.5rem;
  }

  .modal-title-wrap {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .modal-title-wrap h2 {
    font-size: 1.25rem;
    font-weight: 700;
    color: #0f172a;
    margin: 0;
  }

  .close-btn {
    background: none;
    border: none;
    color: #94a3b8;
    cursor: pointer;
  }

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 1.15rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .form-group label {
    font-size: 0.825rem;
    font-weight: 600;
    color: #334155;
  }

  .form-group input,
  .form-group select,
  .form-group textarea {
    padding: 0.65rem 0.85rem;
    border: 1px solid #e2e8f0;
    border-radius: 10px;
    background: #f8fafc;
    font-size: 0.9rem;
    color: #0f172a;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 1rem;
  }

  .btn-cancel {
    padding: 0.65rem 1rem;
    border-radius: 10px;
    border: 1px solid #e2e8f0;
    background: #ffffff;
    color: #475569;
    font-weight: 500;
    cursor: pointer;
  }

  .warning-callout {
    background: #fffbeb;
    border: 1px solid #fef3c7;
    color: #b45309;
    padding: 0.85rem 1rem;
    border-radius: 12px;
    font-size: 0.85rem;
    line-height: 1.4;
    margin-bottom: 1.25rem;
  }

  .credentials-box {
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    border-radius: 14px;
    padding: 1.25rem;
  }

  .cred-title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #0f172a;
    margin-bottom: 0.5rem;
  }

  .cred-desc {
    font-size: 0.85rem;
    color: #64748b;
    margin-bottom: 1rem;
  }

  .cred-info {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 8px;
    padding: 0.85rem;
    font-size: 0.9rem;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .cred-pwd {
    background: #fef2f2;
    color: #dc2626;
    padding: 0.2rem 0.5rem;
    border-radius: 4px;
    font-weight: 700;
  }

  .loading-wrap, .empty-wrap {
    text-align: center;
    padding: 3rem 1.5rem;
    color: #64748b;
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid #e2e8f0;
    border-top-color: #ff6b00;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin: 0 auto 0.75rem auto;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
