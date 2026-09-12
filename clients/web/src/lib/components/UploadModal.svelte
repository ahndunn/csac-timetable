<script lang="ts">
  import type { SongVoteData } from '../types/timetable';
  import { inspectExcelFiles, parseSelectedSheets, type FileInspection } from '../engine/excelParser';
  import SheetSelectionModal from './SheetSelectionModal.svelte';
  import { generateMultiTabSampleFile, generateSingleTabSampleFiles } from '../engine/sampleData';
  import { CloudUpload, FileSpreadsheet, Check, X, CircleAlert } from '@lucide/svelte';
  import { tStore } from '$lib/i18n';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';

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

<Dialog.Root open={true} onOpenChange={(open) => { if (!open) onClose(); }}>
  <Dialog.Content class="max-w-xl max-h-[90vh] overflow-y-auto">
    <Dialog.Header>
      <div class="flex items-center gap-2">
        <CloudUpload size={20} class="text-primary" />
        <Dialog.Title class="text-base font-bold">{$tStore('upload_modal.title')}</Dialog.Title>
      </div>
    </Dialog.Header>

    <div class="flex flex-col gap-3 py-2">
      <!-- Drag and drop zone -->
      <div
        class="flex flex-col items-center justify-center gap-2 rounded-xl border-2 border-dashed border-border bg-muted/40 p-6 text-center transition-colors cursor-pointer hover:border-primary hover:bg-primary/5 {isDragging ? 'border-primary bg-primary/10' : ''}"
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
          class="hidden"
          onchange={handleFileInputChange}
        />

        <div class="flex h-12 w-12 items-center justify-center rounded-full bg-primary/10 text-primary">
          <CloudUpload size={24} />
        </div>

        <div>
          <div class="text-sm font-bold text-foreground">
            {$tStore('upload_modal.drop_title')} <span class="text-primary underline">{$tStore('upload_modal.browse')}</span>
          </div>
          <div class="text-xs text-muted-foreground mt-1">
            {$tStore('upload_modal.drop_subtitle')}
          </div>
        </div>

        {#if isLoading}
          <Badge variant="default" class="mt-2">
            {$tStore('upload_modal.analyzing')}
          </Badge>
        {/if}
      </div>

      <!-- Official Excel Template Download -->
      {#if onDownloadTemplate}
        <div class="flex items-center justify-between rounded-lg border border-slate-200 bg-slate-50 p-3 dark:border-slate-800 dark:bg-card">
          <div class="flex items-center gap-2">
            <FileSpreadsheet size={16} class="text-emerald-600" />
            <span class="text-xs font-semibold text-slate-800 dark:text-slate-200">{$tStore('upload_modal.download_template')}</span>
          </div>
          <Button
            variant="outline"
            size="sm"
            onclick={onDownloadTemplate}
          >
            <span>{$tStore('upload_modal.download_template')}</span>
          </Button>
        </div>
      {/if}

      <!-- Error alert -->
      {#if errorMsg}
        <div class="flex items-center gap-2 rounded-lg border border-rose-200 bg-rose-50 p-3 text-xs font-semibold text-rose-700 dark:border-rose-900/50 dark:bg-rose-950/30 dark:text-rose-300">
          <CircleAlert size={15} />
          <span>{errorMsg}</span>
        </div>
      {/if}

      <!-- Successfully parsed songs list preview -->
      {#if parsedSongs.length > 0}
        <div class="flex flex-col gap-2">
          <div class="text-xs font-bold text-slate-800 dark:text-slate-200">
            {$tStore('upload_modal.found_songs', { count: parsedSongs.length })}
          </div>
          <div class="flex flex-col gap-1.5 max-h-44 overflow-y-auto pr-1">
            {#each parsedSongs as song (song.id)}
              <div class="flex items-center justify-between rounded-lg border border-slate-200 bg-white p-2.5 dark:border-slate-800 dark:bg-slate-900">
                <div class="flex items-center gap-2">
                  <div
                    class="h-2.5 w-2.5 rounded-full"
                    style="background-color: {song.color.border};"
                  ></div>
                  <strong class="text-xs font-semibold text-slate-800 dark:text-slate-100">{song.name}</strong>
                  <span class="text-[11px] text-slate-400">
                    {$tStore('upload_modal.members_count', { count: song.members.length })}
                  </span>
                </div>
                <Badge variant="default" class="text-[10px] py-0">
                  {$tStore('upload_modal.sessions_tag', { count: song.targetSessions })}
                </Badge>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <Dialog.Footer class="flex items-center justify-between gap-2 sm:justify-between">
      <Button variant="outline" size="sm" onclick={onClose}>
        {$tStore('upload_modal.cancel')}
      </Button>
      <Button
        variant="default"
        size="sm"
        disabled={parsedSongs.length === 0}
        onclick={handleConfirm}
      >
        <Check size={14} class="mr-1.5" />
        <span>{$tStore('upload_modal.confirm_import', { count: parsedSongs.length })}</span>
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
