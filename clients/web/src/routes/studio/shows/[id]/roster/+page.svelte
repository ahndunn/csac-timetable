<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { Users, UserCheck, Shield, Music, Mail, AlertTriangle } from '@lucide/svelte';

  interface Performer {
    id: string;
    name: string;
    email: string;
    role: 'DM' | 'PM' | 'Performer';
    instrument: string;
    assignedSongsCount: number;
    totalPracticeHours: number;
  }

  let roster = $state<Performer[]>([
    {
      id: 'p-1',
      name: 'Minh Pháp',
      email: 'minhphap@csac.local',
      role: 'DM',
      instrument: 'Lead Vocal / Acoustic Guitar',
      assignedSongsCount: 5,
      totalPracticeHours: 20,
    },
    {
      id: 'p-2',
      name: 'Hoàng Nam',
      email: 'hoangnam@csac.local',
      role: 'PM',
      instrument: 'Electric Guitar',
      assignedSongsCount: 3,
      totalPracticeHours: 12,
    },
    {
      id: 'p-3',
      name: 'Bảo Anh',
      email: 'baoanh@csac.local',
      role: 'PM',
      instrument: 'Bass Guitar',
      assignedSongsCount: 2,
      totalPracticeHours: 8,
    },
    {
      id: 'p-4',
      name: 'Thu Hà',
      email: 'thuha@csac.local',
      role: 'Performer',
      instrument: 'Drum Kit & Percussion',
      assignedSongsCount: 1,
      totalPracticeHours: 4,
    },
  ]);

  let isAddModalOpen = $state(false);
  let editingPerson = $state<Performer | null>(null);

  // Form State
  let formName = $state('');
  let formEmail = $state('');
  let formRole = $state<'DM' | 'PM' | 'Performer'>('Performer');
  let formInstrument = $state('Vocal Lead');

  function openAddModal() {
    editingPerson = null;
    formName = '';
    formEmail = '';
    formRole = 'Performer';
    formInstrument = 'Vocal Lead';
    isAddModalOpen = true;
  }

  function openEditModal(person: Performer) {
    editingPerson = person;
    formName = person.name;
    formEmail = person.email;
    formRole = person.role;
    formInstrument = person.instrument;
    isAddModalOpen = true;
  }

  function handleSaveMember(e: Event) {
    e.preventDefault();
    if (!formName.trim()) return;

    if (editingPerson) {
      roster = roster.map((p) =>
        p.id === editingPerson?.id
          ? { ...p, name: formName, email: formEmail, role: formRole, instrument: formInstrument }
          : p
      );
    } else {
      const newPerson: Performer = {
        id: `p-${Date.now()}`,
        name: formName,
        email: formEmail || `${formName.toLowerCase().replace(/\s+/g, '')}@csac.local`,
        role: formRole,
        instrument: formInstrument,
        assignedSongsCount: 1,
        totalPracticeHours: 4,
      };
      roster = [...roster, newPerson];
    }
    isAddModalOpen = false;
  }

  function handleRemoveMember(id: string) {
    roster = roster.filter((p) => p.id !== id);
  }

  function getWorkloadStatus(songCount: number): { key: string; level: 'optimal' | 'moderate' | 'fatigued'; colorClass: string } {
    if (songCount >= 5) return { key: 'show_mgmt.workload_fatigued', level: 'fatigued', colorClass: 'workload-red' };
    if (songCount >= 3) return { key: 'show_mgmt.workload_moderate', level: 'moderate', colorClass: 'workload-yellow' };
    return { key: 'show_mgmt.workload_optimal', level: 'optimal', colorClass: 'workload-green' };
  }
</script>

<div class="roster-subpage">
  <div class="header-bar bento-card">
    <div>
      <h2>Show Band Roster ({roster.length} Members)</h2>
      <p>Delivery Manager (DM), Performance Managers (PMs), and assigned performers for this show.</p>
    </div>

    <button type="button" class="bento-btn bento-btn-primary" onclick={openAddModal}>
      <Users size={16} />
      <span>{$tStore('show_mgmt.roster_modal.btn_add_member')}</span>
    </button>
  </div>


  <!-- Resource Allocation Workload Summary -->
  <div class="workload-summary bento-card">
    <div class="summary-item">
      <span class="summary-count">{roster.filter(p => p.assignedSongsCount >= 5).length}</span>
      <span class="summary-label text-red">Fatigue Alerts (5+ Songs)</span>
    </div>
    <div class="summary-item">
      <span class="summary-count">{roster.filter(p => p.assignedSongsCount >= 3 && p.assignedSongsCount < 5).length}</span>
      <span class="summary-label text-yellow">Moderate Workload (3-4 Songs)</span>
    </div>
    <div class="summary-item">
      <span class="summary-count">{roster.filter(p => p.assignedSongsCount < 3).length}</span>
      <span class="summary-label text-green">Optimal Workload (1-2 Songs)</span>
    </div>
  </div>

  <div class="roster-grid">
    {#each roster as person (person.id)}
      {@const workload = getWorkloadStatus(person.assignedSongsCount)}
      <div class="person-card bento-card">
        <div class="person-header">
          <div class="person-avatar">{person.name[0]}</div>
          <div class="person-info">
            <div class="person-name">{person.name}</div>
            <div class="person-email"><Mail size={12} /> {person.email}</div>
          </div>
          {#if person.role === 'DM'}
            <span class="role-pill badge-dm">Delivery Mgr</span>
          {:else if person.role === 'PM'}
            <span class="role-pill badge-pm">Performance Mgr</span>
          {:else}
            <span class="role-pill badge-performer">Performer</span>
          {/if}
        </div>

        <div class="person-details">
          <div class="detail-item">
            <Music size={13} class="text-orange" />
            <span>Role: {person.instrument}</span>
          </div>
          <div class="detail-item">
            <UserCheck size={13} class="text-green" />
            <span>Assigned to {person.assignedSongsCount} Music Numbers ({person.totalPracticeHours}h practice)</span>
          </div>
          <div class="workload-badge {workload.colorClass}">
            {#if workload.level === 'fatigued'}
              <AlertTriangle size={13} />
            {/if}
            <span>{$tStore(workload.key)}</span>
          </div>

          <div class="card-actions">
            <button
              type="button"
              class="bento-btn bento-btn-sm"
              onclick={() => openEditModal(person)}
            >
              <span>Edit</span>
            </button>
            <button
              type="button"
              class="bento-btn bento-btn-sm btn-danger"
              onclick={() => handleRemoveMember(person.id)}
            >
              <span>Remove</span>
            </button>
          </div>
        </div>
      </div>
    {/each}
  </div>
</div>

<!-- Modal: Add / Edit Roster Member -->
{#if isAddModalOpen}
  <div class="modal-backdrop" onclick={() => (isAddModalOpen = false)} role="presentation">
    <div class="modal-card bento-card" onclick={(e) => e.stopPropagation()} role="dialog">
      <h2>{editingPerson ? $tStore('show_mgmt.roster_modal.title_edit') : $tStore('show_mgmt.roster_modal.title_add')}</h2>

      <form onsubmit={handleSaveMember} class="modal-form">
        <div class="form-group">
          <label for="member-name">Member Name</label>
          <input
            id="member-name"
            type="text"
            bind:value={formName}
            placeholder="e.g. Minh Pháp"
            required
            class="form-input"
          />
        </div>

        <div class="form-group">
          <label for="member-email">Email Address</label>
          <input
            id="member-email"
            type="email"
            bind:value={formEmail}
            placeholder="e.g. minhphap@csac.local"
            class="form-input"
          />
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="member-role">{$tStore('show_mgmt.roster_modal.select_role')}</label>
            <select id="member-role" bind:value={formRole} class="form-input">
              <option value="DM">Delivery Manager (DM)</option>
              <option value="PM">Performance Manager (PM)</option>
              <option value="Performer">Performer</option>
            </select>
          </div>

          <div class="form-group">
            <label for="member-inst">{$tStore('show_mgmt.roster_modal.select_instrument')}</label>
            <input
              id="member-inst"
              type="text"
              bind:value={formInstrument}
              placeholder="e.g. Lead Vocal / Acoustic Guitar"
              required
              class="form-input"
            />
          </div>
        </div>

        <div class="modal-actions">
          <button type="button" class="bento-btn" onclick={() => (isAddModalOpen = false)}>
            Cancel
          </button>
          <button type="submit" class="bento-btn bento-btn-primary">
            {$tStore('show_mgmt.roster_modal.btn_submit')}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}


<style>
  .roster-subpage {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .bento-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 16px;
    padding: 20px;
  }

  .header-bar h2 {
    font-size: 18px;
    font-weight: 800;
    margin: 0 0 4px 0;
  }

  .header-bar p {
    font-size: 13px;
    color: #64748b;
    margin: 0;
  }

  .workload-summary {
    display: flex;
    justify-content: space-around;
    padding: 14px 20px;
    background: #f8fafc;
  }

  .summary-item {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .summary-count {
    font-size: 20px;
    font-weight: 800;
    color: #0f172a;
  }

  .summary-label {
    font-size: 12px;
    font-weight: 600;
  }

  .summary-label.text-red { color: #ef4444; }
  .summary-label.text-yellow { color: #d97706; }
  .summary-label.text-green { color: #16a34a; }

  .roster-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 16px;
  }

  .person-card {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .person-header {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .person-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: #f1f5f9;
    color: #0f172a;
    font-size: 16px;
    font-weight: 800;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .person-info { flex: 1; }

  .person-name {
    font-size: 15px;
    font-weight: 700;
    color: #0f172a;
  }

  .person-email {
    font-size: 12px;
    color: #64748b;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .role-pill {
    padding: 4px 8px;
    border-radius: 8px;
    font-size: 11px;
    font-weight: 700;
  }

  .badge-dm { background: rgba(255, 107, 0, 0.1); color: #ff6b00; }
  .badge-pm { background: rgba(59, 130, 246, 0.1); color: #2563eb; }
  .badge-performer { background: #f1f5f9; color: #475569; }

  .person-details {
    display: flex;
    flex-direction: column;
    gap: 8px;
    background: #f8fafc;
    padding: 10px;
    border-radius: 8px;
    font-size: 12px;
  }

  .detail-item {
    display: flex;
    align-items: center;
    gap: 6px;
    color: #334155;
  }

  .workload-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 8px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 700;
    width: fit-content;
  }

  .workload-green { background: rgba(22, 163, 74, 0.1); color: #16a34a; }
  .workload-yellow { background: rgba(217, 119, 6, 0.1); color: #d97706; }
  .workload-red { background: rgba(239, 68, 68, 0.1); color: #ef4444; }

  .card-actions {
    display: flex;
    gap: 8px;
    margin-top: 6px;
  }

  .btn-danger {
    background: #fef2f2;
    color: #dc2626;
    border-color: #fecaca;
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
    max-width: 480px;
  }

  .modal-card h2 {
    font-size: 18px;
    font-weight: 800;
    margin: 0 0 16px 0;
  }

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-group label {
    font-size: 13px;
    font-weight: 600;
    color: #334155;
  }

  .form-input {
    padding: 8px 12px;
    border: 1px solid #cbd5e1;
    border-radius: 8px;
    font-size: 13px;
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 10px;
  }
</style>


