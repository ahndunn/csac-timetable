<script lang="ts">
  import type { FileInspection } from '../engine/excelParser';
  import { parseSelectedSheets } from '../engine/excelParser';
  import type { SongVoteData } from '../types/timetable';
  import {
    Layers,
    FileSpreadsheet,
    CheckSquare,
    Square,
    Search,
    Check,
    X,
    Users,
    AlertCircle,
    Info,
  } from '@lucide/svelte';

  interface Props {
    inspections: FileInspection[];
    existingCount: number;
    onClose: () => void;
    onConfirm: (selectedSongs: SongVoteData[]) => void;
  }

  let { inspections, existingCount, onClose, onConfirm }: Props = $props();

  function getInitialKeys() {
    const initial = new Set<string>();
    for (const file of inspections) {
      for (const sheet of file.sheets) {
        if (sheet.isValid) {
          initial.add(`${file.fileId}::${sheet.sheetName}`);
        }
      }
    }
    return initial;
  }

  let selectedKeys = $state<Set<string>>(getInitialKeys());

  let searchQuery = $state('');

  let allValidSheetKeys = $derived.by(() => {
    const keys: string[] = [];
    for (const file of inspections) {
      for (const sheet of file.sheets) {
        if (sheet.isValid) {
          keys.push(`${file.fileId}::${sheet.sheetName}`);
        }
      }
    }
    return keys;
  });

  let filteredFiles = $derived.by(() => {
    if (!searchQuery.trim()) return inspections;
    const q = searchQuery.toLowerCase();
    return inspections
      .map(file => ({
        ...file,
        sheets: file.sheets.filter(s =>
          s.songName.toLowerCase().includes(q) ||
          s.sheetName.toLowerCase().includes(q) ||
          s.members.some(m => m.toLowerCase().includes(q))
        )
      }))
      .filter(file => file.sheets.length > 0);
  });

  let selectedCount = $derived(
    Array.from(selectedKeys).filter(k => allValidSheetKeys.includes(k)).length
  );

  function toggleSheet(key: string) {
    const next = new Set(selectedKeys);
    if (next.has(key)) {
      next.delete(key);
    } else {
      next.add(key);
    }
    selectedKeys = next;
  }

  function toggleAll() {
    if (selectedCount === allValidSheetKeys.length) {
      selectedKeys = new Set();
    } else {
      selectedKeys = new Set(allValidSheetKeys);
    }
  }

  function handleImport() {
    const songs = parseSelectedSheets(inspections, selectedKeys, existingCount);
    onConfirm(songs);
  }
</script>

<div class="modal-overlay" onclick={onClose} role="presentation">
  <div
    class="modal-dialog"
    style="max-width: 680px;"
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
  >
    <div class="modal-header">
      <div style="display: flex; align-items: center; gap: 8px;">
        <Layers size={20} color="#1a73e8" />
        <h3 class="modal-header-title">Chọn các Tab / Sheet để nhập lịch</h3>
      </div>
      <button type="button" class="modal-close-btn" onclick={onClose} aria-label="Đóng">
        <X size={18} />
      </button>
    </div>

    <div class="modal-body" style="gap: 12px;">
      <div style="display: flex; justify-content: space-between; align-items: center; gap: 12px;">
        <div style="position: relative; flex: 1;">
          <Search size={16} color="#9ca3af" style="position: absolute; left: 10px; top: 10px;" />
          <input
            type="text"
            placeholder="Tìm theo tên bài hát hoặc thành viên..."
            value={searchQuery}
            oninput={(e) => searchQuery = (e.target as HTMLInputElement).value}
            class="input-gcal"
            style="padding-left: 34px; width: 100%;"
          />
        </div>

        <button
          type="button"
          class="btn-gcal-secondary"
          onclick={toggleAll}
          style="font-size: 13px; white-space: nowrap;"
        >
          {selectedCount === allValidSheetKeys.length ? 'Bỏ chọn tất cả' : 'Chọn tất cả'}
        </button>
      </div>

      <div style="max-height: 380px; overflow-y: auto; display: flex; flex-direction: column; gap: 8px;">
        {#each filteredFiles as file (file.fileId)}
          <div style="border: 1px solid #e5e7eb; border-radius: 8px; padding: 10px; background-color: #fafafa;">
            <div style="font-size: 12px; font-weight: 600; color: #6b7280; margin-bottom: 6px; display: flex; align-items: center; gap: 6px;">
              <FileSpreadsheet size={14} />
              <span>{file.fileName} ({file.sheets.length} sheets)</span>
            </div>

            <div style="display: flex; flex-direction: column; gap: 6px;">
              {#each file.sheets as sheet (sheet.sheetName)}
                {@const key = `${file.fileId}::${sheet.sheetName}`}
                {@const isChecked = selectedKeys.has(key)}

                <div
                  style="display: flex; align-items: center; justify-content: space-between; padding: 8px 10px; border-radius: 6px; background-color: #ffffff; border: 1px solid #f3f4f6; cursor: {sheet.isValid ? 'pointer' : 'not-allowed'}; opacity: {sheet.isValid ? 1 : 0.6};"
                  onclick={() => { if (sheet.isValid) toggleSheet(key); }}
                  role="presentation"
                >
                  <div style="display: flex; align-items: center; gap: 10px;">
                    {#if sheet.isValid}
                      {#if isChecked}
                        <CheckSquare size={16} color="#1a73e8" />
                      {:else}
                        <Square size={16} color="#9ca3af" />
                      {/if}
                    {:else}
                      <AlertCircle size={16} color="#ef4444" />
                    {/if}

                    <div>
                      <strong style="font-size: 13px; color: #1f2937;">{sheet.songName}</strong>
                      <div style="font-size: 11px; color: #6b7280;">
                        Sheet: {sheet.sheetName} | 👥 {sheet.members.length} người: {sheet.members.slice(0, 4).join(', ')}{sheet.members.length > 4 ? '...' : ''}
                      </div>
                    </div>
                  </div>

                  {#if !sheet.isValid}
                    <span style="font-size: 11px; color: #ef4444; font-weight: 500;">
                      Không đúng mẫu vote
                    </span>
                  {/if}

                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    </div>

    <div class="modal-footer">
      <button type="button" class="btn-gcal-secondary" onclick={onClose}>
        Hủy
      </button>
      <button
        type="button"
        class="btn-gcal-primary"
        disabled={selectedCount === 0}
        onclick={handleImport}
      >
        <Check size={16} />
        <span>Nhập {selectedCount} bài hát đã chọn</span>
      </button>
    </div>
  </div>
</div>
