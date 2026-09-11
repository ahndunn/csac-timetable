<script lang="ts">
  import { tStore } from '$lib/i18n';
  import {
    Calendar,
    Clock,
    Wand2,
    Check,
    AlertCircle,
    CheckCircle2,
  } from '@lucide/svelte';

  let days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];
  let slots = ['17:00 - 18:00', '18:00 - 19:00', '19:00 - 20:00', '20:00 - 21:00'];

  // Matrix state: dayIdx_slotIdx -> boolean
  let selectedSlots = $state<Record<string, boolean>>({
    '0_0': true,
    '0_1': true,
    '2_2': true,
    '4_1': true,
    '4_2': true,
    '5_0': true,
  });

  let isSaved = $state(false);

  function toggleSlot(dayIdx: number, slotIdx: number) {
    const key = `${dayIdx}_${slotIdx}`;
    selectedSlots[key] = !selectedSlots[key];
    isSaved = false;
  }

  function handleSaveFreetime() {
    isSaved = true;
    setTimeout(() => (isSaved = false), 3000);
  }
</script>

<div class="sprints-subpage">
  <!-- Active Sprint Banner -->
  <div class="sprint-header bento-card">
    <div class="header-info">
      <div class="sprint-tag">
        <Calendar size={14} class="text-orange" />
        <span>Active Sprint: Sprint 3 (Stage QC & Rehearsal Optimization)</span>
      </div>
      <h2>Practice Sprint Management & Performer Availability</h2>
      <p>Click time slots below to register your 1-click free-time availability for active show numbers.</p>
    </div>

    <button type="button" class="bento-btn bento-btn-primary">
      <Wand2 size={16} />
      <span>Auto-Schedule Sprint Rehearsals</span>
    </button>
  </div>

  {#if isSaved}
    <div class="toast-success">
      <CheckCircle2 size={16} />
      <span>{$tStore('studio.freetime_saved')}</span>
    </div>
  {/if}

  <!-- 1-Click Weekly Free-Time Grid -->
  <div class="grid-card bento-card">
    <div class="grid-title-row">
      <h3>1-Click Sprint Free-Time Registration Grid</h3>
      <button type="button" class="bento-btn bento-btn-sm" onclick={handleSaveFreetime}>
        <Check size={14} />
        <span>Save Availability</span>
      </button>
    </div>

    <div class="freetime-table-wrapper">
      <table class="freetime-table">
        <thead>
          <tr>
            <th>Time Slot</th>
            {#each days as day}
              <th>{day}</th>
            {/each}
          </tr>
        </thead>
        <tbody>
          {#each slots as slot, sIdx}
            <tr>
              <td class="slot-label"><Clock size={13} /> {slot}</td>
              {#each days as day, dIdx}
                {@const isSelected = selectedSlots[`${dIdx}_${sIdx}`]}
                <td class="slot-cell-td">
                  <button
                    type="button"
                    class="slot-cell-btn {isSelected ? 'is-selected' : ''}"
                    onclick={() => toggleSlot(dIdx, sIdx)}
                  >
                    {#if isSelected}
                      <Check size={14} />
                      <span>Free</span>
                    {:else}
                      <span class="text-muted">Busy</span>
                    {/if}
                  </button>
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>

<style>
  .sprints-subpage {
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

  .sprint-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .sprint-tag {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    border-radius: 20px;
    font-size: 12px;
    font-weight: 700;
    margin-bottom: 6px;
  }

  .sprint-header h2 {
    font-size: 18px;
    font-weight: 800;
    color: #0f172a;
    margin: 0 0 4px 0;
  }

  .sprint-header p {
    font-size: 13px;
    color: #64748b;
    margin: 0;
  }

  .toast-success {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(22, 163, 74, 0.1);
    color: #16a34a;
    padding: 10px 16px;
    border-radius: 10px;
    font-weight: 700;
    font-size: 13px;
  }

  .grid-title-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 14px;
  }

  .grid-title-row h3 {
    font-size: 15px;
    font-weight: 700;
    margin: 0;
  }

  .freetime-table-wrapper {
    overflow-x: auto;
  }

  .freetime-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
  }

  .freetime-table th {
    background: #f8fafc;
    padding: 10px;
    text-align: center;
    border-bottom: 1px solid #e2e8f0;
    font-weight: 700;
    color: #475569;
  }

  .slot-label {
    font-weight: 700;
    color: #334155;
    white-space: nowrap;
  }

  .freetime-table td {
    padding: 6px;
    border-bottom: 1px solid #f1f5f9;
    text-align: center;
  }

  .slot-cell-btn {
    width: 100%;
    padding: 12px 6px;
    border: 1px dashed #cbd5e1;
    border-radius: 8px;
    background: #ffffff;
    font-size: 12px;
    font-weight: 600;
    color: #94a3b8;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    transition: all 0.15s ease;
  }

  .slot-cell-btn.is-selected {
    background: rgba(22, 163, 74, 0.1);
    border: 1px solid #16a34a;
    color: #16a34a;
    font-weight: 700;
  }
</style>
