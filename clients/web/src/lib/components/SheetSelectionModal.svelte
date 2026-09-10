<script lang="ts">
  import type { FileInspection } from '../engine/excelParser';
  import { parseSelectedSheets } from '../engine/excelParser';
  import type { SongVoteData } from '../types/timetable';
  import {
    Layers,
    FileSpreadsheet,
    Check,
    X,
    Search,
    CheckSquare,
    Square,
    Users,
    Info,
  } from '@lucide/svelte';
  import { tStore } from '$lib/i18n';

  interface Props {
    inspections: FileInspection[];
    existingCount: number;
    onClose: () => void;
    onConfirm: (selectedSongs: SongVoteData[]) => void;
  }

  let { inspections, existingCount, onClose, onConfirm }: Props = $props();

  let allValidSheetKeys = $derived.by(() => {
    const keys: string[] = [];
    for (const f of inspections) {
      for (const s of f.sheets) {
        if (s.isValid) {
          keys.push(`${f.fileId}::${s.sheetName}`);
        }
      }
    }
    return keys;
  });

  let selectedKeys = $state<Set<string>>(new Set());

  $effect(() => {
    selectedKeys = new Set(allValidSheetKeys);
  });

  let searchQuery = $state('');

  let filteredFiles = $derived.by(() => {
    const q = searchQuery.toLowerCase().trim();
    if (!q) return inspections;

    return inspections
      .map(file => {
        const matchingSheets = file.sheets.filter(s => {
          const matchTab = s.sheetName.toLowerCase().includes(q);
          const matchSong = s.songName.toLowerCase().includes(q);
          const matchMember = s.members.some(m => m.toLowerCase().includes(q));
          return matchTab || matchSong || matchMember;
        });

        return {
          ...file,
          sheets: matchingSheets,
        };
      })
      .filter(f => f.sheets.length > 0);
  });

  function handleToggleSheet(key: string) {
    const next = new Set(selectedKeys);
    if (next.has(key)) {
      next.delete(key);
    } else {
      next.add(key);
    }
    selectedKeys = next;
  }

  function handleToggleFile(file: FileInspection) {
    const validFileKeys = file.sheets.filter(s => s.isValid).map(s => `${file.fileId}::${s.sheetName}`);
    const allSelectedInFile = validFileKeys.every(k => selectedKeys.has(k));
    const next = new Set(selectedKeys);

    if (allSelectedInFile) {
      validFileKeys.forEach(k => next.delete(k));
    } else {
      validFileKeys.forEach(k => next.add(k));
    }
    selectedKeys = next;
  }

  function handleSelectAll() {
    selectedKeys = new Set(allValidSheetKeys);
  }

  function handleDeselectAll() {
    selectedKeys = new Set();
  }

  function handleConfirm() {
    const songs = parseSelectedSheets(inspections, selectedKeys, existingCount);
    onConfirm(songs);
    onClose();
  }

  let selectedCount = $derived(selectedKeys.size);
  let multiTabFilesCount = $derived(inspections.filter(f => f.hasMultipleSheets).length);
</script>

<div
  class="modal-overlay"
  onclick={onClose}
  onkeydown={(e) => { if (e.key === 'Escape') onClose(); }}
  role="presentation"
>
  <div
    class="modal-dialog sheet-selection-modal"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- Header -->
    <div class="modal-header">
      <div style="display: flex; align-items: center; gap: 10px;">
        <div class="dropzone-icon-well" style="width: 38px; height: 38px;">
          <Layers size={18} color="var(--accent)" />
        </div>
        <div>
          <h3 class="modal-header-title">{$tStore('sheet_modal.title')}</h3>
          <p style="margin: 0; font-size: 12px; color: var(--text-muted);">
            {$tStore('sheet_modal.subtitle', {
              files: multiTabFilesCount > 0 
                ? $tStore('sheet_modal.files_multitab', { count: multiTabFilesCount })
                : $tStore('sheet_modal.files_generic')
            })}
          </p>
        </div>
      </div>
      <button type="button" class="modal-close-btn" onclick={onClose} aria-label={$tStore('sheet_modal.cancel')}>
        <X size={16} />
      </button>
    </div>

    <!-- Action Toolbar -->
    <div class="sheet-selection-toolbar">
      <div class="sheet-search-box">
        <Search size={14} class="search-icon" />
        <input
          type="text"
          placeholder={$tStore('sheet_modal.search_placeholder')}
          value={searchQuery}
          oninput={(e) => searchQuery = (e.target as HTMLInputElement).value}
        />
        {#if searchQuery}
          <button
            type="button"
            class="search-clear-btn"
            onclick={() => searchQuery = ''}
            aria-label={$tStore('sidebar.clear_filter')}
          >
            <X size={12} />
          </button>
        {/if}
      </div>

      <div style="display: flex; align-items: center; gap: 10px;">
        <span class="bento-pill is-active" style="font-size: 11px;">
          {$tStore('sheet_modal.selected_count', { selected: selectedCount, total: allValidSheetKeys.length })}
        </span>
        <button
          type="button"
          class="bento-btn"
          style="font-size: 11px; padding: 4px 10px;"
          onclick={handleSelectAll}
        >
          {$tStore('sheet_modal.select_all')}
        </button>
        <button
          type="button"
          class="bento-btn"
          style="font-size: 11px; padding: 4px 10px;"
          onclick={handleDeselectAll}
        >
          {$tStore('sheet_modal.deselect_all')}
        </button>
      </div>
    </div>

    <!-- Modal Body: Grouped List by File -->
    <div class="modal-body sheet-selection-body">
      {#if filteredFiles.length === 0}
        <div style="text-align: center; padding: 24px; color: var(--text-muted);">
          <Info size={28} />
          <p style="margin-top: 8px;">{$tStore('sheet_modal.no_match', { query: searchQuery })}</p>
        </div>
      {:else}
        {#each filteredFiles as file (file.fileId)}
          {@const validFileKeys = file.sheets.filter(s => s.isValid).map(s => `${file.fileId}::${s.sheetName}`)}
          {@const selectedInFileCount = validFileKeys.filter(k => selectedKeys.has(k)).length}
          {@const allSelectedInFile = validFileKeys.length > 0 && selectedInFileCount === validFileKeys.length}

          <div class="file-inspection-group">
            <!-- File Header Card -->
            <div class="file-inspection-header">
              <div style="display: flex; align-items: center; gap: 8px;">
                <FileSpreadsheet size={18} color="var(--success)" />
                <strong style="font-size: 13px; color: var(--text-primary);" title={file.fileName}>
                  {file.fileName}
                </strong>
                <span class="bento-pill is-active" style="font-size: 10px; padding: 1px 6px;">
                  {$tStore('sheet_modal.file_tabs', { count: file.sheets.length })} {file.hasMultipleSheets ? $tStore('sheet_modal.multitab_badge') : ''}
                </span>
              </div>

              <div style="display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 11px; color: var(--text-muted);">
                  {$tStore('sheet_modal.selected_file_ratio', { selected: selectedInFileCount, total: validFileKeys.length })}
                </span>
                <button
                  type="button"
                  class="bento-btn"
                  style="font-size: 11px; padding: 3px 8px;"
                  onclick={() => handleToggleFile(file)}
                >
                  {allSelectedInFile ? $tStore('sheet_modal.deselect_file') : $tStore('sheet_modal.select_file')}
                </button>
              </div>
            </div>

            <!-- Tabs Grid in this File -->
            <div class="file-sheets-grid">
              {#each file.sheets as sheet (sheet.sheetName)}
                {@const key = `${file.fileId}::${sheet.sheetName}`}
                {@const isSelected = selectedKeys.has(key)}

                <div
                  class="sheet-item-card {isSelected ? 'selected' : ''} {!sheet.isValid ? 'invalid' : ''}"
                  onclick={() => {
                    if (sheet.isValid) handleToggleSheet(key);
                  }}
                  onkeydown={(e) => {
                    if (e.key === 'Enter' && sheet.isValid) handleToggleSheet(key);
                  }}
                  role="button"
                  tabindex={sheet.isValid ? 0 : -1}
                >
                  <div style="margin-top: 2px;">
                    {#if isSelected}
                      <CheckSquare size={16} color="var(--accent)" />
                    {:else}
                      <Square size={16} color="var(--text-muted)" />
                    {/if}
                  </div>

                  <div style="display: flex; flex-direction: column; gap: 3px; flex: 1; min-width: 0;">
                    <div style="display: flex; align-items: center; justify-content: space-between; gap: 4px;">
                      <span style="font-size: 12px; font-weight: 700; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                        {$tStore('sheet_modal.tab_label', { name: sheet.sheetName })}
                      </span>
                      {#if sheet.isValid}
                        <span class="bento-pill" style="font-size: 9px; padding: 1px 5px;">
                          <Users size={10} />
                          <span>{sheet.members.length}</span>
                        </span>
                      {:else}
                        <span style="font-size: 10px; color: var(--danger-text); font-weight: 600;">
                          {$tStore('sheet_modal.invalid_format')}
                        </span>
                      {/if}
                    </div>

                    <div style="font-size: 11px; color: var(--text-secondary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                      {$tStore('sheet_modal.song_label', { name: sheet.songName })}
                    </div>

                    {#if sheet.members.length > 0}
                      <div style="font-size: 10px; color: var(--text-muted); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                        ({sheet.members.slice(0, 3).join(', ')}{sheet.members.length > 3 ? ` +${sheet.members.length - 3}` : ''})
                      </div>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <!-- Footer -->
    <div class="modal-footer" style="justify-content: space-between;">
      <div style="font-size: 12px; color: var(--text-muted); display: flex; align-items: center; gap: 6px;">
        <Info size={14} />
        <span>{$tStore('sheet_modal.unselected_hint')}</span>
      </div>

      <div style="display: flex; align-items: center; gap: 10px;">
        <button type="button" class="bento-btn" onclick={onClose}>
          {$tStore('sheet_modal.cancel')}
        </button>
        <button
          type="button"
          class="bento-btn bento-btn-primary"
          onclick={handleConfirm}
          disabled={selectedCount === 0}
        >
          <Check size={16} />
          <span>{$tStore('sheet_modal.confirm', { count: selectedCount })}</span>
        </button>
      </div>
    </div>
  </div>
</div>
