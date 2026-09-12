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
  import { canAccessAdmin } from '$lib/auth';
  import type { UserRole } from '$lib/types/timetable';
  import { Button } from '$lib/components/ui/button';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
  import { Badge } from '$lib/components/ui/badge';
  import { cn } from '$lib/utils';

  interface Props {
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

  function switchLanguage(code: Iso639_1Locale) {
    setLocale(code);
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

  const roleBadgeStyles: Record<string, string> = {
    member: 'bg-sky-100 text-sky-800 border-sky-200',
    qc: 'bg-purple-100 text-purple-800 border-purple-200',
    pm: 'bg-amber-100 text-amber-800 border-amber-200',
    dm: 'bg-green-100 text-green-800 border-green-200',
    moderator: 'bg-orange-100 text-orange-800 border-orange-200',
    admin: 'bg-rose-100 text-rose-800 border-rose-200',
  };
</script>

<header class="sticky top-0 z-40 flex h-14 w-full items-center justify-between border-b border-black/[0.08] bg-white/95 px-4 backdrop-blur-md dark:border-white/[0.08] dark:bg-card/95">
  <div class="flex items-center gap-3">
    {#if onToggleSidebar}
      <Button
        variant="ghost"
        size="icon-sm"
        onclick={onToggleSidebar}
        title={$tStore('navbar.sidebar_toggle')}
        aria-label={$tStore('navbar.sidebar_toggle')}
      >
        <Menu size={18} />
      </Button>
    {/if}

    <!-- Brand Logo / Emblem (Static Link to /studio) -->
    <a href="/studio" class="inline-flex items-center justify-center p-1 rounded-lg transition-transform hover:scale-105" title="CSAC Studio">
      <img src="/csac.svg" alt="CSAC Studio Logo" class="h-8 w-8 rounded-md object-contain" />
    </a>

    <!-- Quick Navigation Links (Desktop - Role Scoped) -->
    <nav class="hidden md:flex items-center gap-1">
      <Button
        href="/studio"
        variant={$page.url.pathname.startsWith('/studio') && !$page.url.pathname.startsWith('/studio/gear') ? 'secondary' : 'ghost'}
        size="sm"
        class={$page.url.pathname.startsWith('/studio') && !$page.url.pathname.startsWith('/studio/gear') ? 'text-primary font-bold' : ''}
      >
        <Music size={14} class="mr-1.5" />
        <span>{$tStore('nav.studio')}</span>
      </Button>

      <Button
        href="/studio/gear"
        variant={$page.url.pathname.startsWith('/studio/gear') ? 'secondary' : 'ghost'}
        size="sm"
        class={$page.url.pathname.startsWith('/studio/gear') ? 'text-primary font-bold' : ''}
      >
        <FileSpreadsheet size={14} class="mr-1.5" />
        <span>{$tStore('nav.gear')}</span>
      </Button>

      {#if canAccessAdmin(userRole)}
        <Button
          href="/admin/shows"
          variant={$page.url.pathname.startsWith('/admin/shows') ? 'secondary' : 'ghost'}
          size="sm"
          class={$page.url.pathname.startsWith('/admin/shows') ? 'text-primary font-bold' : ''}
        >
          <Calendar size={14} class="mr-1.5" />
          <span>{$tStore('nav.admin_shows')}</span>
        </Button>

        <Button
          href="/admin/users"
          variant={$page.url.pathname.startsWith('/admin/users') || $page.url.pathname.startsWith('/admin/approve') ? 'secondary' : 'ghost'}
          size="sm"
          class={$page.url.pathname.startsWith('/admin/users') || $page.url.pathname.startsWith('/admin/approve') ? 'text-primary font-bold' : ''}
        >
          <Users size={14} class="mr-1.5" />
          <span>{$tStore('nav.users')}</span>
        </Button>
      {/if}
    </nav>

    <!-- Contextual Document / Week Title (Rendered ONLY in /utils/timetable) -->
    {#if isTimetableRoute && onUpdateWeekTitle}
      <div class="relative flex items-center ml-2">
        <input
          type="text"
          class="h-8 rounded-lg border border-border bg-muted/40 px-2.5 pr-7 text-xs font-semibold text-foreground placeholder-muted-foreground focus:border-primary focus:bg-background focus:outline-none"
          value={weekTitle}
          oninput={(e) => onUpdateWeekTitle?.((e.target as HTMLInputElement).value)}
          placeholder={$tStore('navbar.title_placeholder')}
          title={$tStore('navbar.title_tooltip')}
        />
        <span class="pointer-events-none absolute right-2 text-muted-foreground"><Pencil size={13} /></span>
      </div>
    {/if}
  </div>

  <!-- Desktop Actions -->
  <div class="hidden md:flex items-center gap-2">
    <!-- Contextual Timetable Solver Actions (Rendered ONLY on /utils/timetable) -->
    {#if isTimetableRoute && onRunScheduler}
      {#if onDownloadTemplate}
        <Button variant="outline" size="sm" onclick={onDownloadTemplate} title={$tStore('navbar.download_template_tooltip')}>
          <FileSpreadsheet size={14} class="mr-1.5 text-emerald-600" />
          <span>{$tStore('navbar.download_template')}</span>
        </Button>
      {/if}

      {#if onOpenUpload}
        <Button variant="outline" size="sm" onclick={onOpenUpload} title={$tStore('navbar.upload_files_tooltip')}>
          <Upload size={14} class="mr-1.5 text-blue-600" />
          <span>{$tStore('navbar.upload_files')}</span>
        </Button>
      {/if}

      <Button variant="default" size="sm" onclick={onRunScheduler} disabled={isSolving} title={$tStore('navbar.auto_schedule_tooltip')}>
        <Wand2 size={14} class="mr-1.5" />
        <span>{isSolving ? $tStore('navbar.solving') : $tStore('navbar.auto_schedule')}</span>
      </Button>

      {#if onExportExcel}
        <Button variant="outline" size="sm" onclick={onExportExcel} title={$tStore('navbar.export_excel_tooltip')}>
          <Download size={14} class="mr-1.5 text-amber-600" />
          <span>{$tStore('navbar.export_excel')}</span>
        </Button>
      {/if}

      {#if onResetSchedule}
        <Button variant="ghost" size="icon-sm" onclick={onResetSchedule} title={$tStore('navbar.reset_schedule_tooltip')}>
          <RefreshCw size={14} />
        </Button>
      {/if}
    {/if}

    <!-- User Profile / Interactive Role Switcher Dropdown -->
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        {#snippet child({ props })}
          <button
            {...props}
            type="button"
            class="flex items-center gap-1.5 rounded-lg border border-border bg-muted/40 px-2.5 py-1 text-xs font-semibold text-foreground hover:bg-muted"
          >
            <UserCircle size={15} class="text-primary" />
            <span class="max-w-[80px] truncate">{auth.user?.full_name || auth.user?.email || 'Guest'}</span>
            <span class={cn("rounded px-1.5 py-0.5 text-[10px] font-extrabold uppercase", roleBadgeStyles[userRole] || roleBadgeStyles.member)}>
              {userRole}
            </span>
            <ChevronDown size={12} class="text-muted-foreground" />
          </button>
        {/snippet}
      </DropdownMenu.Trigger>
      <DropdownMenu.Content align="end" class="w-64">
        <DropdownMenu.Label class="text-xs text-muted-foreground uppercase">Simulation & Role View</DropdownMenu.Label>
        <DropdownMenu.Separator />
        {#each ['member', 'qc', 'pm', 'dm', 'moderator', 'admin'] as roleItem}
          <DropdownMenu.Item
            class="flex items-center justify-between py-1.5"
            onclick={() => selectDemoRole(roleItem as UserRole)}
          >
            <span class={cn("rounded px-1.5 py-0.5 text-[10px] font-bold uppercase", roleBadgeStyles[roleItem])}>
              {roleItem}
            </span>
            {#if userRole === roleItem}
              <Check size={14} class="text-primary" />
            {/if}
          </DropdownMenu.Item>
        {/each}
        <DropdownMenu.Separator />
        {#if auth.isAuthenticated}
          <DropdownMenu.Item onclick={() => auth.logout()} class="text-destructive">
            <LogOut size={14} class="mr-2" />
            <span>Sign Out</span>
          </DropdownMenu.Item>
        {:else}
          <DropdownMenu.Item onclick={() => goto('/auth/login')}>
            <UserCircle size={14} class="mr-2 text-primary" />
            <span>Sign In</span>
          </DropdownMenu.Item>
        {/if}
      </DropdownMenu.Content>
    </DropdownMenu.Root>

    <!-- Language Selector Dropdown -->
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        {#snippet child({ props })}
          <Button {...props} variant="outline" size="sm" class="gap-1.5 px-2.5">
            <span>{SUPPORTED_LANGUAGES[$currentLocale]?.flag || '🌐'}</span>
            <span class="text-xs font-semibold">{SUPPORTED_LANGUAGES[$currentLocale]?.nativeName || $currentLocale}</span>
            <ChevronDown size={12} class="text-slate-400" />
          </Button>
        {/snippet}
      </DropdownMenu.Trigger>
      <DropdownMenu.Content align="end">
        {#each Object.values(SUPPORTED_LANGUAGES) as lang}
          <DropdownMenu.Item
            onclick={() => switchLanguage(lang.code)}
            class="flex items-center justify-between gap-3"
          >
            <div class="flex items-center gap-2">
              <span>{lang.flag}</span>
              <span class="text-xs font-medium">{lang.nativeName}</span>
            </div>
            {#if $currentLocale === lang.code}
              <Check size={14} class="text-primary" />
            {/if}
          </DropdownMenu.Item>
        {/each}
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </div>

  <!-- Mobile Menu Trigger -->
  <div class="flex md:hidden items-center gap-1">
    <Button
      variant="ghost"
      size="icon-sm"
      onclick={() => isMobileMenuOpen = !isMobileMenuOpen}
      title={$tStore('navbar.menu')}
    >
      <MoreVertical size={18} />
    </Button>
  </div>
</header>

<!-- Mobile Dropdown Panel -->
{#if isMobileMenuOpen}
  <div class="md:hidden flex flex-col gap-2 border-b border-slate-200 bg-white p-4 shadow-lg dark:border-slate-800 dark:bg-card">
    <nav class="flex flex-col gap-1">
      <Button href="/studio" variant="ghost" size="sm" class="justify-start">
        <Music size={14} class="mr-2" />
        <span>{$tStore('nav.studio')}</span>
      </Button>
      <Button href="/studio/gear" variant="ghost" size="sm" class="justify-start">
        <FileSpreadsheet size={14} class="mr-2" />
        <span>{$tStore('nav.gear')}</span>
      </Button>
      {#if canAccessAdmin(userRole)}
        <Button href="/admin/shows" variant="ghost" size="sm" class="justify-start">
          <Calendar size={14} class="mr-2" />
          <span>{$tStore('nav.admin_shows')}</span>
        </Button>
        <Button href="/admin/users" variant="ghost" size="sm" class="justify-start">
          <Users size={14} class="mr-2" />
          <span>{$tStore('nav.users')}</span>
        </Button>
      {/if}
    </nav>

    <div class="flex items-center justify-between border-t border-slate-100 pt-3 dark:border-slate-800">
      <div class="flex items-center gap-2">
        <span class="text-xs font-semibold text-slate-500">Language:</span>
        {#each Object.values(SUPPORTED_LANGUAGES) as lang}
          <Button
            variant={$currentLocale === lang.code ? 'secondary' : 'ghost'}
            size="xs"
            onclick={() => { switchLanguage(lang.code); isMobileMenuOpen = false; }}
          >
            {lang.flag} {lang.nativeName}
          </Button>
        {/each}
      </div>
    </div>
  </div>
{/if}
