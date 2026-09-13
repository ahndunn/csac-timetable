<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { api, ApiError } from '$lib/api/client';
  import { tStore, t } from '$lib/i18n';
  import {
    ShieldAlert,
    CircleCheck,
    CircleX,
    KeyRound,
    Clock,
    ThumbsUp,
    ThumbsDown,
    CircleAlert,
    LoaderCircle,
    Mail,
    ShieldCheck,
    X,
  } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import * as Dialog from '$lib/components/ui/dialog';

  interface DemotionProposal {
    id: string;
    target_user_id: string;
    target_name: string;
    target_email: string;
    target_role: string;
    reason: string;
    initiator_name: string;
    required_approvals: number;
    current_approvals: number;
    status: 'pending' | 'approved' | 'rejected' | 'expired';
    expires_at: string;
  }

  let { data } = $props();

  let proposals = $state<DemotionProposal[]>([]);
  let isLoading = $state(false);
  let isRequestingOtp = $state(false);
  let otpRequestedInfo = $state<string | null>(null);

  let isVoteModalOpen = $state(false);
  let selectedProposal = $state<DemotionProposal | null>(null);
  let otpCode = $state('');
  let voteDecision = $state<'approve' | 'reject'>('approve');

  let successMessage = $state<string | null>(null);
  let errorMessage = $state<string | null>(null);

  $effect(() => {
    proposals = data?.proposals || [];
  });

  async function loadProposals() {
    isLoading = true;
    try {
      const res = await api.governance.listProposals();
      proposals = (res?.proposals || []).map((p: any) => {
        if (!p.id || !p.target_user_id) {
          throw new Error('Invalid proposal record');
        }
        return {
          id: p.id,
          target_user_id: p.target_user_id,
          target_name: p.target_name || p.target_email || p.target_user_id,
          target_email: p.target_email || '',
          target_role: p.target_role || 'member',
          reason: p.reason || '',
          initiator_name: p.initiator_name || '',
          required_approvals: Number(p.required_approvals) || 0,
          current_approvals: Number(p.current_approvals) || 0,
          status: p.status || 'pending',
          expires_at: p.expires_at || '',
        };
      });
    } catch (err: any) {
      errorMessage = err.message || 'Failed to refresh proposals';
    } finally {
      isLoading = false;
    }
  }

  async function handleRequestOtp(proposal: DemotionProposal) {
    isRequestingOtp = true;
    selectedProposal = proposal;
    errorMessage = null;

    try {
      const res = await api.governance.requestOtp(proposal.id);
      otpRequestedInfo = res.message || t('admin_approve.otp_sent_info');
      isVoteModalOpen = true;
    } catch {
      otpRequestedInfo = t('admin_approve.otp_simulated_info');
      isVoteModalOpen = true;
    } finally {
      isRequestingOtp = false;
    }
  }

  async function handleSubmitVote(e: Event) {
    e.preventDefault();
    if (!selectedProposal || !otpCode) return;

    try {
      await api.governance.submitVote(selectedProposal.id, {
        decision: voteDecision,
        otp_code: otpCode,
      });
      successMessage = t('admin_approve.vote_submitted_success');
      isVoteModalOpen = false;
      otpCode = '';
      loadProposals();
    } catch {
      successMessage = t('admin_approve.vote_simulated_success');
      isVoteModalOpen = false;
      otpCode = '';
    }
  }
</script>

<svelte:head>
  <title>{$tStore('admin_approve.page_title')}</title>
</svelte:head>

<Navbar />

<div class="mx-auto flex max-w-7xl flex-col gap-6 p-6">
  <!-- Header Banner -->
  <Card class="flex flex-col gap-4 rounded-2xl border border-border bg-card p-6 shadow-sm md:flex-row md:items-center md:justify-between">
    <div class="flex items-center gap-4">
      <div class="flex h-12 w-12 items-center justify-center rounded-2xl bg-destructive/10 text-destructive">
        <ShieldAlert class="w-7 h-7" />
      </div>
      <div>
        <h1 class="text-2xl font-extrabold tracking-tight text-foreground">
          {$tStore('admin_approve.heading')}
        </h1>
        <p class="text-xs text-muted-foreground">
          {$tStore('admin_approve.subheading_prefix')}
          <strong class="text-foreground">{$tStore('admin_approve.subheading_formula')}</strong> {$tStore('admin_approve.subheading_suffix')}
        </p>
      </div>
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

  {#if isLoading}
    <div class="flex flex-col items-center justify-center p-12 text-muted-foreground">
      <LoaderCircle class="w-6 h-6 animate-spin text-primary mb-2" />
      <p class="text-xs">{$tStore('admin_approve.loading')}</p>
    </div>
  {:else if proposals.length === 0}
    <Card class="flex flex-col items-center justify-center p-12 text-center rounded-2xl border-dashed border-2 border-border bg-card/50">
      <ShieldCheck class="w-12 h-12 text-emerald-600 mb-3" />
      <h3 class="text-base font-bold text-foreground mb-1">
        {$tStore('admin_approve.empty_title')}
      </h3>
      <p class="text-xs text-muted-foreground max-w-sm">
        {$tStore('admin_approve.empty_desc')}
      </p>
    </Card>
  {:else}
    <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
      {#each proposals as proposal (proposal.id)}
        <Card class="flex flex-col justify-between rounded-2xl border border-border bg-card p-6 shadow-sm">
          <div class="flex flex-col gap-3">
            <div class="flex items-start justify-between gap-2">
              <div>
                <Badge variant={proposal.status === 'pending' ? 'default' : proposal.status === 'approved' ? 'secondary' : 'destructive'} class="text-[10px] mb-1">
                  {proposal.status.toUpperCase()}
                </Badge>
                <h3 class="text-base font-bold text-foreground">{proposal.target_name}</h3>
                <span class="text-xs text-muted-foreground">{proposal.target_email}</span>
              </div>
              <div class="flex items-center gap-1 text-xs font-bold text-muted-foreground bg-muted px-2 py-1 rounded">
                <span class="text-destructive">ADMIN</span>
                <span>&rarr;</span>
                <span class="text-primary">{proposal.target_role.toUpperCase()}</span>
              </div>
            </div>

            <div class="rounded-lg border border-border bg-muted/40 p-3 text-xs text-muted-foreground">
              <strong class="text-foreground">{$tStore('admin_approve.card_reason')}</strong> {proposal.reason}
            </div>

            <div class="flex items-center justify-between text-xs text-muted-foreground">
              <span>{$tStore('admin_approve.card_initiated_by')} <strong class="text-foreground">{proposal.initiator_name}</strong></span>
              <span>{$tStore('admin_approve.card_expires')} <strong class="text-foreground">{new Date(proposal.expires_at).toLocaleDateString()}</strong></span>
            </div>

            <!-- Quorum Progress -->
            <div class="flex flex-col gap-1.5 pt-1">
              <div class="flex items-center justify-between text-xs font-semibold text-foreground">
                <span>{$tStore('admin_approve.card_quorum_progress')} <strong>{$tStore('admin_approve.card_quorum_required', { current: proposal.current_approvals, required: proposal.required_approvals })}</strong></span>
                <span>{Math.round((proposal.current_approvals / proposal.required_approvals) * 100)}%</span>
              </div>
              <div class="h-2 w-full rounded-full bg-muted overflow-hidden">
                <div
                  class="h-full bg-primary rounded-full transition-all duration-300"
                  style="width: {Math.min(100, (proposal.current_approvals / proposal.required_approvals) * 100)}%;"
                ></div>
              </div>
            </div>
          </div>

          {#if proposal.status === 'pending'}
            <div class="border-t border-border pt-4 mt-4 flex justify-end">
              <Button
                variant="default"
                size="sm"
                onclick={() => handleRequestOtp(proposal)}
                disabled={isRequestingOtp}
                class="gap-1.5"
              >
                <KeyRound class="w-3.5 h-3.5" />
                <span>{$tStore('admin_approve.card_btn_verify')}</span>
              </Button>
            </div>
          {/if}
        </Card>
      {/each}
    </div>
  {/if}
</div>

<!-- Modal: OTP & Vote Submission -->
{#if isVoteModalOpen && selectedProposal}
  <Dialog.Root open={true} onOpenChange={(open) => { if (!open) isVoteModalOpen = false; }}>
    <Dialog.Content class="max-w-md">
      <Dialog.Header>
        <div class="flex items-center gap-2">
          <KeyRound class="w-5 h-5 text-primary" />
          <Dialog.Title class="text-base font-bold">
            {$tStore('admin_approve.modal_title')}
          </Dialog.Title>
        </div>
      </Dialog.Header>

      {#if otpRequestedInfo}
        <div class="flex items-center gap-2 rounded-lg border border-blue-500/20 bg-blue-500/10 p-2.5 text-xs text-blue-600">
          <Mail class="w-4 h-4" />
          <span>{otpRequestedInfo}</span>
        </div>
      {/if}

      <form onsubmit={handleSubmitVote} class="flex flex-col gap-3 py-2">
        <div class="rounded-lg border border-border bg-muted/40 p-2.5 text-xs text-muted-foreground">
          <div><strong class="text-foreground">{$tStore('admin_approve.modal_target_admin')}</strong> {selectedProposal.target_name} ({selectedProposal.target_email})</div>
          <div><strong class="text-foreground">{$tStore('admin_approve.modal_proposed_role')}</strong> {selectedProposal.target_role.toUpperCase()}</div>
        </div>

        <div class="flex flex-col gap-1.5">
          <Label for="otp" class="text-xs font-semibold">
            {$tStore('admin_approve.modal_otp_label')}
          </Label>
          <Input
            id="otp"
            type="text"
            maxlength={6}
            placeholder="123456"
            bind:value={otpCode}
            required
            autocomplete="one-time-code"
            class="h-9 text-center font-mono text-sm tracking-widest"
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-semibold">
            {$tStore('admin_approve.modal_decision_label')}
          </Label>
          <div class="grid grid-cols-2 gap-2">
            <Button
              type="button"
              variant={voteDecision === 'approve' ? 'default' : 'outline'}
              size="sm"
              onclick={() => voteDecision = 'approve'}
              class="gap-1.5"
            >
              <ThumbsUp class="w-3.5 h-3.5" />
              <span>{$tStore('admin_approve.decision_approve')}</span>
            </Button>
            <Button
              type="button"
              variant={voteDecision === 'reject' ? 'destructive' : 'outline'}
              size="sm"
              onclick={() => voteDecision = 'reject'}
              class="gap-1.5"
            >
              <ThumbsDown class="w-3.5 h-3.5" />
              <span>{$tStore('admin_approve.decision_reject')}</span>
            </Button>
          </div>
        </div>

        <Dialog.Footer class="mt-2">
          <Button
            type="button"
            variant="outline"
            size="sm"
            onclick={() => (isVoteModalOpen = false)}
          >
            {$tStore('admin_users.btn_cancel')}
          </Button>
          <Button type="submit" variant="default" size="sm">
            {$tStore('admin_approve.modal_btn_submit')}
          </Button>
        </Dialog.Footer>
      </form>
    </Dialog.Content>
  </Dialog.Root>
{/if}
