import { DayOfWeek } from '../types/timetable';

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

export const DEFAULT_TIME_SLOTS = [
  '17h - 18h',
  '18h - 19h',
  '19h - 20h',
  '20h - 21h',
];

export const DEFAULT_WEEK_TITLE = 'VOTE LỊCH TẬP TUẦN 1 (07/09/2026 - 13/09/2026)';
