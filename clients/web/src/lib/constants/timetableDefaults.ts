import type { DayOfWeek } from '../types/timetable';

export const DAYS_OF_WEEK: DayOfWeek[] = [
  'THỨ HAI',
  'THỨ BA',
  'THỨ TƯ',
  'THỨ NĂM',
  'THỨ SÁU',
  'THỨ BẢY',
  'CHỦ NHẬT',
];

export const DAY_SHORT_LABELS: Record<DayOfWeek, string> = {
  'THỨ HAI': 'T2',
  'THỨ BA': 'T3',
  'THỨ TƯ': 'T4',
  'THỨ NĂM': 'T5',
  'THỨ SÁU': 'T6',
  'THỨ BẢY': 'T7',
  'CHỦ NHẬT': 'CN',
};

export const DAY_DISPLAY_LABELS: Record<'vi' | 'en', Record<DayOfWeek, { full: string; short: string }>> = {
  vi: {
    'THỨ HAI': { full: 'THỨ HAI', short: 'T2' },
    'THỨ BA': { full: 'THỨ BA', short: 'T3' },
    'THỨ TƯ': { full: 'THỨ TƯ', short: 'T4' },
    'THỨ NĂM': { full: 'THỨ NĂM', short: 'T5' },
    'THỨ SÁU': { full: 'THỨ SÁU', short: 'T6' },
    'THỨ BẢY': { full: 'THỨ BẢY', short: 'T7' },
    'CHỦ NHẬT': { full: 'CHỦ NHẬT', short: 'CN' },
  },
  en: {
    'THỨ HAI': { full: 'MONDAY', short: 'MON' },
    'THỨ BA': { full: 'TUESDAY', short: 'TUE' },
    'THỨ TƯ': { full: 'WEDNESDAY', short: 'WED' },
    'THỨ NĂM': { full: 'THURSDAY', short: 'THU' },
    'THỨ SÁU': { full: 'FRIDAY', short: 'FRI' },
    'THỨ BẢY': { full: 'SATURDAY', short: 'SAT' },
    'CHỦ NHẬT': { full: 'SUNDAY', short: 'SUN' },
  },
};


export const DEFAULT_TIME_SLOTS = [
  '17h - 18h',
  '18h - 19h',
  '19h - 20h',
  '20h - 21h',
];

export const DEFAULT_WEEK_TITLE = 'VOTE LỊCH TẬP TUẦN 1 (07/09/2026 - 13/09/2026)';
