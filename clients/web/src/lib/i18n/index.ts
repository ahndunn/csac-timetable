import { writable, derived } from 'svelte/store';
import { vi } from './locales/vi';
import { en } from './locales/en';
import { SUPPORTED_LANGUAGES, type Iso639_1Locale, type LanguageOption } from './types';

export { SUPPORTED_LANGUAGES, type Iso639_1Locale, type LanguageOption };

export const DICTIONARIES = { vi, en } as const;

/** Svelte reactive store for currently active ISO 639-1 language code */
export const currentLocale = writable<Iso639_1Locale>('vi');

let _activeLocale: Iso639_1Locale = 'vi';
currentLocale.subscribe((loc) => {
  _activeLocale = loc;
});

/**
 * Set the current active language (must be valid ISO 639-1: 'vi' or 'en')
 */
export function setLocale(locale: Iso639_1Locale) {
  if (locale === 'vi' || locale === 'en') {
    currentLocale.set(locale);
  }
}

/**
 * Get current active language synchronously
 */
export function getLocale(): Iso639_1Locale {
  return _activeLocale;
}

/**
 * Inspect client machine environment to auto-detect preferred locale
 */
export function detectMachineLocale(): Iso639_1Locale {
  if (typeof navigator !== 'undefined') {
    const navLang = (navigator.languages && navigator.languages[0]) || navigator.language;
    if (navLang && navLang.toLowerCase().startsWith('vi')) {
      return 'vi';
    }
  }
  return 'en';
}

function resolvePath(obj: any, path: string): string | undefined {
  const parts = path.split('.');
  let current = obj;
  for (const part of parts) {
    if (current === undefined || current === null) return undefined;
    current = current[part];
  }
  return typeof current === 'string' ? current : undefined;
}

/**
 * Pure translation function for a given locale, key, and optional params
 */
export function translate(
  locale: Iso639_1Locale,
  key: string,
  params?: Record<string, string | number>
): string {
  const dict = DICTIONARIES[locale] || DICTIONARIES.vi;
  const fallbackDict = locale === 'vi' ? DICTIONARIES.en : DICTIONARIES.vi;

  let str = resolvePath(dict, key);
  if (str === undefined) {
    str = resolvePath(fallbackDict, key);
  }
  if (str === undefined) {
    return key;
  }

  if (params) {
    for (const [pKey, pVal] of Object.entries(params)) {
      str = str.replace(new RegExp(`\\{${pKey}\\}`, 'g'), String(pVal));
    }
  }

  return str;
}

/**
 * Direct non-reactive translation helper for current active locale
 */
export function t(key: string, params?: Record<string, string | number>): string {
  return translate(_activeLocale, key, params);
}

/**
 * Reactive Svelte derived translation store.
 * Usage in Svelte components: `$t('navbar.auto_schedule')`
 */
export const tStore = derived(currentLocale, ($locale) => {
  return (key: string, params?: Record<string, string | number>) => translate($locale, key, params);
});
