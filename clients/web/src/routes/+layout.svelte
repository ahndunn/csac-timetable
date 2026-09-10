<script lang="ts">
  import '../app.css';
  import { page } from '$app/state';
  import { replaceState } from '$app/navigation';
  import { onMount } from 'svelte';
  import {
    currentLocale,
    setLocale,
    detectMachineLocale,
    tStore,
    type Iso639_1Locale,
  } from '$lib/i18n';

  let { children } = $props();

  // Keep html lang attribute synchronized
  $effect(() => {
    if (typeof document !== 'undefined') {
      document.documentElement.lang = $currentLocale;
    }
  });

  // Reactive synchronization from URL search param
  $effect(() => {
    const langParam = page.url.searchParams.get('lang');
    if (langParam === 'vi' || langParam === 'en') {
      if ($currentLocale !== langParam) {
        setLocale(langParam as Iso639_1Locale);
      }
    }
  });

  onMount(() => {
    const url = new URL(window.location.href);
    const existingLang = url.searchParams.get('lang');

    if (existingLang === 'vi' || existingLang === 'en') {
      setLocale(existingLang as Iso639_1Locale);
    } else {
      // Default to machine locale when query parameter is missing
      const detected = detectMachineLocale();
      setLocale(detected);
      url.searchParams.set('lang', detected);
      replaceState(url.toString(), {});
    }
  });
</script>

<svelte:head>
  <title>{$tStore('app.page_title')}</title>
</svelte:head>

{@render children()}
