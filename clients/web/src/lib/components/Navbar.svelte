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

  function closeAll() {
    isSampleOpen = false;
    isMobileMenuOpen = false;
  }
</script>

<svelte:window
  onclick={(e) => {
    const target = e.target as HTMLElement;
    if (!target.closest('.sample-dropdown-container') && !target.closest('.mobile-menu-container')) {
      closeAll();
    }
  }}
  onkeydown={(e) => {
    if (e.key === 'Escape') closeAll();
  }}
/>

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

    <!-- Editable Document / Week Title -->
    <div class="title-edit-wrapper">
      <input
        type="text"
        class="title-edit-input"
        value={weekTitle}
        oninput={(e) => onUpdateWeekTitle((e.target as HTMLInputElement).value)}
        placeholder="Nhập tiêu đề lịch tập..."
        title="Bấm để chỉnh sửa tên lịch tập"
      />
      <span class="title-edit-icon"><Pencil size={14} /></span>
    </div>
  </div>

  <!-- Desktop Actions -->
  <div class="navbar-actions navbar-actions-desktop">
    <!-- Dropdown: Test bằng dữ liệu mẫu -->
    <div class="sample-dropdown-container">
      <button
        type="button"
        class="bento-btn {isSampleOpen ? 'is-active' : ''}"
        onclick={(e) => {
          e.stopPropagation();
          isSampleOpen = !isSampleOpen;
        }}
        title="Thử nghiệm xếp lịch bằng các bộ dữ liệu mẫu"
      >
        <Sparkles size={15} color="var(--accent)" />
        <span>Dữ liệu mẫu</span>
        <ChevronDown
          size={14}
          style="transform: {isSampleOpen ? 'rotate(180deg)' : 'none'}; transition: transform 0.2s;"
        />
      </button>

      {#if isSampleOpen}
        <div
          class="dropdown-menu-bento"
          onclick={(e) => e.stopPropagation()}
          role="presentation"
        >
          <div class="dropdown-header-bento">
            CHỌN KỊCH BẢN TEST DỮ LIỆU
          </div>

          <!-- Case 1: Multi-tab -->
          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isSampleOpen = false;
              onLoadSampleMultiTab();
            }}
          >
            <Layers size={18} color="var(--accent)" />
            <div>
              <div style="font-weight: 700;">Excel nhiều tab (Multi-tab)</div>
              <div style="font-size: 11px; color: var(--text-muted);">1 file gồm 5 tab bài hát</div>
            </div>
          </button>

          <!-- Case 2: Single-tab files -->
          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isSampleOpen = false;
              onLoadSampleSingleTab();
            }}
          >
            <Files size={18} color="var(--accent)" />
            <div>
              <div style="font-weight: 700;">5 file Excel rời (Single-tab)</div>
              <div style="font-size: 11px; color: var(--text-muted);">Mỗi file chứa 1 bài hát</div>
            </div>
          </button>

          <!-- Case 3: Mixed files -->
          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isSampleOpen = false;
              onLoadSampleMixed();
            }}
          >
            <FolderSync size={18} color="var(--accent)" />
            <div>
              <div style="font-weight: 700;">Nhiều file hỗn hợp</div>
              <div style="font-size: 11px; color: var(--text-muted);">Kiểm thử bộ chọn sheet</div>
            </div>
          </button>

          <!-- Download template -->
          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isSampleOpen = false;
              onDownloadTemplate();
            }}
          >
            <FileSpreadsheet size={18} color="var(--success)" />
            <div>
              <div style="font-weight: 700; color: var(--success-text);">Tải template Excel mẫu</div>
              <div style="font-size: 11px; color: var(--text-muted);">File chuẩn .xlsx</div>
            </div>
          </button>
        </div>
      {/if}
    </div>

    <button
      type="button"
      class="bento-btn"
      onclick={onDownloadTemplate}
      title="Tải về file Excel dữ liệu mẫu (.xlsx)"
    >
      <FileSpreadsheet size={15} />
      <span>Tải template</span>
    </button>

    <button
      type="button"
      class="bento-btn"
      onclick={onOpenUpload}
      title="Tải lên các file Excel vote lịch tập"
    >
      <Upload size={15} />
      <span>Tải file lên</span>
    </button>

    <button
      type="button"
      class="bento-btn bento-btn-primary"
      onclick={onRunScheduler}
      disabled={isSolving}
      title="Giải thuật tự động phân bổ lịch tập không trùng thành viên"
    >
      <Wand2 size={15} />
      <span>{isSolving ? 'Đang xếp...' : 'Tự động xếp lịch'}</span>
    </button>

    <button
      type="button"
      class="bento-btn"
      onclick={onExportExcel}
      title="Xuất lịch tập hoàn chỉnh ra file Excel (.xlsx)"
    >
      <Download size={15} />
      <span>Xuất Excel</span>
    </button>

    <button
      type="button"
      class="bento-icon-btn"
      onclick={onResetSchedule}
      title="Xóa / Làm mới lịch"
      aria-label="Xóa hoặc làm mới lịch"
    >
      <RefreshCw size={15} />
    </button>
  </div>

  <!-- Mobile Actions -->
  <div class="navbar-actions navbar-mobile-toggle">
    <button
      type="button"
      class="bento-btn bento-btn-primary"
      onclick={onRunScheduler}
      disabled={isSolving}
      title="Tự động xếp lịch"
    >
      <Wand2 size={15} />
      <span>{isSolving ? '...' : 'Xếp'}</span>
    </button>

    <div class="mobile-menu-container sample-dropdown-container">
      <button
        type="button"
        class="bento-icon-btn {isMobileMenuOpen ? 'is-active' : ''}"
        onclick={(e) => {
          e.stopPropagation();
          isMobileMenuOpen = !isMobileMenuOpen;
        }}
        title="Menu tác vụ khác"
        aria-label="Thao tác khác"
      >
        <MoreVertical size={18} />
      </button>

      {#if isMobileMenuOpen}
        <div
          class="dropdown-menu-bento"
          onclick={(e) => e.stopPropagation()}
          role="presentation"
        >
          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isMobileMenuOpen = false;
              onOpenUpload();
            }}
          >
            <Upload size={16} color="var(--accent)" />
            <span>Tải file Excel lên</span>
          </button>

          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isMobileMenuOpen = false;
              onExportExcel();
            }}
          >
            <Download size={16} color="var(--success)" />
            <span>Xuất file Excel</span>
          </button>

          <div class="dropdown-header-bento">DỮ LIỆU MẪU</div>

          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isMobileMenuOpen = false;
              onLoadSampleMultiTab();
            }}
          >
            <Layers size={16} color="var(--accent)" />
            <span>Test file nhiều tab</span>
          </button>

          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isMobileMenuOpen = false;
              onLoadSampleSingleTab();
            }}
          >
            <Files size={16} color="var(--accent)" />
            <span>Test 5 file đơn</span>
          </button>

          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isMobileMenuOpen = false;
              onLoadSampleMixed();
            }}
          >
            <FolderSync size={16} color="var(--accent)" />
            <span>Test file hỗn hợp</span>
          </button>

          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isMobileMenuOpen = false;
              onDownloadTemplate();
            }}
          >
            <FileSpreadsheet size={16} color="var(--success)" />
            <span>Tải template Excel</span>
          </button>

          <button
            type="button"
            class="dropdown-item-bento"
            style="color: var(--danger);"
            onclick={() => {
              isMobileMenuOpen = false;
              onResetSchedule();
            }}
          >
            <RefreshCw size={16} />
            <span>Xóa / Làm mới lịch</span>
          </button>
        </div>
      {/if}
    </div>
  </div>
</header>
