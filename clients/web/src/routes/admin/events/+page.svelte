<script lang="ts">
  import { onMount } from 'svelte';
  import { api, ApiError } from '$lib/api/client';
  import { auth } from '$lib/stores/auth.svelte';
  import Navbar from '$lib/components/Navbar.svelte';
  import { tStore, t } from '$lib/i18n';
  import {
    Calendar,
    CalendarPlus,
    Lock,
    CheckCircle2,
    AlertCircle,
    X,
    Vote,
    Plus,
    Trash2,
    Clock,
    ArrowRight,
  } from '@lucide/svelte';

  interface EventItem {
    id: string;
    title: string;
    description: string | null;
    start_date: string;
    end_date: string;
    status: 'draft' | 'open' | 'closed';
    created_at: string;
    closed_at: string | null;
  }

  let events = $state<EventItem[]>([]);
  let isLoading = $state(true);
  let errorMessage = $state<string | null>(null);
  let successMessage = $state<string | null>(null);

  // Create Event Modal
  let isCreateModalOpen = $state(false);
  let eventTitle = $state('');
  let eventDescription = $state('');
  let eventStartDate = $state(new Date().toISOString().split('T')[0]);
  let eventEndDate = $state(new Date(Date.now() + 7 * 86400000).toISOString().split('T')[0]);
  let timeSlots = $state<Array<{ day_of_week: string; slot_label: string }>>([
    { day_of_week: 'THỨ HAI', slot_label: '17h - 18h' },
    { day_of_week: 'THỨ HAI', slot_label: '18h - 19h' },
    { day_of_week: 'THỨ BA', slot_label: '17h - 18h' },
    { day_of_week: 'THỨ BA', slot_label: '18h - 19h' },
    { day_of_week: 'THỨ TƯ', slot_label: '17h - 18h' },
    { day_of_week: 'THỨ NĂM', slot_label: '17h - 18h' },
    { day_of_week: 'THỨ SÁU', slot_label: '17h - 18h' },
    { day_of_week: 'THỨ BẢY', slot_label: '14h - 15h' },
    { day_of_week: 'CHỦ NHẬT', slot_label: '14h - 15h' },
  ]);

  let newSlotDay = $state('THỨ HAI');
  let newSlotLabel = $state('');

  async function loadEvents() {
    isLoading = true;
    errorMessage = null;
    try {
      events = await api.events.list();
    } catch (err: any) {
      errorMessage = err.message || t('admin_events.error_load');
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    loadEvents();
  });

  function handleAddSlot() {
    if (!newSlotLabel.trim()) return;
    timeSlots = [...timeSlots, { day_of_week: newSlotDay, slot_label: newSlotLabel.trim() }];
    newSlotLabel = '';
  }

  function handleRemoveSlot(index: number) {
    timeSlots = timeSlots.filter((_, i) => i !== index);
  }

  async function handleCreateEvent(e: SubmitEvent) {
    e.preventDefault();
    errorMessage = null;
    if (timeSlots.length === 0) {
      errorMessage = t('admin_events.error_min_slots');
      return;
    }

    try {
      await api.events.create({
        title: eventTitle,
        description: eventDescription || undefined,
        start_date: eventStartDate,
        end_date: eventEndDate,
        time_slots: timeSlots,
      });

      isCreateModalOpen = false;
      successMessage = t('admin_events.success_created', { title: eventTitle });
      eventTitle = '';
      eventDescription = '';
      await loadEvents();
    } catch (err: any) {
      errorMessage = err.message || t('admin_events.error_create');
    }
  }

  async function handleCloseEvent(eventId: string, title: string) {
    if (!confirm(t('admin_events.confirm_close', { title }))) {
      return;
    }

    try {
      await api.events.close(eventId);
      successMessage = t('admin_events.success_closed', { title });
      await loadEvents();
    } catch (err: any) {
      errorMessage = err.message || t('admin_events.error_close');
    }
  }
</script>

<svelte:head>
  <title>{$tStore('admin_events.page_title')}</title>
</svelte:head>

<Navbar />

<div class="events-container">
  <div class="header-card">
    <div class="header-main">
      <div class="icon-box">
        <Calendar size={28} class="text-orange" />
      </div>
      <div>
        <h1 class="page-title">{$tStore('admin_events.heading')}</h1>
        <p class="page-desc">{$tStore('admin_events.subheading')}</p>
      </div>
    </div>

    <button class="primary-btn" onclick={() => isCreateModalOpen = true}>
      <CalendarPlus size={18} />
      <span>{$tStore('admin_events.btn_create')}</span>
    </button>
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
      <p>{$tStore('admin_events.loading')}</p>
    </div>
  {:else if events.length === 0}
    <div class="empty-state">
      <Calendar size={48} class="empty-icon" />
      <h3>{$tStore('admin_events.empty_title')}</h3>
      <p>{$tStore('admin_events.empty_desc')}</p>
    </div>
  {:else}
    <div class="events-grid">
      {#each events as event (event.id)}
        <div class="event-card status-{event.status}">
          <div class="event-header">
            <div>
              <span class="badge badge-{event.status}">{event.status.toUpperCase()}</span>
              <h3 class="event-title">{event.title}</h3>
              {#if event.description}
                <p class="event-desc">{event.description}</p>
              {/if}
            </div>

            <div class="dates-pill">
              <Clock size={16} class="text-orange" />
              <span>{event.start_date} &rarr; {event.end_date}</span>
            </div>
          </div>

          <div class="card-actions">
            {#if event.status === 'open'}
              <a href="/events/{event.id}/vote" class="vote-btn">
                <Vote size={16} />
                <span>{$tStore('admin_events.action_open_portal')}</span>
              </a>
              <button class="close-event-btn" onclick={() => handleCloseEvent(event.id, event.title)}>
                <Lock size={16} />
                <span>{$tStore('admin_events.action_close')}</span>
              </button>
            {:else}
              <div class="closed-notice">
                <Lock size={16} />
                <span>{$tStore('admin_events.action_closed_on', { date: event.closed_at ? new Date(event.closed_at).toLocaleDateString() : 'N/A' })}</span>
              </div>
              <a href="/utils/timetable" class="schedule-btn">
                <span>{$tStore('admin_events.action_solve')}</span>
                <ArrowRight size={14} />
              </a>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Modal: Create Event Wizard -->
{#if isCreateModalOpen}
  <div class="modal-overlay">
    <div class="modal-card modal-large">
      <div class="modal-header">
        <div class="modal-title-wrap">
          <CalendarPlus size={22} class="text-orange" />
          <h2>{$tStore('admin_events.modal_title')}</h2>
        </div>
        <button class="close-btn" onclick={() => isCreateModalOpen = false}><X size={20} /></button>
      </div>

      <form onsubmit={handleCreateEvent} class="modal-form">
        <div class="form-row">
          <div class="form-group flex-2">
            <label for="title">{$tStore('admin_events.modal_event_title')}</label>
            <input id="title" type="text" placeholder={$tStore('admin_events.modal_event_title_placeholder')} bind:value={eventTitle} required />
          </div>
        </div>

        <div class="form-group">
          <label for="desc">{$tStore('admin_events.modal_desc')}</label>
          <textarea id="desc" rows="2" placeholder={$tStore('admin_events.modal_desc_placeholder')} bind:value={eventDescription}></textarea>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="start-date">{$tStore('admin_events.modal_start_date')}</label>
            <input id="start-date" type="date" bind:value={eventStartDate} required />
          </div>
          <div class="form-group">
            <label for="end-date">{$tStore('admin_events.modal_end_date')}</label>
            <input id="end-date" type="date" bind:value={eventEndDate} required />
          </div>
        </div>

        <div class="slots-section">
          <label class="slots-title">{$tStore('admin_events.modal_slots_title', { count: timeSlots.length })}</label>
          <div class="add-slot-row">
            <select bind:value={newSlotDay} class="slot-select">
              <option value="THỨ HAI">{$tStore('days.mon')}</option>
              <option value="THỨ BA">{$tStore('days.tue')}</option>
              <option value="THỨ TƯ">{$tStore('days.wed')}</option>
              <option value="THỨ NĂM">{$tStore('days.thu')}</option>
              <option value="THỨ SÁU">{$tStore('days.fri')}</option>
              <option value="THỨ BẢY">{$tStore('days.sat')}</option>
              <option value="CHỦ NHẬT">{$tStore('days.sun')}</option>
            </select>
            <input type="text" placeholder={$tStore('admin_events.modal_slot_placeholder')} bind:value={newSlotLabel} class="slot-input" />
            <button type="button" class="add-slot-btn" onclick={handleAddSlot}>
              <Plus size={16} />
              <span>{$tStore('admin_events.modal_btn_add_slot')}</span>
            </button>
          </div>

          <div class="slots-list">
            {#each timeSlots as slot, index}
              <div class="slot-tag">
                <span><strong>{slot.day_of_week}:</strong> {slot.slot_label}</span>
                <button type="button" class="remove-slot" onclick={() => handleRemoveSlot(index)}><Trash2 size={14} /></button>
              </div>
            {/each}
          </div>
        </div>

        <div class="modal-footer">
          <button type="button" class="btn-cancel" onclick={() => isCreateModalOpen = false}>{$tStore('admin_events.modal_btn_cancel')}</button>
          <button type="submit" class="primary-btn">{$tStore('admin_events.modal_btn_submit')}</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .events-container {
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

  .events-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1.25rem;
  }

  .event-card {
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 18px;
    padding: 1.5rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.02);
  }

  .event-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 1.25rem;
  }

  .event-title {
    font-size: 1.25rem;
    font-weight: 700;
    color: #0f172a;
    margin: 0.4rem 0 0.25rem 0;
  }

  .event-desc {
    font-size: 0.875rem;
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

  .dates-pill {
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

  .card-actions {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 0.75rem;
    padding-top: 1rem;
    border-top: 1px solid #f1f5f9;
  }

  .vote-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    padding: 0.55rem 1.15rem;
    border-radius: 10px;
    font-weight: 600;
    font-size: 0.9rem;
    text-decoration: none;
    transition: all 0.2s;
  }

  .vote-btn:hover {
    background: rgba(255, 107, 0, 0.2);
  }

  .close-event-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: #fef2f2;
    color: #dc2626;
    border: 1px solid #fee2e2;
    padding: 0.55rem 1rem;
    border-radius: 10px;
    font-weight: 600;
    font-size: 0.85rem;
    cursor: pointer;
  }

  .closed-notice {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.85rem;
    color: #64748b;
  }

  .schedule-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    color: #ff6b00;
    font-weight: 600;
    font-size: 0.85rem;
    text-decoration: none;
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
    max-width: 600px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
    max-height: 90vh;
    overflow-y: auto;
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

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 1.15rem;
  }

  .form-row {
    display: flex;
    gap: 1rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    flex: 1;
  }

  .form-group label {
    font-size: 0.825rem;
    font-weight: 600;
    color: #334155;
  }

  .form-group input,
  .form-group textarea,
  .form-group select {
    padding: 0.65rem 0.85rem;
    border: 1px solid #e2e8f0;
    border-radius: 10px;
    background: #f8fafc;
    font-size: 0.9rem;
  }

  .slots-section {
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    border-radius: 12px;
    padding: 1rem;
  }

  .slots-title {
    font-size: 0.85rem;
    font-weight: 600;
    color: #334155;
    margin-bottom: 0.5rem;
    display: block;
  }

  .add-slot-row {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
  }

  .slot-select {
    padding: 0.5rem;
    border: 1px solid #cbd5e1;
    border-radius: 8px;
    background: #ffffff;
    font-size: 0.85rem;
  }

  .slot-input {
    flex: 1;
    padding: 0.5rem 0.75rem;
    border: 1px solid #cbd5e1;
    border-radius: 8px;
    font-size: 0.85rem;
  }

  .add-slot-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    background: #334155;
    color: #ffffff;
    border: none;
    padding: 0.5rem 0.85rem;
    border-radius: 8px;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
  }

  .slots-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    max-height: 140px;
    overflow-y: auto;
  }

  .slot-tag {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    background: #ffffff;
    border: 1px solid #cbd5e1;
    padding: 0.35rem 0.65rem;
    border-radius: 8px;
    font-size: 0.8rem;
    color: #334155;
  }

  .remove-slot {
    background: none;
    border: none;
    color: #ef4444;
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
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

  .empty-state {
    text-align: center;
    padding: 4rem 2rem;
    background: #ffffff;
    border: 1px dashed #cbd5e1;
    border-radius: 20px;
    color: #64748b;
  }

  .empty-icon { color: #94a3b8; margin-bottom: 1rem; }
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
