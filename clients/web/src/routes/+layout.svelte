<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import {
    currentLocale,
    setLocale,
    detectMachineLocale,
    tStore,
    type Iso639_1Locale,
  } from '$lib/i18n';

  let { children } = $props();

  // Synchronize on initial render (supports SSR when ?lang= is present)
  if ($page.url.searchParams.has('lang')) {
    const initLang = $page.url.searchParams.get('lang');
    if (initLang === 'vi' || initLang === 'en') {
      setLocale(initLang as Iso639_1Locale);
    }
  }

  // Reactive synchronization: when $page.url changes, update active locale
  $effect(() => {
    const lang = $page.url.searchParams.get('lang');
    if (lang === 'vi' || lang === 'en') {
      setLocale(lang as Iso639_1Locale);
    }
  });

  // Keep html lang attribute in sync
  $effect(() => {
    if (typeof document !== 'undefined') {
      document.documentElement.lang = $currentLocale;
    }
  });

  // On client initial mount: if ?lang= is missing or invalid, detect machine locale and sync URL via goto
  onMount(() => {
    const currentLang = $page.url.searchParams.get('lang');
    if (currentLang !== 'vi' && currentLang !== 'en') {
      const detected = detectMachineLocale();
      setLocale(detected);
      const url = new URL($page.url);
      url.searchParams.set('lang', detected);
      goto(`?${url.searchParams.toString()}`, {
        replaceState: true,
        keepFocus: true,
        noScroll: true,
        invalidateAll: false,
      });
    }
  });
</script>

<svelte:head>
  <title>{$tStore('app.page_title')}</title>
</svelte:head>

{@render children()}
