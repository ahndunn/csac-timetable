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
        </div>
      </div>
    {/each}
  </div>
</div>

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
</style>

