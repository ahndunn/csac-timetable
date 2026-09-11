<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    Music,
    CheckCircle2,
    Clock,
    AlertCircle,
    UserCheck,
    MessageSquare,
    Plus,
    X,
  } from '@lucide/svelte';

  interface SongNumber {
    id: string;
    title: string;
    pmName: string;
    stage: 'draft' | 'in_practice' | 'ready_for_qc' | 'qc_approved' | 'stage_ready';
    qcReviewer: string;
    qcNotes?: string;
  }

  let numbers = $state<SongNumber[]>([
    {
      id: 'num-1',
      title: 'Hào Khí Việt Nam',
      pmName: 'Minh Pháp',
      stage: 'stage_ready',
      qcReviewer: 'Hoàng Nam',
      qcNotes: 'Flawless vocal harmonies and drum fills. Stage ready.',
    },
    {
      id: 'num-2',
      title: 'Đi Giữa Trời Rực Rỡ',
      pmName: 'Hoàng Nam',
      stage: 'qc_approved',
      qcReviewer: 'Thu Hà',
      qcNotes: 'Lead guitar solo approved.',
    },
    {
      id: 'num-3',
      title: 'Giọt Sương Trên Mí Mắt',
      pmName: 'Bảo Anh',
      stage: 'ready_for_qc',
      qcReviewer: 'Minh Pháp',
    },
    {
      id: 'num-4',
      title: 'Nối Torớ Lớn',
      pmName: 'Thu Hà',
      stage: 'in_practice',
      qcReviewer: 'Bảo Anh',
    },
  ]);

  let isQcDrawerOpen = $state(false);
  let activeSongForQc = $state<SongNumber | null>(null);
  let qcVerdict = $state<'pass' | 'revision'>('pass');
  let qcNotesInput = $state('');

  function openQcDrawer(song: SongNumber) {
    activeSongForQc = song;
    qcNotesInput = song.qcNotes || '';
    isQcDrawerOpen = true;
  }

  function handleQcSubmit(e: Event) {
    e.preventDefault();
    if (!activeSongForQc) return;

    numbers = numbers.map((n) =>
      n.id === activeSongForQc?.id
        ? {
            ...n,
            stage: qcVerdict === 'pass' ? 'qc_approved' : 'in_practice',
            qcNotes: qcNotesInput,
          }
        : n
    );

    isQcDrawerOpen = false;
    activeSongForQc = null;
  }
</script>

<div class="numbers-subpage">
  <div class="header-actions">
    <h2>Music Numbers ({numbers.length})</h2>
    <button type="button" class="bento-btn bento-btn-primary">
      <Plus size={16} />
      <span>Add Music Number</span>
    </button>
  </div>

  <!-- Kanban Columns -->
  <div class="kanban-board">
    <!-- Stage: In Practice -->
    <div class="kanban-col bento-card">
      <div class="col-header">
        <span class="col-title">{$tStore('studio_shows.kanban_practice')}</span>
        <span class="col-count">{numbers.filter((n) => n.stage === 'in_practice').length}</span>
      </div>

      <div class="col-cards">
        {#each numbers.filter((n) => n.stage === 'in_practice') as song (song.id)}
          <div class="song-card bento-card">
            <h4 class="song-title">{song.title}</h4>
            <div class="song-pm">Leader (PM): {song.pmName}</div>
            <div class="song-qc-meta">QC Reviewer: {song.qcReviewer}</div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Stage: Ready for QC -->
    <div class="kanban-col bento-card">
      <div class="col-header">
        <span class="col-title text-orange">{$tStore('studio_shows.kanban_ready_qc')}</span>
        <span class="col-count">{numbers.filter((n) => n.stage === 'ready_for_qc').length}</span>
      </div>

      <div class="col-cards">
        {#each numbers.filter((n) => n.stage === 'ready_for_qc') as song (song.id)}
          <div class="song-card bento-card">
            <h4 class="song-title">{song.title}</h4>
            <div class="song-pm">Leader (PM): {song.pmName}</div>
            <div class="song-qc-meta">QC Reviewer: {song.qcReviewer}</div>

            <button
              type="button"
              class="bento-btn bento-btn-sm qc-btn"
              onclick={() => openQcDrawer(song)}
            >
              <UserCheck size={13} />
              <span>Audit & Submit QC</span>
            </button>
          </div>
        {/each}
      </div>
    </div>

    <!-- Stage: QC Approved -->
    <div class="kanban-col bento-card">
      <div class="col-header">
        <span class="col-title text-blue">{$tStore('studio_shows.kanban_qc_approved')}</span>
        <span class="col-count">{numbers.filter((n) => n.stage === 'qc_approved').length}</span>
      </div>

      <div class="col-cards">
        {#each numbers.filter((n) => n.stage === 'qc_approved') as song (song.id)}
          <div class="song-card bento-card">
            <h4 class="song-title">{song.title}</h4>
            <div class="song-pm">Leader (PM): {song.pmName}</div>
            <div class="song-qc-meta">QC Reviewer: {song.qcReviewer}</div>
            {#if song.qcNotes}
              <div class="qc-notes-box"><MessageSquare size={12} /> {song.qcNotes}</div>
            {/if}
          </div>
        {/each}
      </div>
    </div>

    <!-- Stage: Stage Ready -->
    <div class="kanban-col bento-card">
      <div class="col-header">
        <span class="col-title text-green">{$tStore('studio_shows.kanban_stage_ready')}</span>
        <span class="col-count">{numbers.filter((n) => n.stage === 'stage_ready').length}</span>
      </div>

      <div class="col-cards">
        {#each numbers.filter((n) => n.stage === 'stage_ready') as song (song.id)}
          <div class="song-card bento-card highlight-green">
            <h4 class="song-title">{song.title}</h4>
            <div class="song-pm">Leader (PM): {song.pmName}</div>
            <div class="song-qc-meta">QC Reviewer: {song.qcReviewer}</div>
            {#if song.qcNotes}
              <div class="qc-notes-box"><MessageSquare size={12} /> {song.qcNotes}</div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>

<!-- Modal: QC Verdict Drawer -->
{#if isQcDrawerOpen && activeSongForQc}
  <div class="modal-backdrop" onclick={() => (isQcDrawerOpen = false)} role="presentation">
    <div class="modal-card bento-card" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="drawer-header">
        <h2>{$tStore('studio_shows.qc_drawer_title')}</h2>
        <button class="icon-close" onclick={() => (isQcDrawerOpen = false)}><X size={18} /></button>
      </div>

      <p class="song-ref">Song: <strong>{activeSongForQc.title}</strong></p>
      <p class="reviewer-ref">Assigned QC Reviewer: <strong>{activeSongForQc.qcReviewer}</strong></p>

      <form onsubmit={handleQcSubmit} class="modal-form">
        <div class="form-group">
          <label for="verdict">{$tStore('studio_shows.qc_verdict')}</label>
          <div class="radio-group">
            <label class="radio-label">
              <input type="radio" bind:group={qcVerdict} value="pass" />
              <span>{$tStore('studio_shows.qc_pass')}</span>
            </label>
            <label class="radio-label">
              <input type="radio" bind:group={qcVerdict} value="revision" />
              <span>{$tStore('studio_shows.qc_revision')}</span>
            </label>
          </div>
        </div>

        <div class="form-group">
          <label for="qc-notes">{$tStore('studio_shows.qc_notes')}</label>
          <textarea
            id="qc-notes"
            bind:value={qcNotesInput}
            rows="3"
            placeholder="Feedback notes..."
            required
            class="form-input"
          ></textarea>
        </div>

        <div class="modal-actions">
          <button type="button" class="bento-btn" onclick={() => (isQcDrawerOpen = false)}>Cancel</button>
          <button type="submit" class="bento-btn bento-btn-primary">
            {$tStore('studio_shows.btn_submit_qc')}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .numbers-subpage {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .header-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .header-actions h2 {
    font-size: 18px;
    font-weight: 800;
    margin: 0;
  }

  .kanban-board {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 16px;

  }

  .bento-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 16px;
    padding: 16px;
  }

  .kanban-col {
    display: flex;
    flex-direction: column;
    gap: 14px;
    background: #f8fafc;
    min-height: 450px;
  }

  .col-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .col-title {
    font-size: 13px;
    font-weight: 700;
    color: #475569;
  }

  .col-title.text-orange { color: #ff6b00; }
  .col-title.text-blue { color: #2563eb; }
  .col-title.text-green { color: #16a34a; }

  .col-count {
    background: #e2e8f0;
    color: #475569;
    padding: 2px 8px;
    border-radius: 10px;
    font-size: 12px;
    font-weight: 700;
  }

  .col-cards {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .song-card {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 12px;
    background: #ffffff;
  }

  .song-title {
    font-size: 14px;
    font-weight: 700;
    color: #0f172a;
    margin: 0;
  }

  .song-pm, .song-qc-meta {
    font-size: 12px;
    color: #64748b;
  }

  .qc-notes-box {
    margin-top: 4px;
    font-size: 11px;
    background: #f1f5f9;
    padding: 6px 8px;
    border-radius: 6px;
    color: #334155;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .qc-btn {
    margin-top: 6px;
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    font-weight: 700;
  }

  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 23, 42, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 16px;
  }

  .modal-card {
    width: 100%;
    max-width: 460px;
  }

  .drawer-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .icon-close {
    background: none;
    border: none;
    cursor: pointer;
    color: #94a3b8;
  }

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 14px;
    margin-top: 14px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .radio-group {
    display: flex;
    gap: 16px;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    font-weight: 600;
  }

  .form-input {
    padding: 8px 12px;
    border: 1px solid #cbd5e1;
    border-radius: 8px;
    font-size: 13px;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
  }

  @media (max-width: 960px) {
    .kanban-board {
      grid-template-columns: 1fr 1fr;
    }
  }
</style>
