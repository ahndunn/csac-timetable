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
    Menu,
    MoreVertical,
    Check,
    LayoutGrid,
    Calendar,
    Users,
    ShieldAlert,
    UserCircle,
    LogOut,
    Music,
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
  import { canAccessAdmin, hasRole } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';

  interface Props {
    // Optional timetable solver props: only provided by /utils/timetable
    weekTitle?: string;
    onUpdateWeekTitle?: (newTitle: string) => void;
    onOpenUpload?: () => void;
    onRunScheduler?: () => void;
    onExportExcel?: () => void;
    onLoadSampleSingleTab?: () => void;
    onLoadSampleMultiTab?: () => void;
    onLoadSampleMixed?: () => void;
    onResetSchedule?: () => void;
    onDownloadTemplate?: () => void;
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

  let isMobileMenuOpen = $state(false);
  let isLangOpen = $state(false);
  let isNavDropdownOpen = $state(false);

  function closeAll() {
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

  const isTimetableRoute = $derived(($page.url.pathname as string) === '/utils/timetable');
  const userRole = $derived(($page.data.user?.role || auth.user?.role || 'admin') as UserRole);

  function selectDemoRole(newRole: UserRole) {
    const url = new URL($page.url);
    url.searchParams.set('role', newRole);
    goto(url.toString(), { invalidateAll: true });
  }
</script>

<svelte:window
  onclick={(e) => {
    const target = e.target as HTMLElement;
    if (
      !target.closest('.sample-dropdown-container') &&
      !target.closest('.mobile-menu-container') &&
      !target.closest('.lang-dropdown-container') &&
      !target.closest('.nav-menu-container') &&
      !target.closest('.role-picker-container')
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

    <!-- Brand Logo / Emblem (Static Link to /studio) -->
    <a href="/studio" class="brand-logo-btn" title="CSAC Studio">
      <img src="/csac.svg" alt="CSAC Studio Logo" class="brand-logo-img" />
    </a>

    <!-- Quick Navigation Links (Desktop - Role Scoped) -->
    <nav class="nav-links-desktop">
      <a href="/studio" class="nav-link {$page.url.pathname.startsWith('/studio') && !$page.url.pathname.startsWith('/studio/gear') ? 'is-active' : ''}">
        <Music size={14} />
        <span>{$tStore('nav.studio')}</span>
      </a>
      <a href="/studio/gear" class="nav-link {$page.url.pathname.startsWith('/studio/gear') ? 'is-active' : ''}">
        <FileSpreadsheet size={14} />
        <span>{$tStore('nav.gear')}</span>
      </a>

      {#if canAccessAdmin(userRole)}
        <a href="/admin/shows" class="nav-link {$page.url.pathname.startsWith('/admin/shows') ? 'is-active' : ''}">
          <Calendar size={14} />
          <span>{$tStore('nav.admin_shows')}</span>
        </a>
        <a href="/admin/users" class="nav-link {$page.url.pathname.startsWith('/admin/users') || $page.url.pathname.startsWith('/admin/approve') ? 'is-active' : ''}">
          <Users size={14} />
          <span>{$tStore('nav.users')}</span>
        </a>
      {/if}
    </nav>

    <!-- Contextual Document / Week Title (Rendered ONLY in /utils/timetable) -->
    {#if isTimetableRoute && onUpdateWeekTitle}
      <div class="title-edit-wrapper">
        <input
          type="text"
          class="title-edit-input"
          value={weekTitle}
          oninput={(e) => onUpdateWeekTitle?.((e.target as HTMLInputElement).value)}
          placeholder={$tStore('navbar.title_placeholder')}
          title={$tStore('navbar.title_tooltip')}
        />
        <span class="title-edit-icon"><Pencil size={14} /></span>
      </div>
    {/if}
  </div>

  <!-- Desktop Actions -->
  <div class="navbar-actions navbar-actions-desktop">
    <!-- Contextual Timetable Solver Actions (Rendered ONLY on /utils/timetable) -->
    {#if isTimetableRoute && onRunScheduler}
      {#if onDownloadTemplate}
        <button
          type="button"
          class="bento-btn"
          onclick={onDownloadTemplate}
          title={$tStore('navbar.download_template_tooltip')}
        >
          <FileSpreadsheet size={15} />
          <span>{$tStore('navbar.download_template')}</span>
        </button>
      {/if}

      {#if onOpenUpload}
        <button
          type="button"
          class="bento-btn"
          onclick={onOpenUpload}
          title={$tStore('navbar.upload_files_tooltip')}
        >
          <Upload size={15} />
          <span>{$tStore('navbar.upload_files')}</span>
        </button>
      {/if}

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

      {#if onExportExcel}
        <button
          type="button"
          class="bento-btn"
          onclick={onExportExcel}
          title={$tStore('navbar.export_excel_tooltip')}
        >
          <Download size={15} />
          <span>{$tStore('navbar.export_excel')}</span>
        </button>
      {/if}

      {#if onResetSchedule}
        <button
          type="button"
          class="bento-icon-btn"
          onclick={onResetSchedule}
          title={$tStore('navbar.reset_schedule_tooltip')}
          aria-label={$tStore('navbar.reset_schedule_tooltip')}
        >
          <RefreshCw size={15} />
        </button>
      {/if}
    {/if}

    <!-- User Profile / Interactive Role Switcher Pill -->
    <div class="user-badge-wrapper sample-dropdown-container role-picker-container">
      <button 
        type="button" 
        class="user-badge-btn" 
        onclick={(e) => {
          e.stopPropagation();
          isNavDropdownOpen = !isNavDropdownOpen;
          isLangOpen = false;
        }}
        title="Switch Role View (Demo)"
      >
        <UserCircle size={16} class="text-orange" />
        <span class="user-role-pill role-{userRole}">{userRole.toUpperCase()}</span>
        <ChevronDown size={12} />
      </button>

      {#if isNavDropdownOpen}
        <div class="dropdown-menu-bento role-dropdown-menu" onclick={(e) => e.stopPropagation()} role="presentation">
          <div class="dropdown-header-bento">Role-Based Access Level (Demo)</div>
          {#each (['member', 'qc', 'pm', 'dm', 'moderator', 'admin'] as const) as roleOpt}
            <button
              type="button"
              class="dropdown-item-bento {userRole === roleOpt ? 'is-active' : ''}"
              onclick={() => {
                isNavDropdownOpen = false;
                selectDemoRole(roleOpt);
              }}
            >
              <span class="role-pill-mini role-{roleOpt}">{roleOpt.toUpperCase()}</span>
              <div style="flex: 1; text-align: left;">
                <div style="font-weight: 600; font-size: 13px;">
                  {roleOpt === 'member' ? 'Member (Nhạc công)' :
                   roleOpt === 'qc' ? 'QC Auditor (Kiểm định)' :
                   roleOpt === 'pm' ? 'PM (Quản lý bài)' :
                   roleOpt === 'dm' ? 'DM (Trưởng ban nhạc)' :
                   roleOpt === 'moderator' ? 'Moderator (Điều hành)' : 'System Admin'}
                </div>
              </div>
              {#if userRole === roleOpt}
                <Check size={14} color="var(--accent)" />
              {/if}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Language Selector Dropdown (Top Right) -->
    <div class="sample-dropdown-container lang-dropdown-container">
      <button
        type="button"
        class="bento-btn {isLangOpen ? 'is-active' : ''}"
        onclick={(e) => {
          e.stopPropagation();
          isLangOpen = !isLangOpen;
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
    {#if isTimetableRoute && onRunScheduler}
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
    {/if}

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
          <a href="/studio" class="dropdown-item-bento" onclick={() => isMobileMenuOpen = false}>
            <Music size={16} class="text-orange" />
            <span>{$tStore('nav.studio')}</span>
          </a>
          <a href="/studio/gear" class="dropdown-item-bento" onclick={() => isMobileMenuOpen = false}>
            <FileSpreadsheet size={16} class="text-blue" />
            <span>{$tStore('nav.gear')}</span>
          </a>
          <a href="/admin/shows" class="dropdown-item-bento" onclick={() => isMobileMenuOpen = false}>
            <Calendar size={16} class="text-green" />
            <span>{$tStore('nav.admin_shows')}</span>
          </a>
          <a href="/admin/users" class="dropdown-item-bento" onclick={() => isMobileMenuOpen = false}>
            <Users size={16} class="text-purple" />
            <span>{$tStore('nav.users')}</span>
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

          {#if isTimetableRoute && (onOpenUpload || onExportExcel)}
            <div class="dropdown-header-bento" style="margin-top: 4px;">
              {$tStore('navbar.mobile_more')}
            </div>

            {#if onOpenUpload}
              <button
                type="button"
                class="dropdown-item-bento"
                onclick={() => {
                  isMobileMenuOpen = false;
                  onOpenUpload?.();
                }}
              >
                <Upload size={16} color="var(--accent)" />
                <span>{$tStore('navbar.upload_files')}</span>
              </button>
            {/if}

            {#if onExportExcel}
              <button
                type="button"
                class="dropdown-item-bento"
                onclick={() => {
                  isMobileMenuOpen = false;
                  onExportExcel?.();
                }}
              >
                <Download size={16} color="var(--success)" />
                <span>{$tStore('navbar.export_excel')}</span>
              </button>
            {/if}
          {/if}
        </div>
      {/if}
    </div>
  </div>
</header>

<style>
  .nav-links-desktop {
    display: flex;
    align-items: center;
    gap: 4px;
    margin-left: 12px;
    flex-shrink: 0;
    white-space: nowrap;
  }

  .nav-link {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: var(--radius-sm);
    font-size: 13px;
    font-weight: 600;
    color: var(--text-secondary);
    text-decoration: none;
    white-space: nowrap;
    transition: all 0.15s ease;
  }

  .nav-link span {
    white-space: nowrap;
  }

  .nav-link:hover {
    color: var(--text-primary);
    background: var(--surface-card-subtle);
  }

  .nav-link.is-active {
    color: var(--accent);
    background: var(--accent-light);
  }

  .user-badge-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    padding: 4px 10px;
    border-radius: 12px;
    font-size: 12px;
    font-weight: 600;
    color: #334155;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .user-badge-btn:hover {
    background: #f1f5f9;
    border-color: #cbd5e1;
  }

  .user-role-pill, .role-pill-mini {
    display: inline-block;
    padding: 2px 6px;
    border-radius: 6px;
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.04em;
  }

  .role-member { background: #e0f2fe; color: #0369a1; }
  .role-qc { background: #ede9fe; color: #6d28d9; }
  .role-pm { background: #fef9c3; color: #a16207; }
  .role-dm { background: #dcfce7; color: #15803d; }
  .role-moderator { background: #ffedd5; color: #c2410c; }
  .role-admin { background: #ffe4e6; color: #be123c; }

  .role-dropdown-menu {
    min-width: 240px;
    right: 0;
  }

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

  .brand-logo-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 2px;
    border-radius: 10px;
    transition: transform 0.2s ease, opacity 0.2s ease;
    text-decoration: none;
  }

  .brand-logo-btn:hover {
    transform: scale(1.05);
    opacity: 0.9;
  }

  .brand-logo-img {
    height: 32px;
    width: 32px;
    object-fit: contain;
    border-radius: 8px;
  }

  .nav-dropdown {
    min-width: 250px;
  }

  .text-orange { color: #ff6b00; }
  .text-blue { color: #3b82f6; }
  .text-green { color: #16a34a; }
  .text-purple { color: #9333ea; }
  .text-red { color: #ef4444; }

  @media (max-width: 960px) {
    .nav-links-desktop {
      display: none;
    }
  }
</style>
