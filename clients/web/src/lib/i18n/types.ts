export type Iso639_1Locale = 'vi' | 'en';

export interface LanguageOption {
  code: Iso639_1Locale;
  nativeName: string;
  englishName: string;
  flag: string;
}

export const SUPPORTED_LANGUAGES: Record<Iso639_1Locale, LanguageOption> = {
  vi: {
    code: 'vi',
    nativeName: 'Tiếng Việt',
    englishName: 'Vietnamese',
    flag: '🇻🇳',
  },
  en: {
    code: 'en',
    nativeName: 'English',
    englishName: 'English',
    flag: '🇺🇸',
  },
};
