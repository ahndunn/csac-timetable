<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    Music,
    CheckCircle2,
    Clock,
    AlertCircle,
    UserCheck,
    Users,
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
    lineup?: {
      vocalLead?: string;
      guitarLead?: string;
      bass?: string;
      drums?: string;
    };
  }

  let numbers = $state<SongNumber[]>([
    {
      id: 'num-1',
      title: 'Hào Khí Việt Nam',
      pmName: 'Minh Pháp',
      stage: 'stage_ready',
      qcReviewer: 'Hoàng Nam',
      qcNotes: 'Flawless vocal harmonies and drum fills. Stage ready.',
      lineup: { vocalLead: 'Minh Pháp', guitarLead: 'Hoàng Nam', bass: 'Bảo Anh', drums: 'Thu Hà' },
    },
    {
      id: 'num-2',
      title: 'Đi Giữa Trời Rực Rỡ',
      pmName: 'Hoàng Nam',
      stage: 'qc_approved',
      qcReviewer: 'Thu Hà',
      qcNotes: 'Lead guitar solo approved.',
      lineup: { vocalLead: 'Gia Huy', guitarLead: 'Hoàng Nam', bass: 'Bảo Anh' },
    },
    {
      id: 'num-3',
      title: 'Giọt Sương Trên Mí Mắt',
      pmName: 'Bảo Anh',
      stage: 'ready_for_qc',
      qcReviewer: 'Minh Pháp',
      lineup: { vocalLead: 'Minh Pháp', bass: 'Bảo Anh' },
    },
    {
      id: 'num-4',
      title: 'Nối Torớ Lớn',
      pmName: 'Thu Hà',
      stage: 'in_practice',
      qcReviewer: 'Bảo Anh',
      lineup: { vocalLead: 'Anh Pha', drums: 'Thu Hà' },
    },
  ]);

  let availableRoster = [
    'Minh Pháp',
    'Hoàng Nam',
    'Bảo Anh',
    'Thu Hà',
    'Gia Huy',
    'Anh Pha',
  ];

  let isQcDrawerOpen = $state(false);
  let activeSongForQc = $state<SongNumber | null>(null);
  let qcVerdict = $state<'pass' | 'revision'>('pass');
  let qcNotesInput = $state('');

  let isLineupDrawerOpen = $state(false);
  let activeSongForLineup = $state<SongNumber | null>(null);
  let formVocalLead = $state('');
  let formGuitarLead = $state('');
  let formBass = $state('');
  let formDrums = $state('');

  function openQcDrawer(song: SongNumber) {
    activeSongForQc = song;
    qcNotesInput = song.qcNotes || '';
    isQcDrawerOpen = true;
  }

  function openLineupDrawer(song: SongNumber) {
    activeSongForLineup = song;
    formVocalLead = song.lineup?.vocalLead || '';
    formGuitarLead = song.lineup?.guitarLead || '';
    formBass = song.lineup?.bass || '';
    formDrums = song.lineup?.drums || '';
    isLineupDrawerOpen = true;
  }

  function handleLineupSubmit(e: Event) {
    e.preventDefault();
    if (!activeSongForLineup) return;

    numbers = numbers.map((n) =>
      n.id === activeSongForLineup?.id
        ? {
            ...n,
            lineup: {
              vocalLead: formVocalLead || undefined,
              guitarLead: formGuitarLead || undefined,
              bass: formBass || undefined,
              drums: formDrums || undefined,
            },
          }
        : n
    );
    isLineupDrawerOpen = false;
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

  function advanceStatus(song: SongNumber, nextStage: SongNumber['stage']) {
    numbers = numbers.map((n) => (n.id === song.id ? { ...n, stage: nextStage } : n));
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

  <!-- Kanban Board: 5 FSM Columns -->
  <div class="kanban-board">
    <!-- Stage: Draft -->
    <div class="kanban-col bento-card">
      <div class="col-header">
        <span class="col-title" style="color: #64748b">{$tStore('show_mgmt.status.draft')}</span>
        <span class="col-count">{numbers.filter((n) => n.stage === 'draft').length}</span>
      </div>
      <div class="col-cards">
        {#each numbers.filter((n) => n.stage === 'draft') as song (song.id)}
          <div class="song-card bento-card">
            <h4 class="song-title">{song.title}</h4>
            <div class="song-pm">Leader (PM): {song.pmName}</div>
            <button
              type="button"
              class="bento-btn bento-btn-sm action-btn"
              onclick={() => advanceStatus(song, 'in_practice')}
            >
              <span>Start Practice</span>
            </button>
          </div>
        {/each}
      </div>
    </div>

    <!-- Stage: In Practice -->
    <div class="kanban-col bento-card">
      <div class="col-header">
        <span class="col-title text-blue">{$tStore('studio_shows.kanban_practice')}</span>
        <span class="col-count">{numbers.filter((n) => n.stage === 'in_practice').length}</span>
      </div>
      <div class="col-cards">
        {#each numbers.filter((n) => n.stage === 'in_practice') as song (song.id)}
          <div class="song-card bento-card">
            <h4 class="song-title">{song.title}</h4>
            <div class="song-pm">Leader (PM): {song.pmName}</div>
            <div class="song-qc-meta">QC Reviewer: {song.qcReviewer}</div>

            {#if song.lineup}
              <div class="lineup-tags">
                {#if song.lineup.vocalLead}<span class="lineup-tag vocal">Vocal: {song.lineup.vocalLead}</span>{/if}
                {#if song.lineup.guitarLead}<span class="lineup-tag guitar">Guitar: {song.lineup.guitarLead}</span>{/if}
                {#if song.lineup.bass}<span class="lineup-tag bass">Bass: {song.lineup.bass}</span>{/if}
                {#if song.lineup.drums}<span class="lineup-tag drums">Drums: {song.lineup.drums}</span>{/if}
              </div>
            {/if}

            {#if song.qcNotes}
              <div class="qc-notes-box warning"><MessageSquare size={12} /> {song.qcNotes}</div>
            {/if}
            
            <div class="card-btn-row">
              <button
                type="button"
                class="bento-btn bento-btn-sm lineup-btn"
                onclick={() => openLineupDrawer(song)}
              >
                <Users size={12} />
                <span>Assign Lineup</span>
              </button>
              <button
                type="button"
                class="bento-btn bento-btn-sm action-btn-orange"
                onclick={() => advanceStatus(song, 'ready_for_qc')}
              >
                <span>Submit for QC</span>
              </button>
            </div>
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
            <button
              type="button"
              class="bento-btn bento-btn-sm action-btn-green"
              onclick={() => advanceStatus(song, 'stage_ready')}
            >
              <span>Promote to Stage Ready</span>
            </button>
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

<!-- Modal: Lineup Role Assignment Drawer -->
{#if isLineupDrawerOpen && activeSongForLineup}
  <div class="modal-backdrop" onclick={() => (isLineupDrawerOpen = false)} role="presentation">
    <div class="modal-card bento-card" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="drawer-header">
        <h2>Assign Song Band Lineup</h2>
        <button class="icon-close" onclick={() => (isLineupDrawerOpen = false)}><X size={18} /></button>
      </div>

      <p class="song-ref">Song: <strong>{activeSongForLineup.title}</strong></p>

      <form onsubmit={handleLineupSubmit} class="modal-form">
        <div class="form-group">
          <label for="role-vocal">Vocal Lead</label>
          <select id="role-vocal" bind:value={formVocalLead} class="form-input">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label for="role-guitar">Guitar Lead</label>
          <select id="role-guitar" bind:value={formGuitarLead} class="form-input">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label for="role-bass">Bass Guitar</label>
          <select id="role-bass" bind:value={formBass} class="form-input">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label for="role-drums">Drums</label>
          <select id="role-drums" bind:value={formDrums} class="form-input">
            <option value="">-- Unassigned --</option>
            {#each availableRoster as member}
              <option value={member}>{member}</option>
            {/each}
          </select>
        </div>

        <div class="modal-actions">
          <button type="button" class="bento-btn" onclick={() => (isLineupDrawerOpen = false)}>Cancel</button>
          <button type="submit" class="bento-btn bento-btn-primary">
            Save Lineup Allocation
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
    grid-template-columns: repeat(5, 1fr);
    gap: 14px;
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
    font-size: 12px;
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

  .song-card.highlight-green {
    border-left: 3px solid #16a34a;
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

  .qc-notes-box.warning {
    background: #fff7ed;
    color: #c2410c;
    border: 1px solid #ffedd5;
  }

  .action-btn {
    margin-top: 6px;
    font-size: 11px;
    padding: 4px 8px;
  }

  .action-btn-orange {
    margin-top: 6px;
    font-size: 11px;
    padding: 4px 8px;
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    font-weight: 700;
  }

  .action-btn-green {
    margin-top: 6px;
    font-size: 11px;
    padding: 4px 8px;
    background: rgba(22, 163, 74, 0.1);
    color: #16a34a;
    font-weight: 700;
  }

  .qc-btn {
    margin-top: 6px;
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    font-weight: 700;
    font-size: 11px;
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

  .lineup-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    margin-top: 4px;
  }

  .lineup-tag {
    font-size: 10px;
    font-weight: 700;
    padding: 2px 6px;
    border-radius: 4px;
    background: #f1f5f9;
    color: #475569;
  }

  .lineup-tag.vocal { background: rgba(255, 107, 0, 0.1); color: #ff6b00; }
  .lineup-tag.guitar { background: rgba(59, 130, 246, 0.1); color: #2563eb; }
  .lineup-tag.bass { background: rgba(147, 51, 234, 0.1); color: #9333ea; }
  .lineup-tag.drums { background: rgba(22, 163, 74, 0.1); color: #16a34a; }

  .card-btn-row {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 6px;
    width: 100%;
  }

  .lineup-btn, .action-btn-orange {
    flex: 1 1 auto;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    white-space: nowrap;
    font-size: 11px;
    padding: 6px 10px;
    border-radius: 8px;
  }

  .lineup-btn {
    background: #f1f5f9;
    color: #334155;
  }

  @media (max-width: 1100px) {
    .kanban-board {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>


