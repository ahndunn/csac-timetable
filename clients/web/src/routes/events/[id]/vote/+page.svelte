<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import { api, ApiError } from '$lib/api/client';
  import { auth } from '$lib/stores/auth.svelte';
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore, t } from '$lib/i18n';
  import {
    Vote,
    Check,
    X,
    Calendar,
    Lock,
    CheckCircle2,
    AlertCircle,
    ArrowLeft,
    Sparkles,
  } from '@lucide/svelte';

  let eventId = $derived(page.params.id);
  let eventData = $state<any | null>(null);
  let timeSlots = $state<any[]>([]);
  let userVotes = $state<Record<string, { is_available: boolean; note?: string }>>({});
  let isLoading = $state(true);
  let isSubmitting = $state(false);
  let errorMessage = $state<string | null>(null);
  let successMessage = $state<string | null>(null);

  async function loadEventData() {
    isLoading = true;
    errorMessage = null;
    try {
      const res = await api.events.get(eventId);
      eventData = res.event;
      timeSlots = res.time_slots;
      // Initialize votes
      for (const slot of res.time_slots) {
        if (!userVotes[slot.id]) {
          userVotes[slot.id] = { is_available: false, note: '' };
        }
      }
    } catch (err: any) {
      errorMessage = err.message || t('events_vote.error_load');
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    loadEventData();
  });

  function toggleSlot(slotId: string) {
    if (eventData?.status === 'closed') return;
    const current = userVotes[slotId]?.is_available ?? false;
    userVotes[slotId] = {
      ...userVotes[slotId],
      is_available: !current,
    };
  }

  async function handleSaveVotes() {
    if (eventData?.status === 'closed') return;
    isSubmitting = true;
    errorMessage = null;
    successMessage = null;

    try {
      for (const [slot_id, vote] of Object.entries(userVotes)) {
        await api.events.submitVote(eventId, {
          slot_id,
          is_available: vote.is_available,
          note: vote.note || undefined,
        });
      }
      successMessage = t('events_vote.success_saved');
    } catch (err: any) {
      errorMessage = err.message || t('events_vote.error_save');
    } finally {
      isSubmitting = false;
    }
  }
</script>

<svelte:head>
  <title>{$tStore('events_vote.page_title')}</title>
</svelte:head>

<Navbar />

<div class="vote-container">
  <div class="top-nav">
    <a href="/admin/events" class="back-link">
      <ArrowLeft size={16} />
      <span>{$tStore('events_vote.back_to_events')}</span>
    </a>
  </div>

  {#if isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>{$tStore('events_vote.loading')}</p>
    </div>
  {:else if !eventData}
    <div class="alert-box alert-error">
      <AlertCircle size={18} />
      <span>{$tStore('events_vote.error_not_found')}</span>
    </div>
  {:else}
    <div class="event-hero-card">
      <div class="hero-left">
        <span class="badge badge-{eventData.status}">{eventData.status.toUpperCase()}</span>
        <h1 class="hero-title">{eventData.title}</h1>
        {#if eventData.description}
          <p class="hero-desc">{eventData.description}</p>
        {/if}
      </div>

      <div class="hero-right">
        <div class="dates-tag">
          <Calendar size={18} class="text-orange" />
          <span>{eventData.start_date} &rarr; {eventData.end_date}</span>
        </div>
        {#if eventData.status === 'closed'}
          <div class="closed-badge">
            <Lock size={16} />
            <span>{$tStore('events_vote.closed_badge')}</span>
          </div>
        {/if}
      </div>
    </div>

    {#if successMessage}
      <div class="alert-box alert-success">
        <CheckCircle2 size={18} />
        <span>{successMessage}</span>
      </div>
    {/if}

    {#if errorMessage}
      <div class="alert-box alert-error">
        <AlertCircle size={18} />
        <span>{errorMessage}</span>
      </div>
    {/if}

    <div class="matrix-card">
      <div class="matrix-header">
        <div>
          <h2 class="matrix-title">{$tStore('events_vote.matrix_title')}</h2>
          <p class="matrix-subtitle">{$tStore('events_vote.matrix_subtitle')}</p>
        </div>

        {#if eventData.status === 'open'}
          <button class="primary-btn" onclick={handleSaveVotes} disabled={isSubmitting}>
            <Sparkles size={16} />
            <span>{isSubmitting ? $tStore('events_vote.btn_saving') : $tStore('events_vote.btn_save')}</span>
          </button>
        {/if}
      </div>

      <div class="slots-grid">
        {#each timeSlots as slot (slot.id)}
          <button
            type="button"
            class="slot-cell {userVotes[slot.id]?.is_available ? 'slot-available' : 'slot-unavailable'} {eventData.status === 'closed' ? 'slot-disabled' : ''}"
            onclick={() => toggleSlot(slot.id)}
          >
            <div class="slot-header">
              <span class="slot-day">{slot.day_of_week}</span>
              {#if userVotes[slot.id]?.is_available}
                <Check size={18} class="text-check" />
              {:else}
                <X size={18} class="text-cross" />
              {/if}
            </div>
            <span class="slot-time">{slot.slot_label}</span>
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .vote-container {
    max-width: 1000px;
    margin: 0 auto;
    padding: 2rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .back-link {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.85rem;
    font-weight: 600;
    color: #64748b;
    text-decoration: none;
    transition: color 0.2s;
  }

  .back-link:hover { color: #ff6b00; }

  .event-hero-card {
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 20px;
    padding: 1.75rem 2rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.03);
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    flex-wrap: wrap;
    gap: 1.25rem;
  }

  .hero-title {
    font-size: 1.45rem;
    font-weight: 700;
    color: #0f172a;
    margin: 0.4rem 0 0.25rem 0;
  }

  .hero-desc {
    font-size: 0.9rem;
    color: #64748b;
    margin: 0;
  }

  .badge {
    font-size: 0.75rem;
    font-weight: 700;
    padding: 0.25rem 0.65rem;
    border-radius: 8px;
    display: inline-block;
  }

  .badge-open { background: #f0fdf4; color: #16a34a; }
  .badge-closed { background: #f8fafc; color: #64748b; border: 1px solid #e2e8f0; }

  .hero-right {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 0.5rem;
  }

  .dates-tag {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    padding: 0.5rem 0.85rem;
    border-radius: 10px;
    font-size: 0.85rem;
    font-weight: 600;
    color: #334155;
  }

  .text-orange { color: #ff6b00; }

  .closed-badge {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    background: #fef2f2;
    border: 1px solid #fee2e2;
    color: #dc2626;
    padding: 0.35rem 0.75rem;
    border-radius: 8px;
    font-size: 0.8rem;
    font-weight: 600;
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

  .matrix-card {
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 20px;
    padding: 1.75rem 2rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.03);
  }

  .matrix-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 1.5rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid #f1f5f9;
  }

  .matrix-title {
    font-size: 1.25rem;
    font-weight: 700;
    color: #0f172a;
    margin: 0 0 0.25rem 0;
  }

  .matrix-subtitle {
    font-size: 0.875rem;
    color: #64748b;
    margin: 0;
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

  .primary-btn:hover:not(:disabled) {
    background: #e65c00;
  }

  .primary-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .slots-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 1rem;
  }

  .slot-cell {
    border-radius: 14px;
    padding: 1.15rem;
    cursor: pointer;
    transition: all 0.2s;
    text-align: left;
    border: 2px solid;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .slot-available {
    background: #f0fdf4;
    border-color: #22c55e;
  }

  .slot-unavailable {
    background: #f8fafc;
    border-color: #e2e8f0;
  }

  .slot-disabled {
    cursor: not-allowed;
    opacity: 0.7;
  }

  .slot-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .slot-day {
    font-size: 0.8rem;
    font-weight: 700;
    color: #475569;
    text-transform: uppercase;
  }

  .text-check { color: #16a34a; }
  .text-cross { color: #94a3b8; }

  .slot-time {
    font-size: 1.05rem;
    font-weight: 600;
    color: #0f172a;
  }

  .loading-state {
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

  @keyframes spin { to { transform: rotate(360deg); } }
</style>
