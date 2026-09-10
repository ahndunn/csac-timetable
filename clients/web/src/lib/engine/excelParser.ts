import * as XLSX from 'xlsx';
import { PASTEL_PALETTE } from '../types/timetable';
import type { SongVoteData, DayOfWeek } from '../types/timetable';
import { DAYS_OF_WEEK, DEFAULT_WEEK_TITLE } from '../constants/timetableDefaults';

// Normalize Vietnamese day names to standard DayOfWeek
export function normalizeDay(dayRaw: string): DayOfWeek | null {
  if (!dayRaw) return null;
  const cleaned = dayRaw.trim().toUpperCase()
    .replace(/\s+/g, ' ')
    .normalize('NFC');
  
  if (cleaned.includes('HAI') || cleaned === 'THỨ 2' || cleaned === 'THU 2' || cleaned === 'T2') return 'THỨ HAI';
  if (cleaned.includes('BA') || cleaned === 'THỨ 3' || cleaned === 'THU 3' || cleaned === 'T3') return 'THỨ BA';
  if (cleaned.includes('TƯ') || cleaned.includes('TU') || cleaned === 'THỨ 4' || cleaned === 'THU 4' || cleaned === 'T4') return 'THỨ TƯ';
  if (cleaned.includes('NĂM') || cleaned.includes('NAM') || cleaned === 'THỨ 5' || cleaned === 'THU 5' || cleaned === 'T5') return 'THỨ NĂM';
  if (cleaned.includes('SÁU') || cleaned.includes('SAU') || cleaned === 'THỨ 6' || cleaned === 'THU 6' || cleaned === 'T6') return 'THỨ SÁU';
  if (cleaned.includes('BẢY') || cleaned.includes('BAY') || cleaned === 'THỨ 7' || cleaned === 'THU 7' || cleaned === 'T7') return 'THỨ BẢY';
  if (cleaned.includes('CHỦ NHẬT') || cleaned.includes('CHU NHAT') || cleaned === 'CN') return 'CHỦ NHẬT';

  return null;
}

// Normalize time slot text, e.g. "17h - 18h", "17h-18h", "17:00 - 18:00"
export function normalizeSlot(slotRaw: string): string {
  if (!slotRaw) return '';
  return slotRaw.toString().trim()
    .replace(/\s*-\s*/g, ' - ')
    .replace(/\s+/g, ' ');
}

// Check if a cell represents a "checked" vote
export function isCellChecked(val: unknown): boolean {
  if (val === true || val === 1) return true;
  if (typeof val === 'string') {
    const s = val.trim().toLowerCase();
    if (s === 'true' || s === '1' || s === 'x' || s === 'v' || s === 'yes' || s === 'có' || s === 'rảnh' || s === 'ok') {
      return true;
    }
    // Unicode checkmarks
    if (s.includes('☑') || s.includes('✔') || s.includes('✓')) {
      return true;
    }
  }
  return false;
}

export interface SheetInfo {
  sheetName: string;
  songName: string;
  weekTitle: string;
  members: string[];
  isValid: boolean;
  rowCount: number;
}

export interface FileInspection {
  fileId: string;
  fileName: string;
  file: File;
  workbook: XLSX.WorkBook;
  sheets: SheetInfo[];
  hasMultipleSheets: boolean;
}

// Inspect a single sheet to extract metadata (song name, members, validity) without full parsing
export function inspectSheet(
  sheet: XLSX.WorkSheet,
  sheetName: string,
  fileName: string
): SheetInfo {
  const rows: (string | number | boolean | null | undefined)[][] = XLSX.utils.sheet_to_json(sheet, {
    header: 1,
    defval: null,
    raw: true,
  });

  if (!rows || rows.length < 5) {
    return {
      sheetName,
      songName: sheetName,
      weekTitle: DEFAULT_WEEK_TITLE,
      members: [],
      isValid: false,
      rowCount: rows ? rows.length : 0,
    };
  }

  // 1. Detect Week Title
  let weekTitle = DEFAULT_WEEK_TITLE;
  for (let r = 0; r < Math.min(4, rows.length); r++) {
    const rowStr = rows[r]?.join(' ') || '';
    if (rowStr.toUpperCase().includes('VOTE') || rowStr.toUpperCase().includes('TUẦN') || rowStr.toUpperCase().includes('LỊCH')) {
      weekTitle = rowStr.trim();
      break;
    }
  }

  // 2. Detect Song Name
  let songName = '';
  for (let r = 0; r < Math.min(6, rows.length); r++) {
    const row = rows[r];
    if (!row) continue;
    for (let c = 0; c < row.length; c++) {
      const val = String(row[c] || '').trim();
      if (val.toUpperCase().includes('BÀI HÁT') || val.toUpperCase().includes('BAI HAT')) {
        const parts = val.split(/[:：]/);
        if (parts.length > 1 && parts[1].trim()) {
          songName = parts[1].trim();
        } else {
          for (let nextC = c + 1; nextC < row.length; nextC++) {
            const nextVal = String(row[nextC] || '').trim();
            if (nextVal) {
              songName = nextVal;
              break;
            }
          }
        }
        break;
      }
    }
    if (songName) break;
  }

  if (!songName || songName.toUpperCase() === 'BÀI HÁT') {
    const baseFileName = fileName.replace(/\.[^/.]+$/, '').replace(/^[Vv]ote[_\s-]*/i, '');
    songName = sheetName.startsWith('Sheet') ? baseFileName : sheetName;
  }

  // 3. Detect Members Header Row
  let memberRowIdx = -1;
  let memberStartCol = -1;
  let notesCol = -1;

  for (let r = 0; r < Math.min(8, rows.length); r++) {
    const row = rows[r];
    if (!row) continue;
    for (let c = 0; c < row.length; c++) {
      const val = String(row[c] || '').trim().toUpperCase();
      if (val.includes('TÊN THÀNH VIÊN') || val.includes('TEN THANH VIEN')) {
        memberRowIdx = r + 1;
        memberStartCol = c;
        break;
      }
    }
    if (memberRowIdx !== -1) break;
  }

  if (memberRowIdx === -1 || memberRowIdx >= rows.length) {
    for (let r = 0; r < rows.length; r++) {
      const col0 = String(rows[r]?.[0] || '');
      const col1 = String(rows[r]?.[1] || '');
      if (normalizeDay(col0) || normalizeDay(col1)) {
        memberRowIdx = r - 1;
        memberStartCol = 2;
        break;
      }
    }
  }

  if (memberRowIdx === -1 || memberRowIdx >= rows.length) {
    memberRowIdx = 4;
    memberStartCol = 2;
  }

  // Find notes column
  for (let r = 0; r <= memberRowIdx; r++) {
    const row = rows[r];
    if (!row) continue;
    for (let c = 0; c < row.length; c++) {
      const val = String(row[c] || '').trim().toUpperCase();
      if (val.includes('GHI CHÚ') || val.includes('GHI CHU')) {
        notesCol = c;
        break;
      }
    }
    if (notesCol !== -1) break;
  }

  const memberRow = rows[memberRowIdx] || [];
  const members: string[] = [];
  const maxCol = notesCol !== -1 ? notesCol : memberRow.length;

  for (let c = memberStartCol; c < maxCol; c++) {
    const name = String(memberRow[c] || '').trim();
    if (name && !name.toUpperCase().includes('GHI CHÚ') && !name.toUpperCase().includes('KHUNG GIỜ')) {
      members.push(name);
    }
  }

  if (members.length === 0) {
    const altRow = rows[memberRowIdx - 1] || [];
    for (let c = memberStartCol; c < maxCol; c++) {
      const name = String(altRow[c] || '').trim();
      if (name && !name.toUpperCase().includes('TÊN') && !name.toUpperCase().includes('KHUNG')) {
        members.push(name);
      }
    }
  }

  const isValid = rows.length >= 5 && members.length > 0;

  return {
    sheetName,
    songName,
    weekTitle,
    members,
    isValid,
    rowCount: rows.length,
  };
}

// Parse a single worksheet into a complete SongVoteData object
export function parseSingleSheet(
  sheet: XLSX.WorkSheet,
  sheetName: string,
  fileName: string,
  colorIndex: number = 0
): SongVoteData | null {
  const rows: (string | number | boolean | null | undefined)[][] = XLSX.utils.sheet_to_json(sheet, {
    header: 1,
    defval: null,
    raw: true,
  });

  if (!rows || rows.length < 5) return null;

  // 1. Week Title
  let weekTitle = DEFAULT_WEEK_TITLE;
  for (let r = 0; r < Math.min(4, rows.length); r++) {
    const rowStr = rows[r]?.join(' ') || '';
    if (rowStr.toUpperCase().includes('VOTE') || rowStr.toUpperCase().includes('TUẦN') || rowStr.toUpperCase().includes('LỊCH')) {
      weekTitle = rowStr.trim();
      break;
    }
  }

  // 2. Song Name
  let songName = '';
  for (let r = 0; r < Math.min(6, rows.length); r++) {
    const row = rows[r];
    if (!row) continue;
    for (let c = 0; c < row.length; c++) {
      const val = String(row[c] || '').trim();
      if (val.toUpperCase().includes('BÀI HÁT') || val.toUpperCase().includes('BAI HAT')) {
        const parts = val.split(/[:：]/);
        if (parts.length > 1 && parts[1].trim()) {
          songName = parts[1].trim();
        } else {
          for (let nextC = c + 1; nextC < row.length; nextC++) {
            const nextVal = String(row[nextC] || '').trim();
            if (nextVal) {
              songName = nextVal;
              break;
            }
          }
        }
        break;
      }
    }
    if (songName) break;
  }

  if (!songName || songName.toUpperCase() === 'BÀI HÁT') {
    const baseFileName = fileName.replace(/\.[^/.]+$/, '').replace(/^[Vv]ote[_\s-]*/i, '');
    songName = sheetName.startsWith('Sheet') ? baseFileName : sheetName;
  }

  // 3. Member Columns
  let memberRowIdx = -1;
  let memberStartCol = -1;
  let notesCol = -1;

  for (let r = 0; r < Math.min(8, rows.length); r++) {
    const row = rows[r];
    if (!row) continue;
    for (let c = 0; c < row.length; c++) {
      const val = String(row[c] || '').trim().toUpperCase();
      if (val.includes('TÊN THÀNH VIÊN') || val.includes('TEN THANH VIEN')) {
        memberRowIdx = r + 1;
        memberStartCol = c;
        break;
      }
    }
    if (memberRowIdx !== -1) break;
  }

  if (memberRowIdx === -1 || memberRowIdx >= rows.length) {
    for (let r = 0; r < rows.length; r++) {
      const col0 = String(rows[r]?.[0] || '');
      const col1 = String(rows[r]?.[1] || '');
      if (normalizeDay(col0) || normalizeDay(col1)) {
        memberRowIdx = r - 1;
        memberStartCol = 2;
        break;
      }
    }
  }

  if (memberRowIdx === -1 || memberRowIdx >= rows.length) {
    memberRowIdx = 4;
    memberStartCol = 2;
  }

  for (let r = 0; r <= memberRowIdx; r++) {
    const row = rows[r];
    if (!row) continue;
    for (let c = 0; c < row.length; c++) {
      const val = String(row[c] || '').trim().toUpperCase();
      if (val.includes('GHI CHÚ') || val.includes('GHI CHU')) {
        notesCol = c;
        break;
      }
    }
    if (notesCol !== -1) break;
  }

  const memberRow = rows[memberRowIdx] || [];
  const members: { name: string; col: number }[] = [];
  const maxCol = notesCol !== -1 ? notesCol : memberRow.length;

  for (let c = memberStartCol; c < maxCol; c++) {
    const name = String(memberRow[c] || '').trim();
    if (name && !name.toUpperCase().includes('GHI CHÚ') && !name.toUpperCase().includes('KHUNG GIỜ')) {
      members.push({ name, col: c });
    }
  }

  if (members.length === 0) {
    const altRow = rows[memberRowIdx - 1] || [];
    for (let c = memberStartCol; c < maxCol; c++) {
      const name = String(altRow[c] || '').trim();
      if (name && !name.toUpperCase().includes('TÊN') && !name.toUpperCase().includes('KHUNG')) {
        members.push({ name, col: c });
      }
    }
  }

  if (members.length === 0) return null;

  // 4. Availability Matrix & Notes
  const availability: Record<string, boolean> = {};
  const notes: Record<string, string> = {};

  let currentDay: DayOfWeek | null = null;
  const dataStartRow = memberRowIdx + 1;

  for (let r = dataStartRow; r < rows.length; r++) {
    const row = rows[r];
    if (!row || row.length === 0) continue;

    const dayCell = String(row[0] || '').trim();
    const detectedDay = normalizeDay(dayCell);
    if (detectedDay) {
      currentDay = detectedDay;
    }

    const slotCell = String(row[1] || '').trim();
    const normalizedSlot = normalizeSlot(slotCell);

    if (!currentDay || !normalizedSlot) {
      const altDay = normalizeDay(String(row[1] || ''));
      if (altDay) currentDay = altDay;
      continue;
    }

    for (const m of members) {
      const cellVal = row[m.col];
      const isYes = isCellChecked(cellVal);
      availability[`${currentDay}__${normalizedSlot}__${m.name}`] = isYes;
    }

    if (notesCol !== -1 && row[notesCol]) {
      const noteText = String(row[notesCol]).trim();
      if (noteText) {
        notes[`${currentDay}__${normalizedSlot}`] = noteText;
      }
    }
  }

  const color = PASTEL_PALETTE[colorIndex % PASTEL_PALETTE.length];

  return {
    id: `song-${Date.now()}-${Math.random().toString(36).substring(2, 8)}`,
    name: songName,
    weekTitle,
    members: members.map(m => m.name),
    availability,
    notes,
    color,
    targetSessions: 1,
    sourceFileName: fileName,
  };
}

// Inspect multiple Excel files and extract sheet metadata
export async function inspectExcelFiles(files: File[]): Promise<FileInspection[]> {
  const inspections: FileInspection[] = [];

  for (let i = 0; i < files.length; i++) {
    const file = files[i];
    const data = await file.arrayBuffer();
    const workbook = XLSX.read(data, { type: 'array', cellDates: true });
    const sheets: SheetInfo[] = [];

    for (const sheetName of workbook.SheetNames) {
      const sheet = workbook.Sheets[sheetName];
      if (!sheet) continue;
      const info = inspectSheet(sheet, sheetName, file.name);
      sheets.push(info);
    }

    inspections.push({
      fileId: `file_${i}_${Date.now()}_${Math.random().toString(36).substring(2, 7)}`,
      fileName: file.name,
      file,
      workbook,
      sheets,
      hasMultipleSheets: sheets.length > 1,
    });
  }

  return inspections;
}

// Parse only the selected sheets from inspected files
// selectedKeys format: `${fileId}::${sheetName}`
export function parseSelectedSheets(
  inspections: FileInspection[],
  selectedKeys: Set<string>,
  existingCount: number = 0
): SongVoteData[] {
  const results: SongVoteData[] = [];
  let colorIndex = existingCount;

  for (const inspection of inspections) {
    for (const sheetInfo of inspection.sheets) {
      const key = `${inspection.fileId}::${sheetInfo.sheetName}`;
      if (selectedKeys.has(key)) {
        const sheet = inspection.workbook.Sheets[sheetInfo.sheetName];
        if (!sheet) continue;
        const song = parseSingleSheet(sheet, sheetInfo.sheetName, inspection.fileName, colorIndex);
        if (song) {
          results.push(song);
          colorIndex++;
        }
      }
    }
  }

  return results;
}

// Backward compatible: parses all sheets of a single file
export async function parseExcelFile(
  file: File,
  existingCount: number = 0
): Promise<SongVoteData[]> {
  const [inspection] = await inspectExcelFiles([file]);
  if (!inspection) return [];

  const allKeys = new Set(
    inspection.sheets
      .filter(s => s.isValid)
      .map(s => `${inspection.fileId}::${s.sheetName}`)
  );

  return parseSelectedSheets([inspection], allKeys, existingCount);
}
