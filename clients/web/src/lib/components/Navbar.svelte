<script lang="ts">
  import {
    Upload,
    Wand2,
    Download,
    Sparkles,
    RefreshCw,
    FileSpreadsheet,
    Pencil,
    ChevronDown,
    Layers,
    Files,
    FolderSync,
    Menu,
    MoreVertical,
    X,
  } from '@lucide/svelte';

  interface Props {
    weekTitle: string;
    onUpdateWeekTitle: (newTitle: string) => void;
    onOpenUpload: () => void;
    onRunScheduler: () => void;
    onExportExcel: () => void;
    onLoadSampleSingleTab: () => void;
    onLoadSampleMultiTab: () => void;
    onLoadSampleMixed: () => void;
    onResetSchedule: () => void;
    onDownloadTemplate: () => void;
    isSolving?: boolean;
    onToggleSidebar?: () => void;
  }

  let {
    weekTitle,
    onUpdateWeekTitle,
    onOpenUpload,
    onRunScheduler,
    onExportExcel,
    onLoadSampleSingleTab,
    onLoadSampleMultiTab,
    onLoadSampleMixed,
    onResetSchedule,
    onDownloadTemplate,
    isSolving = false,
    onToggleSidebar,
  }: Props = $props();

  let isSampleOpen = $state(false);
  let isMobileMenuOpen = $state(false);

  function toggleSample() {
    isSampleOpen = !isSampleOpen;
  }

  function closeAll() {
    isSampleOpen = false;
    isMobileMenuOpen = false;
  }
</script>

<svelte:window onclick={(e) => {
  const target = e.target as HTMLElement;
  if (!target.closest('.sample-dropdown-container') && !target.closest('.mobile-menu-container')) {
    closeAll();
  }
}} onkeydown={(e) => {
  if (e.key === 'Escape') closeAll();
}} />

<header class="navbar">
  <div class="navbar-left">
    {#if onToggleSidebar}
      <button
        type="button"
        class="navbar-hamburger-btn"
        onclick={onToggleSidebar}
        title="Mở menu danh sách bài hát & cài đặt"
        aria-label="Menu"
      >
        <Menu size={20} />
      </button>
    {/if}

    <div class="title-edit-wrapper">
      <input
        type="text"
        class="title-edit-input"
        value={weekTitle}
        oninput={(e) => onUpdateWeekTitle((e.target as HTMLInputElement).value)}
        placeholder="Nhập tiêu đề tuần..."
        title="Bấm để chỉnh sửa tên tuần"
      />
      <span class="title-edit-icon"><Pencil size={13} /></span>
    </div>
  </div>

  <div class="navbar-right">
    <button
      type="button"
      class="btn btn-secondary desktop-only"
      onclick={onOpenUpload}
      title="Tải lên tệp Excel (.xlsx) chứa vote lịch tập"
    >
      <Upload size={16} />
      <span>Tải File Vote Excel</span>
    </button>

    <div class="sample-dropdown-container desktop-only">
      <button
        type="button"
        class="btn btn-secondary dropdown-trigger"
        onclick={toggleSample}
        title="Nạp dữ liệu mẫu để thử nghiệm nhanh thuật toán"
        aria-expanded={isSampleOpen}
      >
        <Sparkles size={16} />
        <span>Dữ Liệu Mẫu</span>
        <ChevronDown size={14} class={isSampleOpen ? 'rotate-180' : ''} />
      </button>

      {#if isSampleOpen}
        <div class="dropdown-menu">
          <button
            type="button"
            class="dropdown-item"
            onclick={() => { onLoadSampleMultiTab(); isSampleOpen = false; }}
          >
            <Layers size={16} />
            <div class="dropdown-item-text">
              <strong>File 1 Workbook Nhiều Sheet (5 bài)</strong>
              <span>Mở modal chọn tab: Phonecert, Nàng Thơ...</span>
            </div>
          </button>

          <button
            type="button"
            class="dropdown-item"
            onclick={() => { onLoadSampleSingleTab(); isSampleOpen = false; }}
          >
            <Files size={16} />
            <div class="dropdown-item-text">
              <strong>5 File Riêng Lẻ (Mỗi file 1 sheet)</strong>
              <span>Tự động nạp trực tiếp toàn bộ 5 bài hát</span>
            </div>
          </button>

          <button
            type="button"
            class="dropdown-item"
            onclick={() => { onLoadSampleMixed(); isSampleOpen = false; }}
          >
            <FolderSync size={16} />
            <div class="dropdown-item-text">
              <strong>Tập Hợp Hỗn Hợp (1 multi + 1 single)</strong>
              <span>Mô phỏng ban tổ chức nhận từ nhiều nguồn</span>
            </div>
          </button>
        </div>
      {/if}
    </div>

    <button
      type="button"
      class="btn btn-primary"
      onclick={onRunScheduler}
      disabled={isSolving}
      title="Kích hoạt thuật toán CSP giải và tối ưu xếp lịch"
    >
      <Wand2 size={16} class={isSolving ? 'animate-spin' : ''} />
      <span>{isSolving ? 'Đang Xếp Lịch...' : 'Tự Động Xếp Lịch'}</span>
    </button>

    <button
      type="button"
      class="btn btn-success desktop-only"
      onclick={onExportExcel}
      title="Xuất file Excel gồm 3 sheet: Lịch Tuần, Chi Tiết Bài Hát, Lịch Cá Nhân"
    >
      <Download size={16} />
      <span>Xuất File Excel</span>
    </button>

    <div class="mobile-menu-container mobile-only">
      <button
        type="button"
        class="navbar-icon-btn"
        onclick={() => isMobileMenuOpen = !isMobileMenuOpen}
        title="Thao tác khác"
        aria-label="Thao tác khác"
      >
        {#if isMobileMenuOpen}<X size={20} />{:else}<MoreVertical size={20} />{/if}
      </button>

      {#if isMobileMenuOpen}
        <div class="mobile-dropdown-menu">
          <button
            type="button"
            class="mobile-menu-item"
            onclick={() => { onOpenUpload(); isMobileMenuOpen = false; }}
          >
            <Upload size={18} />
            <span>Tải File Vote Excel</span>
          </button>
          <button
            type="button"
            class="mobile-menu-item"
            onclick={() => { onLoadSampleMultiTab(); isMobileMenuOpen = false; }}
          >
            <Sparkles size={18} />
            <span>Dữ Liệu Mẫu (Multi-Sheet)</span>
          </button>
          <button
            type="button"
            class="mobile-menu-item"
            onclick={() => { onExportExcel(); isMobileMenuOpen = false; }}
          >
            <Download size={18} />
            <span>Xuất File Excel</span>
          </button>
        </div>
      {/if}
    </div>
  </div>
</header>
