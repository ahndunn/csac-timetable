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
    LayoutGrid,
    Calendar,
    Users,
    ShieldAlert,
    UserCircle,
    LogOut,
  } from '@lucide/svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { auth } from '$lib/stores/auth.svelte';
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
  let isNavDropdownOpen = $state(false);

  function closeAll() {
    isSampleOpen = false;
    isMobileMenuOpen = false;
    isLangOpen = false;
    isNavDropdownOpen = false;
  }

  function switchLanguage(code: Iso639_1Locale) {
    setLocale(code);
    isLangOpen = false;
    const url = new URL($page.url);
    url.searchParams.set('lang', code);
    goto(`?${url.searchParams.toString()}`, {
      replaceState: true,
      keepFocus: true,
      noScroll: true,
      invalidateAll: false,
    });
  }
</script>

<svelte:window
  onclick={(e) => {
    const target = e.target as HTMLElement;
    if (
      !target.closest('.sample-dropdown-container') &&
      !target.closest('.mobile-menu-container') &&
      !target.closest('.lang-dropdown-container') &&
      !target.closest('.nav-menu-container')
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

    <!-- Navigation Hub Dropdown -->
    <div class="nav-menu-container sample-dropdown-container">
      <button
        type="button"
        class="bento-btn brand-btn {isNavDropdownOpen ? 'is-active' : ''}"
        onclick={(e) => {
          e.stopPropagation();
          isNavDropdownOpen = !isNavDropdownOpen;
          isSampleOpen = false;
          isLangOpen = false;
        }}
      >
        <Sparkles size={16} class="text-orange" />
        <span class="brand-text">CSAC Studio</span>
        <ChevronDown size={14} style="transform: {isNavDropdownOpen ? 'rotate(180deg)' : 'none'}; transition: transform 0.2s;" />
      </button>

      {#if isNavDropdownOpen}
        <div class="dropdown-menu-bento nav-dropdown" onclick={(e) => e.stopPropagation()} role="presentation">
          <div class="dropdown-header-bento">{$tStore('nav.apps_header')}</div>
          <a href="/" class="dropdown-item-bento" onclick={() => isNavDropdownOpen = false}>
            <LayoutGrid size={16} class="text-orange" />
            <div>
              <div style="font-weight: 700;">{$tStore('nav.hub')}</div>
              <div style="font-size: 11px; color: var(--text-muted);">{$tStore('nav.hub_desc')}</div>
            </div>
          </a>
          <a href="/utils/timetable" class="dropdown-item-bento" onclick={() => isNavDropdownOpen = false}>
            <Calendar size={16} class="text-orange" />
            <div>
              <div style="font-weight: 700;">{$tStore('nav.timetable')}</div>
              <div style="font-size: 11px; color: var(--text-muted);">{$tStore('nav.timetable_desc')}</div>
            </div>
          </a>
          <a href="/utils/inspector" class="dropdown-item-bento" onclick={() => isNavDropdownOpen = false}>
            <FileSpreadsheet size={16} class="text-blue" />
            <div>
              <div style="font-weight: 700;">{$tStore('nav.inspector')}</div>
              <div style="font-size: 11px; color: var(--text-muted);">{$tStore('nav.inspector_desc')}</div>
            </div>
          </a>

          <div class="dropdown-header-bento" style="margin-top: 4px;">{$tStore('nav.admin_header')}</div>
          <a href="/admin/users" class="dropdown-item-bento" onclick={() => isNavDropdownOpen = false}>
            <Users size={16} class="text-purple" />
            <div>
              <div style="font-weight: 700;">{$tStore('nav.users')}</div>
              <div style="font-size: 11px; color: var(--text-muted);">{$tStore('nav.users_desc')}</div>
            </div>
          </a>
          <a href="/admin/events" class="dropdown-item-bento" onclick={() => isNavDropdownOpen = false}>
            <Calendar size={16} class="text-green" />
            <div>
              <div style="font-weight: 700;">{$tStore('nav.events')}</div>
              <div style="font-size: 11px; color: var(--text-muted);">{$tStore('nav.events_desc')}</div>
            </div>
          </a>
          <a href="/admin/approve" class="dropdown-item-bento" onclick={() => isNavDropdownOpen = false}>
            <ShieldAlert size={16} class="text-red" />
            <div>
              <div style="font-weight: 700;">{$tStore('nav.approve')}</div>
              <div style="font-size: 11px; color: var(--text-muted);">{$tStore('nav.approve_desc')}</div>
            </div>
          </a>
        </div>
      {/if}
    </div>

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
          isNavDropdownOpen = false;
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

    <!-- User Profile / Auth Button -->
    {#if auth.isAuthenticated}
      <div class="user-badge" title={$tStore('nav.logged_in_as', { name: auth.user?.full_name ?? '', role: auth.user?.role ?? '' })}>
        <UserCircle size={16} class="text-orange" />
        <span class="user-name">{auth.user?.full_name?.split(' ')[0]}</span>
        <button class="logout-mini-btn" onclick={() => auth.logout()} title={$tStore('nav.sign_out')}>
          <LogOut size={13} />
        </button>
      </div>
    {:else}
      <a href="/auth/login" class="bento-btn login-btn">
        <span>{$tStore('nav.sign_in')}</span>
      </a>
    {/if}

    <!-- Language Selector Dropdown (Top Right) -->
    <div class="sample-dropdown-container lang-dropdown-container">
      <button
        type="button"
        class="bento-btn {isLangOpen ? 'is-active' : ''}"
        onclick={(e) => {
          e.stopPropagation();
          isLangOpen = !isLangOpen;
          isSampleOpen = false;
          isNavDropdownOpen = false;
        }}
        title={$tStore('navbar.language_switcher')}
        aria-label={$tStore('navbar.language_switcher')}
      >
        <span style="font-size: 15px; line-height: 1;">{SUPPORTED_LANGUAGES[$currentLocale]?.flag ?? '🇻🇳'}</span>
        <span style="font-weight: 600;">{SUPPORTED_LANGUAGES[$currentLocale]?.nativeName ?? 'Tiếng Việt'}</span>
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
          <div class="dropdown-header-bento">{$tStore('nav.nav_header')}</div>
          <a href="/" class="dropdown-item-bento" onclick={() => isMobileMenuOpen = false}>
            <LayoutGrid size={16} class="text-orange" />
            <span>{$tStore('nav.hub')}</span>
          </a>
          <a href="/utils/timetable" class="dropdown-item-bento" onclick={() => isMobileMenuOpen = false}>
            <Calendar size={16} class="text-orange" />
            <span>{$tStore('nav.timetable')}</span>
          </a>
          <a href="/admin/users" class="dropdown-item-bento" onclick={() => isMobileMenuOpen = false}>
            <Users size={16} class="text-purple" />
            <span>{$tStore('nav.users')}</span>
          </a>
          <a href="/admin/events" class="dropdown-item-bento" onclick={() => isMobileMenuOpen = false}>
            <Calendar size={16} class="text-green" />
            <span>{$tStore('nav.events')}</span>
          </a>
          <a href="/admin/approve" class="dropdown-item-bento" onclick={() => isMobileMenuOpen = false}>
            <ShieldAlert size={16} class="text-red" />
            <span>{$tStore('nav.approve')}</span>
          </a>

          <!-- Mobile Language Switcher Options -->
          <div class="dropdown-header-bento" style="margin-top: 4px;">
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
        </div>
      {/if}
    </div>
  </div>
</header>

<style>
  .user-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    padding: 0.4rem 0.75rem;
    border-radius: 12px;
    font-size: 0.85rem;
    font-weight: 600;
    color: #334155;
  }

  .user-name {
    max-width: 80px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .logout-mini-btn {
    background: none;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    padding: 0.1rem;
    display: flex;
    align-items: center;
    transition: color 0.2s;
  }

  .logout-mini-btn:hover {
    color: #ef4444;
  }

  .login-btn {
    text-decoration: none;
    background: rgba(255, 107, 0, 0.1);
    color: #ff6b00;
    font-weight: 600;
  }

  .brand-btn {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    font-weight: 700;
  }

  .brand-text {
    color: #0f172a;
  }

  .nav-dropdown {
    min-width: 240px;
  }

  .text-orange { color: #ff6b00; }
  .text-blue { color: #3b82f6; }
  .text-green { color: #16a34a; }
  .text-purple { color: #9333ea; }
  .text-red { color: #ef4444; }
</style>
