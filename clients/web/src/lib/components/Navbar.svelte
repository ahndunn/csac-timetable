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
    Check,
    Languages,
  } from '@lucide/svelte';
  import { replaceState } from '$app/navigation';
  import {
    tStore,
    currentLocale,
    setLocale,
    SUPPORTED_LANGUAGES,
    type Iso639_1Locale,
  } from '$lib/i18n';

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
  let isLangOpen = $state(false);

  function closeAll() {
    isSampleOpen = false;
    isMobileMenuOpen = false;
    isLangOpen = false;
  }

  function switchLanguage(code: Iso639_1Locale) {
    setLocale(code);
    isLangOpen = false;
    if (typeof window !== 'undefined') {
      const url = new URL(window.location.href);
      url.searchParams.set('lang', code);
      replaceState(url.toString(), {});
    }
  }
</script>

<svelte:window
  onclick={(e) => {
    const target = e.target as HTMLElement;
    if (
      !target.closest('.sample-dropdown-container') &&
      !target.closest('.mobile-menu-container') &&
      !target.closest('.lang-dropdown-container')
    ) {
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
        title={$tStore('navbar.sidebar_toggle')}
        aria-label={$tStore('navbar.sidebar_toggle')}
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
        placeholder={$tStore('navbar.title_placeholder')}
        title={$tStore('navbar.title_tooltip')}
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
          isLangOpen = false;
        }}
        title={$tStore('navbar.sample_data_tooltip')}
      >
        <Sparkles size={15} color="var(--accent)" />
        <span>{$tStore('navbar.sample_data')}</span>
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
            {$tStore('navbar.sample_header')}
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
              <div style="font-weight: 700;">{$tStore('navbar.sample_multitab_title')}</div>
              <div style="font-size: 11px; color: var(--text-muted);">{$tStore('navbar.sample_multitab_desc')}</div>
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
              <div style="font-weight: 700;">{$tStore('navbar.sample_singletab_title')}</div>
              <div style="font-size: 11px; color: var(--text-muted);">{$tStore('navbar.sample_singletab_desc')}</div>
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
              <div style="font-weight: 700;">{$tStore('navbar.sample_mixed_title')}</div>
              <div style="font-size: 11px; color: var(--text-muted);">{$tStore('navbar.sample_mixed_desc')}</div>
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
              <div style="font-weight: 700; color: var(--success-text);">{$tStore('navbar.sample_download_title')}</div>
              <div style="font-size: 11px; color: var(--text-muted);">{$tStore('navbar.sample_download_desc')}</div>
            </div>
          </button>
        </div>
      {/if}
    </div>

    <button
      type="button"
      class="bento-btn"
      onclick={onDownloadTemplate}
      title={$tStore('navbar.download_template_tooltip')}
    >
      <FileSpreadsheet size={15} />
      <span>{$tStore('navbar.download_template')}</span>
    </button>

    <button
      type="button"
      class="bento-btn"
      onclick={onOpenUpload}
      title={$tStore('navbar.upload_files_tooltip')}
    >
      <Upload size={15} />
      <span>{$tStore('navbar.upload_files')}</span>
    </button>

    <button
      type="button"
      class="bento-btn bento-btn-primary"
      onclick={onRunScheduler}
      disabled={isSolving}
      title={$tStore('navbar.auto_schedule_tooltip')}
    >
      <Wand2 size={15} />
      <span>{isSolving ? $tStore('navbar.solving') : $tStore('navbar.auto_schedule')}</span>
    </button>

    <button
      type="button"
      class="bento-btn"
      onclick={onExportExcel}
      title={$tStore('navbar.export_excel_tooltip')}
    >
      <Download size={15} />
      <span>{$tStore('navbar.export_excel')}</span>
    </button>

    <button
      type="button"
      class="bento-icon-btn"
      onclick={onResetSchedule}
      title={$tStore('navbar.reset_schedule_tooltip')}
      aria-label={$tStore('navbar.reset_schedule_tooltip')}
    >
      <RefreshCw size={15} />
    </button>

    <!-- Language Selector Dropdown (Top Right) -->
    <div class="sample-dropdown-container lang-dropdown-container">
      <button
        type="button"
        class="bento-btn {isLangOpen ? 'is-active' : ''}"
        onclick={(e) => {
          e.stopPropagation();
          isLangOpen = !isLangOpen;
          isSampleOpen = false;
        }}
        title={$tStore('navbar.language_switcher')}
        aria-label={$tStore('navbar.language_switcher')}
      >
        <span style="font-size: 15px; line-height: 1;">{SUPPORTED_LANGUAGES[$currentLocale].flag}</span>
        <span style="font-weight: 600;">{SUPPORTED_LANGUAGES[$currentLocale].nativeName}</span>
        <ChevronDown
          size={14}
          style="transform: {isLangOpen ? 'rotate(180deg)' : 'none'}; transition: transform 0.2s;"
        />
      </button>

      {#if isLangOpen}
        <div
          class="dropdown-menu-bento"
          style="min-width: 200px;"
          onclick={(e) => e.stopPropagation()}
          role="presentation"
        >
          <div class="dropdown-header-bento">
            {$tStore('navbar.language_switcher')}
          </div>
          {#each Object.values(SUPPORTED_LANGUAGES) as lang}
            <button
              type="button"
              class="dropdown-item-bento {lang.code === $currentLocale ? 'is-active' : ''}"
              onclick={() => switchLanguage(lang.code)}
            >
              <span style="font-size: 18px; line-height: 1;">{lang.flag}</span>
              <div style="flex: 1; text-align: left;">
                <div style="font-weight: 700;">{lang.nativeName}</div>
                <div style="font-size: 11px; color: var(--text-muted);">{lang.englishName} ({lang.code})</div>
              </div>
              {#if lang.code === $currentLocale}
                <Check size={16} color="var(--accent)" />
              {/if}
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  <!-- Mobile Actions -->
  <div class="navbar-actions navbar-mobile-toggle">
    <button
      type="button"
      class="bento-btn bento-btn-primary"
      onclick={onRunScheduler}
      disabled={isSolving}
      title={$tStore('navbar.auto_schedule')}
    >
      <Wand2 size={15} />
      <span>{isSolving ? '...' : $tStore('navbar.auto_schedule')}</span>
    </button>

    <div class="mobile-menu-container sample-dropdown-container">
      <button
        type="button"
        class="bento-icon-btn {isMobileMenuOpen ? 'is-active' : ''}"
        onclick={(e) => {
          e.stopPropagation();
          isMobileMenuOpen = !isMobileMenuOpen;
          isLangOpen = false;
        }}
        title={$tStore('navbar.mobile_more')}
        aria-label={$tStore('navbar.mobile_more')}
      >
        <MoreVertical size={18} />
      </button>

      {#if isMobileMenuOpen}
        <div
          class="dropdown-menu-bento"
          onclick={(e) => e.stopPropagation()}
          role="presentation"
        >
          <!-- Mobile Language Switcher Options -->
          <div class="dropdown-header-bento">
            {$tStore('navbar.language_switcher')}
          </div>
          {#each Object.values(SUPPORTED_LANGUAGES) as lang}
            <button
              type="button"
              class="dropdown-item-bento {lang.code === $currentLocale ? 'is-active' : ''}"
              onclick={() => {
                isMobileMenuOpen = false;
                switchLanguage(lang.code);
              }}
            >
              <span style="font-size: 18px;">{lang.flag}</span>
              <span style="flex: 1; font-weight: {lang.code === $currentLocale ? '700' : '500'};">{lang.nativeName}</span>
              {#if lang.code === $currentLocale}
                <Check size={16} color="var(--accent)" />
              {/if}
            </button>
          {/each}

          <div class="dropdown-header-bento" style="margin-top: 4px;">
            {$tStore('navbar.mobile_more')}
          </div>

          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isMobileMenuOpen = false;
              onOpenUpload();
            }}
          >
            <Upload size={16} color="var(--accent)" />
            <span>{$tStore('navbar.upload_files')}</span>
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
            <span>{$tStore('navbar.export_excel')}</span>
          </button>

          <div class="dropdown-header-bento">{$tStore('navbar.sample_data')}</div>

          <button
            type="button"
            class="dropdown-item-bento"
            onclick={() => {
              isMobileMenuOpen = false;
              onLoadSampleMultiTab();
            }}
          >
            <Layers size={16} color="var(--accent)" />
            <span>{$tStore('navbar.sample_multitab_title')}</span>
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
            <span>{$tStore('navbar.sample_singletab_title')}</span>
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
            <span>{$tStore('navbar.sample_mixed_title')}</span>
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
            <span>{$tStore('navbar.download_template')}</span>
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
            <span>{$tStore('navbar.reset_schedule')}</span>
          </button>
        </div>
      {/if}
    </div>
  </div>
</header>
