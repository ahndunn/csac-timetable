<script lang="ts">
  import { tStore } from '$lib/i18n';
  import { Zap, Clock, Trash2, Check } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';

  interface TimeSlotInfo {
    label: string;
    hour: number;
    minute: string;
    isHourStart: boolean;
  }

  interface Props {
    days: string[];
    timeSlots: TimeSlotInfo[];
    selectedSlots: Record<string, boolean>;
    onSlotChange: (updated: Record<string, boolean>) => void;
    onSave: () => void;
  }

  let { days, timeSlots, selectedSlots, onSlotChange, onSave }: Props = $props();

  // Computed count of selected 15-min slots
  let selectedCount = $derived(
    Object.values(selectedSlots).filter(Boolean).length
  );
  let totalHoursFormatted = $derived((selectedCount * 0.25).toFixed(2));

  // Drag interaction state
  let isDragging = $state(false);
  let dragTargetValue = $state(true);
  let dragStart = $state<{ dayIdx: number; slotIdx: number } | null>(null);
  let initialSelectedSlotsSnapshot = $state<Record<string, boolean>>({});

  function handleCellMouseDown(dayIdx: number, slotIdx: number, event: MouseEvent) {
    event.preventDefault();
    isDragging = true;
    dragStart = { dayIdx, slotIdx };
    initialSelectedSlotsSnapshot = { ...selectedSlots };

    const slotLabel = timeSlots[slotIdx].label;
    const key = `${dayIdx}_${slotLabel}`;
    dragTargetValue = !selectedSlots[key];

    const updated = { ...selectedSlots, [key]: dragTargetValue };
    onSlotChange(updated);
  }

  function handleCellMouseEnter(dayIdx: number, slotIdx: number) {
    if (isDragging && dragStart) {
      const minDay = Math.min(dragStart.dayIdx, dayIdx);
      const maxDay = Math.max(dragStart.dayIdx, dayIdx);
      const minSlot = Math.min(dragStart.slotIdx, slotIdx);
      const maxSlot = Math.max(dragStart.slotIdx, slotIdx);

      const updated = { ...initialSelectedSlotsSnapshot };

      for (let d = minDay; d <= maxDay; d++) {
        for (let s = minSlot; s <= maxSlot; s++) {
          const label = timeSlots[s].label;
          updated[`${d}_${label}`] = dragTargetValue;
        }
      }

      onSlotChange(updated);
    }
  }

  function handleMouseUpGlobal() {
    isDragging = false;
    dragStart = null;
  }

  function selectPresetEvenings() {
    const updated = { ...selectedSlots };
    for (let d = 0; d < 7; d++) {
      for (const slot of timeSlots) {
        if (slot.hour >= 18 && slot.hour < 21) {
          updated[`${d}_${slot.label}`] = true;
        }
      }
    }
    onSlotChange(updated);
  }

  function selectPresetAfternoons() {
    const updated = { ...selectedSlots };
    for (let d = 0; d < 7; d++) {
      for (const slot of timeSlots) {
        if (slot.hour >= 17 && slot.hour < 19) {
          updated[`${d}_${slot.label}`] = true;
        }
      }
    }
    onSlotChange(updated);
  }

  function clearAllSlots() {
    onSlotChange({});
  }
</script>

<svelte:window onmouseup={handleMouseUpGlobal} />

<Card class="p-5 flex flex-col gap-4 shadow-sm">
  <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
    <div>
      <h3 class="text-base font-bold text-foreground m-0">{$tStore('studio.freetime_title_15m')}</h3>
      <p class="text-xs text-muted-foreground m-0 mt-0.5">
        Selected 15-min slots: <strong class="text-foreground">{selectedCount}</strong> ({totalHoursFormatted} total practice hours available)
      </p>
    </div>

    <div class="flex items-center gap-2 flex-wrap">
      <div class="flex items-center bg-muted/60 border border-border/60 rounded-lg p-1 gap-1">
        <Button variant="ghost" size="sm" class="h-7 px-2.5 text-xs gap-1.5 font-medium text-foreground hover:bg-background hover:shadow-2xs" onclick={selectPresetEvenings}>
          <Zap class="w-3.5 h-3.5 text-primary" />
          <span>{$tStore('studio.preset_evenings')}</span>
        </Button>
        <Button variant="ghost" size="sm" class="h-7 px-2.5 text-xs gap-1.5 font-medium text-foreground hover:bg-background hover:shadow-2xs" onclick={selectPresetAfternoons}>
          <Clock class="w-3.5 h-3.5 text-blue-600" />
          <span>{$tStore('studio.preset_afternoons')}</span>
        </Button>
        <Button variant="ghost" size="sm" class="h-7 px-2.5 text-xs gap-1.5 font-medium text-red-600 hover:text-red-700 hover:bg-red-500/10" onclick={clearAllSlots}>
          <Trash2 class="w-3.5 h-3.5" />
          <span>{$tStore('studio.preset_clear')}</span>
        </Button>
      </div>

      <Button size="sm" class="h-8 gap-1.5 bg-primary hover:bg-primary/90 text-primary-foreground font-semibold shadow-xs" onclick={onSave}>
        <Check class="w-3.5 h-3.5" />
        <span>{$tStore('studio.btn_save_freetime')}</span>
      </Button>
    </div>
  </div>

  <!-- 15-Minute Drag Matrix Table -->
  <div class="overflow-x-auto select-none border border-border rounded-xl max-h-[460px] bg-card shadow-xs">
    <table class="w-full border-collapse text-xs">
      <thead class="sticky top-0 bg-muted/90 backdrop-blur-xs z-10 border-b border-border">
        <tr>
          <th class="p-2 text-center font-bold text-muted-foreground w-20 border-r border-border">15m Slot</th>
          {#each days as day}
            <th class="p-2 text-center font-bold text-foreground border-r border-border last:border-r-0">{day}</th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each timeSlots as slot, sIdx}
          <tr class="hover:bg-muted/40 transition-colors {slot.isHourStart ? 'border-t-2 border-t-border' : 'border-t border-t-border/50'}">
            <td class="p-1.5 text-center font-mono text-[11px] border-r border-border {slot.isHourStart ? 'bg-muted/80 text-foreground font-bold' : 'text-muted-foreground'}">
              <span>{slot.label}</span>
            </td>
            {#each days as day, dIdx}
              {@const isSelected = selectedSlots[`${dIdx}_${slot.label}`]}
              <td
                class="p-0.5 border-r border-border/50 last:border-r-0 text-center cursor-pointer"
                onmousedown={(e) => handleCellMouseDown(dIdx, sIdx, e)}
                onmouseenter={() => handleCellMouseEnter(dIdx, sIdx)}
              >
                <div
                  class="w-full h-5 rounded-md transition-all duration-150 flex items-center justify-center {isSelected
                    ? 'bg-primary text-primary-foreground shadow-2xs'
                    : 'hover:bg-muted/60'}"
                >
                  {#if isSelected}
                    <span class="w-1.5 h-1.5 rounded-full bg-white ring-2 ring-primary/40"></span>
                  {/if}
                </div>
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</Card>
