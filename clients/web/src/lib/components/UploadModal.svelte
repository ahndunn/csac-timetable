<script lang="ts">
  import type { SongVoteData } from '../types/timetable';
  import { inspectExcelFiles, parseSelectedSheets, type FileInspection } from '../engine/excelParser';
  import SheetSelectionModal from './SheetSelectionModal.svelte';
  import { generateMultiTabSampleFile, generateSingleTabSampleFiles } from '../engine/sampleData';
  import { UploadCloud, FileSpreadsheet, Check, X, AlertCircle, Layers, Files } from '@lucide/svelte';
  import { tStore } from '$lib/i18n';

  interface Props {
    onClose: () => void;
    onAddSongs: (newSongs: SongVoteData[]) => void;
    onDownloadTemplate?: () => void;
    existingCount: number;
  }

  let { onClose, onAddSongs, onDownloadTemplate, existingCount }: Props = $props();

  let isDragging = $state(false);
  let isLoading = $state(false);
  let errorMsg = $state<string | null>(null);
  let parsedSongs = $state<SongVoteData[]>([]);
  let multiTabInspections = $state<FileInspection[] | null>(null);

  async function handleFiles(files: FileList | File[] | null) {
    if (!files || files.length === 0) return;
    isLoading = true;
    errorMsg = null;

    try {
      const fileList = Array.from(files).filter(
        f => f.name.endsWith('.xlsx') || f.name.endsWith('.xls')
      );

      if (fileList.length === 0) {
        errorMsg = $tStore('upload_modal.error_extension');
        isLoading = false;
        return;
      }

      const inspections = await inspectExcelFiles(fileList);
      const hasMultiTab = inspections.some(f => f.hasMultipleSheets);

      if (hasMultiTab) {
        multiTabInspections = inspections;
      } else {
        const allKeys = new Set<string>();
        for (const f of inspections) {
          for (const s of f.sheets) {
            if (s.isValid) allKeys.add(`${f.fileId}::${s.sheetName}`);
          }
        }
        const songs = parseSelectedSheets(inspections, allKeys, existingCount + parsedSongs.length);
        if (songs.length === 0) {
          errorMsg = $tStore('upload_modal.error_no_valid');
        } else {
          parsedSongs = [...parsedSongs, ...songs];
        }
      }
    } catch (err) {
      console.error(err);
      errorMsg = $tStore('upload_modal.error_read');
    } finally {
      isLoading = false;
    }
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
    if (e.dataTransfer?.files) {
      handleFiles(e.dataTransfer.files);
    }
  }

  function handleFileInputChange(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files) {
      handleFiles(target.files);
    }
  }

  function handleConfirm() {
    if (parsedSongs.length > 0) {
      onAddSongs(parsedSongs);
      onClose();
    }
  }

  function handleTestMultiTab() {
    const file = generateMultiTabSampleFile();
    handleFiles([file]);
  }

  function handleTestSingleTab() {
    const files = generateSingleTabSampleFiles();
    handleFiles(files);
  }

  function handleConfirmMultiTab(selectedSongs: SongVoteData[]) {
    parsedSongs = [...parsedSongs, ...selectedSongs];
    multiTabInspections = null;
  }
</script>

<!-- Sheet Selection Modal if multi-tab file detected -->
{#if multiTabInspections}
  <SheetSelectionModal
    inspections={multiTabInspections}
    existingCount={existingCount + parsedSongs.length}
    onClose={() => multiTabInspections = null}
    onConfirm={handleConfirmMultiTab}
  />
{/if}

<div
  class="modal-overlay"
  onclick={onClose}
  onkeydown={(e) => { if (e.key === 'Escape') onClose(); }}
  role="presentation"
>
  <div
    class="modal-dialog"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="modal-header">
      <div style="display: flex; align-items: center; gap: 8px;">
        <UploadCloud size={20} color="var(--accent)" />
        <h3 class="modal-header-title">{$tStore('upload_modal.title')}</h3>
      </div>
      <button type="button" class="modal-close-btn" onclick={onClose} aria-label={$tStore('upload_modal.cancel')}>
        <X size={16} />
      </button>
    </div>

    <div class="modal-body">
      <!-- Drag and drop zone -->
      <div
        class="upload-dropzone {isDragging ? 'is-dragging' : ''}"
        ondragover={(e) => { e.preventDefault(); isDragging = true; }}
        ondragleave={() => isDragging = false}
        ondrop={handleDrop}
        onclick={() => document.getElementById('excel-file-input')?.click()}
        onkeydown={(e) => { if (e.key === 'Enter') document.getElementById('excel-file-input')?.click(); }}
        role="button"
        tabindex="0"
      >
        <input
          type="file"
          id="excel-file-input"
          multiple
          accept=".xlsx, .xls"
          style="display: none;"
          onchange={handleFileInputChange}
        />

        <div class="dropzone-icon-well">
          <UploadCloud size={26} />
        </div>

        <div>
          <div style="font-size: 15px; font-weight: 700; color: var(--text-primary);">
            {$tStore('upload_modal.drop_title')} <span style="color: var(--accent); text-decoration: underline;">{$tStore('upload_modal.browse')}</span>
          </div>
          <div style="font-size: 12px; color: var(--text-muted); margin-top: 4px;">
            {$tStore('upload_modal.drop_subtitle')}
          </div>
        </div>

        {#if isLoading}
          <div class="bento-pill is-active">
            <span>{$tStore('upload_modal.analyzing')}</span>
          </div>
        {/if}
      </div>

      <!-- Quick sample test buttons -->
      <div style="padding: 14px; background: var(--surface-card-subtle); border: 1px solid var(--border-card); border-radius: var(--radius-sm); display: flex; flex-direction: column; gap: 10px;">
        <div style="font-size: 12px; font-weight: 700; color: var(--text-secondary);">
          {$tStore('upload_modal.quick_test')}
        </div>
        <div style="display: flex; gap: 8px; flex-wrap: wrap;">
          <button
            type="button"
            class="bento-btn"
            style="font-size: 12px; padding: 6px 12px;"
            onclick={handleTestMultiTab}
          >
            <Layers size={14} color="var(--accent)" />
            <span>{$tStore('upload_modal.test_multitab')}</span>
          </button>
          <button
            type="button"
            class="bento-btn"
            style="font-size: 12px; padding: 6px 12px;"
            onclick={handleTestSingleTab}
          >
            <Files size={14} color="var(--accent)" />
            <span>{$tStore('upload_modal.test_singletab')}</span>
          </button>
          {#if onDownloadTemplate}
            <button
              type="button"
              class="bento-btn"
              style="font-size: 12px; padding: 6px 12px;"
              onclick={onDownloadTemplate}
            >
              <FileSpreadsheet size={14} color="var(--success)" />
              <span>{$tStore('upload_modal.download_template')}</span>
            </button>
          {/if}
        </div>
      </div>

      <!-- Error alert -->
      {#if errorMsg}
        <div style="padding: 12px; background: var(--danger-light); border: 1px solid rgba(239, 68, 68, 0.3); border-radius: var(--radius-sm); display: flex; align-items: center; gap: 8px; color: var(--danger-text);">
          <AlertCircle size={16} />
          <span style="font-size: 12px; font-weight: 600;">{errorMsg}</span>
        </div>
      {/if}

      <!-- Successfully parsed songs list preview -->
      {#if parsedSongs.length > 0}
        <div>
          <div style="font-size: 13px; font-weight: 700; color: var(--text-primary); margin-bottom: 8px;">
            {$tStore('upload_modal.found_songs', { count: parsedSongs.length })}
          </div>
          <div style="display: flex; flex-direction: column; gap: 6px; max-height: 180px; overflow-y: auto;">
            {#each parsedSongs as song (song.id)}
              <div class="bento-card" style="display: flex; align-items: center; justify-content: space-between; padding: 8px 12px;">
                <div style="display: flex; align-items: center; gap: 8px;">
                  <div
                    style="width: 10px; height: 10px; border-radius: var(--radius-circle); background-color: {song.color.border};"
                  ></div>
                  <strong style="font-size: 13px; color: var(--text-primary);">{song.name}</strong>
                  <span style="font-size: 11px; color: var(--text-muted);">
                    {$tStore('upload_modal.members_count', { count: song.members.length })}
                  </span>
                </div>
                <span class="bento-pill is-active" style="font-size: 10px; padding: 2px 6px;">
                  {$tStore('upload_modal.sessions_tag', { count: song.targetSessions })}
                </span>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <div class="modal-footer">
      <button type="button" class="bento-btn" onclick={onClose}>
        {$tStore('upload_modal.cancel')}
      </button>
      <button
        type="button"
        class="bento-btn bento-btn-primary"
        disabled={parsedSongs.length === 0}
        onclick={handleConfirm}
      >
        <Check size={16} />
        <span>{$tStore('upload_modal.confirm_import', { count: parsedSongs.length })}</span>
      </button>
    </div>
  </div>
</div>
