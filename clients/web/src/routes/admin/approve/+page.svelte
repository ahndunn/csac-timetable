<script lang="ts">
  import { onMount } from 'svelte';
  import { api, ApiError } from '$lib/api/client';
  import { auth } from '$lib/stores/auth.svelte';
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore, t } from '$lib/i18n';
  import {
    ShieldAlert,
    ShieldCheck,
    CheckCircle2,
    AlertCircle,
    X,
    KeyRound,
    Clock,
    UserX,
    Mail,
    ThumbsUp,
    ThumbsDown,
  } from '@lucide/svelte';

  interface ProposalItem {
    id: string;
    target_admin_id: string;
    target_name: string;
    target_email: string;
    target_role: 'moderator' | 'member';
    initiated_by: string;
    initiator_name: string;
    reason: string;
    total_admins_at_proposal: number;
    required_approvals: number;
    current_approvals: number;
    status: 'pending' | 'approved' | 'rejected' | 'expired';
    created_at: string;
    expires_at: string;
  }

  let proposals = $state<ProposalItem[]>([]);
  let isLoading = $state(true);
  let errorMessage = $state<string | null>(null);
  let successMessage = $state<string | null>(null);

  // OTP Vote Modal
  let isVoteModalOpen = $state(false);
  let selectedProposal = $state<ProposalItem | null>(null);
  let otpCode = $state('');
  let voteDecision = $state<'approve' | 'reject'>('approve');
  let isRequestingOtp = $state(false);
  let otpRequestedInfo = $state<string | null>(null);

  async function loadProposals() {
    isLoading = true;
    errorMessage = null;
    try {
      proposals = await api.admin.listProposals();
    } catch (err: any) {
      errorMessage = err.message || t('admin_approve.error_load');
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    loadProposals();
  });

  async function handleRequestOtp(proposal: ProposalItem) {
    isRequestingOtp = true;
    errorMessage = null;
    otpRequestedInfo = null;
    try {
      const res = await api.admin.requestOtp(proposal.id);
      otpRequestedInfo = t('admin_approve.otp_info', { minutes: res.ttl_seconds / 60 });
      selectedProposal = proposal;
      isVoteModalOpen = true;
    } catch (err: any) {
      errorMessage = err.message || t('admin_approve.error_otp');
    } finally {
      isRequestingOtp = false;
    }
  }

  async function handleSubmitVote(e: SubmitEvent) {
    e.preventDefault();
    if (!selectedProposal) return;

    try {
      const res = await api.admin.voteProposal(selectedProposal.id, {
        otp: otpCode,
        decision: voteDecision,
      });

      isVoteModalOpen = false;
      otpCode = '';
      if (res.role_downgraded) {
        successMessage = t('admin_approve.success_downgraded', { current: res.current_approvals, required: res.required_approvals, name: selectedProposal.target_name });
      } else {
        successMessage = t('admin_approve.success_recorded', { decision: voteDecision.toUpperCase(), current: res.current_approvals, required: res.required_approvals });
      }
      await loadProposals();
    } catch (err: any) {
      errorMessage = err.message || t('admin_approve.error_vote');
    }
  }
</script>

<svelte:head>
  <title>{$tStore('admin_approve.page_title')}</title>
</svelte:head>

<Navbar />

<div class="approve-container">
  <div class="header-card">
    <div class="header-content">
      <div class="icon-wrap">
        <ShieldAlert size={32} class="text-orange" />
      </div>
      <div>
        <h1 class="title">{$tStore('admin_approve.heading')}</h1>
        <p class="subtitle">
          {$tStore('admin_approve.subheading_prefix')}
          <strong>{$tStore('admin_approve.subheading_formula')}</strong> {$tStore('admin_approve.subheading_suffix')}
        </p>
      </div>
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

  {#if isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>{$tStore('admin_approve.loading')}</p>
    </div>
  {:else if proposals.length === 0}
    <div class="empty-state">
      <ShieldCheck size={48} class="empty-icon" />
      <h3>{$tStore('admin_approve.empty_title')}</h3>
      <p>{$tStore('admin_approve.empty_desc')}</p>
    </div>
  {:else}
    <div class="proposals-grid">
      {#each proposals as proposal (proposal.id)}
        <div class="proposal-card status-{proposal.status}">
          <div class="proposal-header">
            <div class="target-info">
              <span class="badge badge-{proposal.status}">{proposal.status.toUpperCase()}</span>
              <h3 class="target-name">{proposal.target_name}</h3>
              <span class="target-email">{proposal.target_email}</span>
            </div>
            <div class="role-shift">
              <span class="old-role">ADMIN</span>
              <span class="arrow">&rarr;</span>
              <span class="new-role">{proposal.target_role.toUpperCase()}</span>
            </div>
          </div>

          <div class="reason-box">
            <strong>{$tStore('admin_approve.card_reason')}</strong> {proposal.reason}
          </div>

          <div class="meta-row">
            <span>{$tStore('admin_approve.card_initiated_by')} <strong>{proposal.initiator_name}</strong></span>
            <span>{$tStore('admin_approve.card_expires')} <strong>{new Date(proposal.expires_at).toLocaleDateString()}</strong></span>
          </div>

          <!-- Quorum Progress -->
          <div class="progress-section">
            <div class="progress-labels">
              <span>{$tStore('admin_approve.card_quorum_progress')} <strong>{$tStore('admin_approve.card_quorum_required', { current: proposal.current_approvals, required: proposal.required_approvals })}</strong></span>
              <span>{Math.round((proposal.current_approvals / proposal.required_approvals) * 100)}%</span>
            </div>
            <div class="progress-bar-bg">
              <div
                class="progress-bar-fill"
                style="width: {Math.min(100, (proposal.current_approvals / proposal.required_approvals) * 100)}%;"
              ></div>
            </div>
          </div>

          {#if proposal.status === 'pending'}
            <div class="action-footer">
              <button class="otp-request-btn" onclick={() => handleRequestOtp(proposal)} disabled={isRequestingOtp}>
                <KeyRound size={16} />
                <span>{$tStore('admin_approve.card_btn_verify')}</span>
              </button>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Modal: OTP & Vote Submission -->
{#if isVoteModalOpen && selectedProposal}
  <div class="modal-overlay">
    <div class="modal-card">
      <div class="modal-header">
        <div class="modal-title-wrap">
          <KeyRound size={22} class="text-orange" />
          <h2>{$tStore('admin_approve.modal_title')}</h2>
        </div>
        <button class="close-btn" onclick={() => isVoteModalOpen = false}><X size={20} /></button>
      </div>

      {#if otpRequestedInfo}
        <div class="otp-info-banner">
          <Mail size={18} />
          <span>{otpRequestedInfo}</span>
        </div>
      {/if}

      <form onsubmit={handleSubmitVote} class="modal-form">
        <div class="proposal-summary">
          <div><strong>{$tStore('admin_approve.modal_target_admin')}</strong> {selectedProposal.target_name} ({selectedProposal.target_email})</div>
          <div><strong>{$tStore('admin_approve.modal_proposed_role')}</strong> {selectedProposal.target_role.toUpperCase()}</div>
        </div>

        <div class="form-group">
          <label for="otp">{$tStore('admin_approve.modal_otp_label')}</label>
          <input
            id="otp"
            type="text"
            maxlength="6"
            placeholder="123456"
            bind:value={otpCode}
            class="otp-input"
            required
            autocomplete="one-time-code"
          />
        </div>

        <div class="form-group">
          <label>{$tStore('admin_approve.modal_decision_label')}</label>
          <div class="decision-radios">
            <label class="radio-label {voteDecision === 'approve' ? 'selected-approve' : ''}">
              <input type="radio" name="decision" value="approve" bind:group={voteDecision} />
              <ThumbsUp size={16} />
              <span>{$tStore('admin_approve.modal_decision_approve')}</span>
            </label>
            <label class="radio-label {voteDecision === 'reject' ? 'selected-reject' : ''}">
              <input type="radio" name="decision" value="reject" bind:group={voteDecision} />
              <ThumbsDown size={16} />
              <span>{$tStore('admin_approve.modal_decision_reject')}</span>
            </label>
          </div>
        </div>

        <div class="modal-footer">
          <button type="button" class="btn-cancel" onclick={() => isVoteModalOpen = false}>{$tStore('admin_approve.modal_btn_cancel')}</button>
          <button type="submit" class="primary-btn">{$tStore('admin_approve.modal_btn_submit')}</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .approve-container {
    max-width: 1100px;
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
  }

  .header-content {
    display: flex;
    align-items: center;
    gap: 1.25rem;
  }

  .icon-wrap {
    width: 56px;
    height: 56px;
    border-radius: 16px;
    background: rgba(255, 107, 0, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .text-orange { color: #ff6b00; }

  .title {
    font-size: 1.45rem;
    font-weight: 700;
    color: #0f172a;
    margin: 0 0 0.25rem 0;
  }

  .subtitle {
    font-size: 0.9rem;
    color: #64748b;
    margin: 0;
    line-height: 1.4;
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

  .proposals-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1.25rem;
  }

  .proposal-card {
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 18px;
    padding: 1.5rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.02);
  }

  .proposal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .target-name {
    font-size: 1.2rem;
    font-weight: 700;
    color: #0f172a;
    margin: 0.35rem 0 0 0;
  }

  .target-email {
    font-size: 0.85rem;
    color: #64748b;
  }

  .badge {
    font-size: 0.75rem;
    font-weight: 700;
    padding: 0.25rem 0.65rem;
    border-radius: 8px;
    display: inline-block;
  }

  .badge-pending { background: #fffbeb; color: #b45309; }
  .badge-approved { background: #f0fdf4; color: #16a34a; }
  .badge-rejected { background: #fef2f2; color: #ef4444; }

  .role-shift {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: #f8fafc;
    padding: 0.5rem 0.85rem;
    border-radius: 10px;
    font-weight: 700;
    font-size: 0.85rem;
  }

  .old-role { color: #9333ea; }
  .arrow { color: #94a3b8; }
  .new-role { color: #2563eb; }

  .reason-box {
    background: #f8fafc;
    border: 1px solid #f1f5f9;
    border-radius: 10px;
    padding: 0.85rem 1rem;
    font-size: 0.9rem;
    color: #334155;
    margin-bottom: 1rem;
  }

  .meta-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    color: #64748b;
    margin-bottom: 1.25rem;
  }

  .progress-section {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    margin-bottom: 1.25rem;
  }

  .progress-labels {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    font-weight: 600;
    color: #334155;
  }

  .progress-bar-bg {
    width: 100%;
    height: 10px;
    border-radius: 5px;
    background: #e2e8f0;
    overflow: hidden;
  }

  .progress-bar-fill {
    height: 100%;
    background: #ff6b00;
    border-radius: 5px;
    transition: width 0.3s;
  }

  .action-footer {
    padding-top: 1rem;
    border-top: 1px solid #f1f5f9;
    display: flex;
    justify-content: flex-end;
  }

  .otp-request-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: #ff6b00;
    color: #ffffff;
    border: none;
    padding: 0.65rem 1.25rem;
    border-radius: 12px;
    font-weight: 600;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .otp-request-btn:hover:not(:disabled) {
    background: #e65c00;
  }

  .otp-request-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
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
    max-width: 460px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1.25rem;
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

  .otp-info-banner {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    background: #eff6ff;
    border: 1px solid #dbeafe;
    color: #1d4ed8;
    padding: 0.75rem 1rem;
    border-radius: 12px;
    font-size: 0.85rem;
    margin-bottom: 1.25rem;
  }

  .proposal-summary {
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    border-radius: 10px;
    padding: 0.85rem;
    font-size: 0.875rem;
    color: #334155;
    margin-bottom: 1rem;
  }

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 1.15rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .form-group label {
    font-size: 0.825rem;
    font-weight: 600;
    color: #334155;
  }

  .otp-input {
    font-size: 1.5rem;
    font-weight: 700;
    letter-spacing: 0.35rem;
    text-align: center;
    padding: 0.75rem;
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    background: #f8fafc;
  }

  .otp-input:focus {
    outline: none;
    border-color: #ff6b00;
    background: #ffffff;
  }

  .decision-radios {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }

  .radio-label {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.75rem;
    border: 1px solid #e2e8f0;
    border-radius: 10px;
    cursor: pointer;
    font-weight: 600;
    font-size: 0.85rem;
    color: #475569;
    transition: all 0.2s;
  }

  .radio-label input { display: none; }
  .selected-approve { background: #f0fdf4; border-color: #16a34a; color: #16a34a; }
  .selected-reject { background: #fef2f2; border-color: #ef4444; color: #ef4444; }

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

  .primary-btn {
    background: #ff6b00;
    color: #ffffff;
    padding: 0.65rem 1.25rem;
    border-radius: 10px;
    font-weight: 600;
    border: none;
    cursor: pointer;
  }

  .empty-state {
    text-align: center;
    padding: 4rem 2rem;
    background: #ffffff;
    border: 1px dashed #cbd5e1;
    border-radius: 20px;
    color: #64748b;
  }

  .empty-icon { color: #10b981; margin-bottom: 1rem; }

  .loading-state { text-align: center; padding: 3rem 1.5rem; color: #64748b; }
  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid #e2e8f0;
    border-top-color: #ff6b00;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin: 0 auto 0.75rem auto;
  }

  @keyframes spin { to { transform: rotate(360deg); } }
</style>
