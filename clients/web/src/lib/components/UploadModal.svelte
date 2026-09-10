<script lang="ts">
  import type { SongVoteData } from '../types/timetable';
  import { inspectExcelFiles, parseSelectedSheets, type FileInspection } from '../engine/excelParser';
  import SheetSelectionModal from './SheetSelectionModal.svelte';
  import { generateMultiTabSampleFile, generateSingleTabSampleFiles } from '../engine/sampleData';
  import { UploadCloud, FileSpreadsheet, Check, X, AlertCircle, Layers, Sparkles, Files } from '@lucide/svelte';

  interface Props {
    onClose: () => void;
    onAddSongs: (newSongs: SongVoteData[]) => void;
    existingCount: number;
  }

  let { onClose, onAddSongs, existingCount }: Props = $props();

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
        errorMsg = 'Vui lòng chọn file Excel có đuôi .xlsx hoặc .xls';
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
          errorMsg = 'Không tìm thấy dữ liệu hợp lệ trong file. Vui lòng kiểm tra định dạng file Excel.';
        } else {
          parsedSongs = [...parsedSongs, ...songs];
        }
      }
    } catch (err) {
      console.error(err);
      errorMsg = 'Đã có lỗi khi đọc file Excel. Vui lòng kiểm tra lại cấu trúc file.';
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

  function handleFileInput(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files) {
      handleFiles(input.files);
    }
  }

  function handleConfirmMultiTab(selectedSongs: SongVoteData[]) {
    parsedSongs = [...parsedSongs, ...selectedSongs];
    multiTabInspections = null;
  }
</script>

{#if multiTabInspections}
  <SheetSelectionModal
    inspections={multiTabInspections}
    existingCount={existingCount + parsedSongs.length}
    onClose={() => multiTabInspections = null}
    onConfirm={handleConfirmMultiTab}
  />
{:else}
  <div class="modal-overlay" onclick={onClose} role="presentation">
    <div class="modal-dialog" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true">
      <div class="modal-header">
        <div style="display: flex; align-items: center; gap: 8px;">
          <UploadCloud size={20} color="#1a73e8" />
          <h3 class="modal-header-title">Tải lên file Vote Lịch Tập (.xlsx)</h3>
        </div>
        <button type="button" class="modal-close-btn" onclick={onClose} aria-label="Đóng">
          <X size={18} />
        </button>
      </div>

      <div class="modal-body">
        <div
          class="upload-dropzone {isDragging ? 'dragging' : ''}"
          ondragover={(e) => { e.preventDefault(); isDragging = true; }}
          ondragleave={() => isDragging = false}
          ondrop={handleDrop}
          role="presentation"
        >
          <UploadCloud size={40} color="#1a73e8" />
          <p style="font-size: 14px; font-weight: 500; color: #1f2937;">
            Kéo thả một hoặc nhiều file Excel (.xlsx) vào đây
          </p>
          <p style="font-size: 12px; color: #6b7280;">
            Hỗ trợ cả file 1 bài lẫn file nhiều Sheet (Phonecert, Nàng Thơ...)
          </p>
          <label class="btn-gcal-primary" style="cursor: pointer; margin-top: 6px;">
            <span>Chọn file từ máy tính</span>
            <input
              type="file"
              accept=".xlsx, .xls"
              multiple
              style="display: none;"
              onchange={handleFileInput}
            />
          </label>
        </div>

        {#if isLoading}
          <div style="text-align: center; padding: 12px; font-size: 13px; color: #1a73e8;">
            Đang phân tích và xử lý file Excel...
          </div>
        {/if}

        {#if errorMsg}
          <div style="display: flex; align-items: center; gap: 8px; padding: 10px; background-color: #fee2e2; border-radius: 6px; color: #b91c1c; font-size: 13px;">
            <AlertCircle size={16} />
            <span>{errorMsg}</span>
          </div>
        {/if}

        {#if parsedSongs.length > 0}
          <div style="display: flex; flex-direction: column; gap: 6px;">
            <div style="font-size: 13px; font-weight: 600; color: #374151;">
              Đã đọc thành công {parsedSongs.length} bài hát:
            </div>
            <div style="max-height: 140px; overflow-y: auto; display: flex; flex-direction: column; gap: 4px;">
              {#each parsedSongs as s}
                <div style="display: flex; align-items: center; justify-content: space-between; padding: 6px 10px; background-color: #f3f4f6; border-radius: 6px; font-size: 12px;">
                  <span style="font-weight: 500; color: #1f2937;">{s.name}</span>
                  <span style="color: #6b7280;">{s.members.length} thành viên</span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <div class="modal-footer">
        <button type="button" class="btn-gcal-secondary" onclick={onClose}>
          Hủy
        </button>
        {#if parsedSongs.length > 0}
          <button
            type="button"
            class="btn-gcal-primary"
            onclick={() => {
              onAddSongs(parsedSongs);
              onClose();
            }}
          >
            <Check size={16} />
            <span>Thêm {parsedSongs.length} bài vào studio</span>
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}
