<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { inspectExcelFiles, type FileInspection } from '$lib/engine/excelParser';
  import { generateMultiTabSampleFile, generateSingleTabSampleFiles, generateMixedSampleFiles } from '$lib/engine/sampleData';
  import { tStore } from '$lib/i18n';
  import { FileSpreadsheet, CheckCircle, AlertTriangle, ArrowRight, Upload } from '@lucide/svelte';

  let inspections = $state<FileInspection[]>([]);
  let isLoading = $state(false);

  async function handleFileSelect(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      isLoading = true;
      try {
        const files = Array.from(target.files);
        inspections = await inspectExcelFiles(files);
      } finally {
        isLoading = false;
      }
    }
  }

  async function loadSample(type: 'single' | 'multi' | 'mixed') {
    isLoading = true;
    try {
      let files: File[] = [];
      if (type === 'single') files = generateSingleTabSampleFiles();
      else if (type === 'multi') files = [generateMultiTabSampleFile()];
      else files = generateMixedSampleFiles();
      inspections = await inspectExcelFiles(files);
    } finally {
      isLoading = false;
    }
  }
</script>

<svelte:head>
  <title>{$tStore('inspector.page_title')}</title>
</svelte:head>

<Navbar
  weekTitle={$tStore('inspector.navbar_title')}
  onUpdateWeekTitle={() => {}}
  onOpenUpload={() => {}}
  onRunScheduler={() => {}}
  onExportExcel={() => {}}
  onLoadSampleSingleTab={() => loadSample('single')}
  onLoadSampleMultiTab={() => loadSample('multi')}
  onLoadSampleMixed={() => loadSample('mixed')}
  onResetSchedule={() => inspections = []}
  onDownloadTemplate={() => {}}
  isSolving={false}
  onToggleSidebar={() => {}}
/>

<div class="inspector-container">
  <div class="header-card">
    <div class="header-content">
      <div class="icon-wrap">
        <FileSpreadsheet size={32} class="text-orange" />
      </div>
      <div>
        <h1 class="title">{$tStore('inspector.heading')}</h1>
        <p class="subtitle">{$tStore('inspector.subheading')}</p>
      </div>
    </div>

    <div class="action-bar">
      <label class="upload-btn">
        <Upload size={18} />
        <span>{$tStore('inspector.btn_inspect')}</span>
        <input type="file" multiple accept=".xlsx,.xls" onchange={handleFileSelect} style="display: none;" />
      </label>
      <div class="sample-pills">
        <button class="pill-btn" onclick={() => loadSample('single')}>{$tStore('inspector.btn_sample_single')}</button>
        <button class="pill-btn" onclick={() => loadSample('multi')}>{$tStore('inspector.btn_sample_multi')}</button>
        <button class="pill-btn" onclick={() => loadSample('mixed')}>{$tStore('inspector.btn_sample_mixed')}</button>
      </div>
    </div>
  </div>

  {#if isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>{$tStore('inspector.loading')}</p>
    </div>
  {:else if inspections.length === 0}
    <div class="empty-state">
      <FileSpreadsheet size={48} class="empty-icon" />
      <h3>{$tStore('inspector.empty_title')}</h3>
      <p>{$tStore('inspector.empty_desc')}</p>
    </div>
  {:else}
    <div class="inspections-grid">
      {#each inspections as file (file.fileId)}
        <div class="file-card">
          <div class="file-header">
            <div>
              <h3 class="file-name">{file.fileName}</h3>
              <span class="file-meta">{$tStore('inspector.sheets_meta', { count: file.sheets.length, size: (file.fileSize / 1024).toFixed(1) })}</span>
            </div>
            <span class="badge {file.isMultiSheet ? 'badge-multi' : 'badge-single'}">
              {file.isMultiSheet ? $tStore('inspector.badge_multisheet') : $tStore('inspector.badge_singlesheet')}
            </span>
          </div>

          <div class="sheets-list">
            {#each file.sheets as sheet}
              <div class="sheet-row {sheet.isValid ? 'sheet-valid' : 'sheet-invalid'}">
                <div class="sheet-info">
                  <div class="sheet-title-row">
                    {#if sheet.isValid}
                      <CheckCircle size={16} class="text-success" />
                    {:else}
                      <AlertTriangle size={16} class="text-warning" />
                    {/if}
                    <span class="sheet-name">{sheet.sheetName}</span>
                    <span class="song-title">({sheet.songName})</span>
                  </div>
                  <div class="sheet-details">
                    <span>{$tStore('inspector.members_summary', { count: sheet.memberCount, names: sheet.sampleMembers.slice(0, 3).join(', ') + (sheet.sampleMembers.length > 3 ? '...' : '') })}</span>
                    <span>&bull; {$tStore('inspector.valid_matrix', { status: sheet.hasValidMatrix ? $tStore('inspector.yes') : $tStore('inspector.no') })}</span>
                  </div>
                </div>

                <a href="/utils/timetable" class="solve-link">
                  <span>{$tStore('inspector.solve_in_studio')}</span>
                  <ArrowRight size={14} />
                </a>
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .inspector-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .header-card {
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 20px;
    padding: 1.75rem 2rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.03);
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .header-content {
    display: flex;
    align-items: center;
    gap: 1.25rem;
  }

  .icon-wrap {
    width: 56px;
    height: 56px;
    border-radius: 16px;
    background: rgba(255, 107, 0, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .text-orange { color: #ff6b00; }
  .text-success { color: #10b981; }
  .text-warning { color: #f59e0b; }

  .title {
    font-size: 1.5rem;
    font-weight: 700;
    color: #0f172a;
    margin: 0 0 0.25rem 0;
  }

  .subtitle {
    font-size: 0.95rem;
    color: #64748b;
    margin: 0;
  }

  .action-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 1rem;
    padding-top: 1rem;
    border-top: 1px solid #f1f5f9;
  }

  .upload-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: #ff6b00;
    color: #ffffff;
    padding: 0.65rem 1.25rem;
    border-radius: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }

  .upload-btn:hover {
    background: #e65c00;
  }

  .sample-pills {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .pill-btn {
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    padding: 0.5rem 0.9rem;
    border-radius: 10px;
    font-size: 0.85rem;
    font-weight: 500;
    color: #334155;
    cursor: pointer;
    transition: all 0.2s;
  }

  .pill-btn:hover {
    border-color: #cbd5e1;
    background: #f1f5f9;
  }

  .inspections-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1.25rem;
  }

  .file-card {
    background: #ffffff;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 18px;
    padding: 1.5rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.02);
  }

  .file-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid #f1f5f9;
  }

  .file-name {
    font-size: 1.15rem;
    font-weight: 600;
    color: #1e293b;
    margin: 0;
  }

  .file-meta {
    font-size: 0.85rem;
    color: #94a3b8;
  }

  .badge {
    font-size: 0.75rem;
    font-weight: 600;
    padding: 0.35rem 0.75rem;
    border-radius: 20px;
  }

  .badge-multi { background: #eff6ff; color: #3b82f6; }
  .badge-single { background: #f0fdf4; color: #16a34a; }

  .sheets-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .sheet-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.85rem 1rem;
    border-radius: 12px;
    background: #f8fafc;
    border: 1px solid #f1f5f9;
  }

  .sheet-title-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .sheet-name {
    font-weight: 600;
    color: #1e293b;
  }

  .song-title {
    color: #64748b;
    font-size: 0.9rem;
  }

  .sheet-details {
    font-size: 0.825rem;
    color: #64748b;
    margin-top: 0.25rem;
    margin-left: 1.5rem;
  }

  .solve-link {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.85rem;
    font-weight: 600;
    color: #ff6b00;
    text-decoration: none;
    padding: 0.4rem 0.8rem;
    border-radius: 8px;
    background: rgba(255, 107, 0, 0.08);
    transition: all 0.2s;
  }

  .solve-link:hover {
    background: rgba(255, 107, 0, 0.15);
  }

  .empty-state {
    text-align: center;
    padding: 4rem 2rem;
    background: #ffffff;
    border: 1px dashed #cbd5e1;
    border-radius: 20px;
    color: #64748b;
  }

  .empty-icon {
    color: #94a3b8;
    margin-bottom: 1rem;
  }

  .loading-state {
    text-align: center;
    padding: 3rem 2rem;
    color: #64748b;
  }

  .spinner {
    width: 36px;
    height: 36px;
    border: 3px solid #e2e8f0;
    border-top-color: #ff6b00;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin: 0 auto 1rem auto;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
