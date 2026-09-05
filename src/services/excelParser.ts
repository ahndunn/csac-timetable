import * as XLSX from 'xlsx';
import { SongVoteData, PASTEL_PALETTE, DayOfWeek } from '../types/timetable';
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

export async function parseExcelFile(
  file: File,
  existingCount: number = 0
): Promise<SongVoteData[]> {
  const data = await file.arrayBuffer();
  const workbook = XLSX.read(data, { type: 'array', cellDates: true });
  const results: SongVoteData[] = [];

  let colorIndex = existingCount;

  for (const sheetName of workbook.SheetNames) {
    const sheet = workbook.Sheets[sheetName];
    if (!sheet) continue;

    // Convert sheet to 2D array of raw values
    const rows: (string | number | boolean | null | undefined)[][] = XLSX.utils.sheet_to_json(sheet, {
      header: 1,
      defval: null,
      raw: true,
    });

    if (!rows || rows.length < 5) continue;

    // 1. Detect Week Title from top rows
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
          // Check if song name is after a colon in the same cell
          const parts = val.split(/[:：]/);
          if (parts.length > 1 && parts[1].trim()) {
            songName = parts[1].trim();
          } else {
            // Or in the next non-empty cell to the right
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

    // Fallback song name from sheet name or filename
    if (!songName || songName.toUpperCase() === 'BÀI HÁT') {
      const baseFileName = file.name.replace(/\.[^/.]+$/, '').replace(/^[Vv]ote[_\s-]*/i, '');
      songName = sheetName.startsWith('Sheet') ? baseFileName : sheetName;
    }

    // 3. Detect Members Header Row
    // Look for row containing "TÊN THÀNH VIÊN" or column headers with member names
    let memberRowIdx = -1;
    let memberStartCol = -1;
    let notesCol = -1;

    for (let r = 0; r < Math.min(8, rows.length); r++) {
      const row = rows[r];
      if (!row) continue;
      for (let c = 0; c < row.length; c++) {
        const val = String(row[c] || '').trim().toUpperCase();
        if (val.includes('TÊN THÀNH VIÊN') || val.includes('TEN THANH VIEN')) {
          // The actual member names are typically in the next row, or same row starting here
          memberRowIdx = r + 1;
          memberStartCol = c;
          break;
        }
      }
      if (memberRowIdx !== -1) break;
    }

    // Fallback: if "TÊN THÀNH VIÊN" not explicitly written, scan rows 3-6 for row above first day
    if (memberRowIdx === -1 || memberRowIdx >= rows.length) {
      for (let r = 0; r < rows.length; r++) {
        const col0 = String(rows[r]?.[0] || '');
        const col1 = String(rows[r]?.[1] || '');
        if (normalizeDay(col0) || normalizeDay(col1)) {
          memberRowIdx = r - 1;
          memberStartCol = 2; // Column C
          break;
        }
      }
    }

    if (memberRowIdx === -1 || memberRowIdx >= rows.length) {
      memberRowIdx = 4; // default Row 5 (0-indexed 4)
      memberStartCol = 2;
    }

    // Find notes column ("GHI CHÚ")
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

    // Extract member names from memberRowIdx
    const memberRow = rows[memberRowIdx] || [];
    const members: { name: string; col: number }[] = [];
    const maxCol = notesCol !== -1 ? notesCol : memberRow.length;

    for (let c = memberStartCol; c < maxCol; c++) {
      const name = String(memberRow[c] || '').trim();
      // Avoid header words
      if (name && !name.toUpperCase().includes('GHI CHÚ') && !name.toUpperCase().includes('KHUNG GIỜ')) {
        members.push({ name, col: c });
      }
    }

    if (members.length === 0) {
      // If still empty, try scanning row before
      const altRow = rows[memberRowIdx - 1] || [];
      for (let c = memberStartCol; c < maxCol; c++) {
        const name = String(altRow[c] || '').trim();
        if (name && !name.toUpperCase().includes('TÊN') && !name.toUpperCase().includes('KHUNG')) {
          members.push({ name, col: c });
        }
      }
    }

    // 4. Extract Day, Slot, and Member Availability Votes
    const availability: Record<string, boolean> = {};
    const notes: Record<string, string> = {};

    let currentDay: DayOfWeek | null = null;

    const dataStartRow = memberRowIdx + 1;
    for (let r = dataStartRow; r < rows.length; r++) {
      const row = rows[r];
      if (!row || row.length === 0) continue;

      // Col 0: Day
      const dayCell = String(row[0] || '').trim();
      const detectedDay = normalizeDay(dayCell);
      if (detectedDay) {
        currentDay = detectedDay;
      }

      // Col 1: Time Slot
      const slotCell = String(row[1] || '').trim();
      const normalizedSlot = normalizeSlot(slotCell);

      if (!currentDay || !normalizedSlot) {
        // Maybe slot is in col 0 if no day column? Or check if day was in col 1
        const altDay = normalizeDay(String(row[1] || ''));
        if (altDay) currentDay = altDay;
        continue;
      }

      // Read member votes
      for (const m of members) {
        const cellVal = row[m.col];
        const isYes = isCellChecked(cellVal);
        availability[`${currentDay}__${normalizedSlot}__${m.name}`] = isYes;
      }

      // Read note
      if (notesCol !== -1 && row[notesCol]) {
        const noteText = String(row[notesCol]).trim();
        if (noteText) {
          notes[`${currentDay}__${normalizedSlot}`] = noteText;
        }
      }
    }

    const color = PASTEL_PALETTE[colorIndex % PASTEL_PALETTE.length];
    colorIndex++;

    results.push({
      id: `song-${Date.now()}-${Math.random().toString(36).substr(2, 6)}`,
      name: songName,
      weekTitle,
      members: members.map(m => m.name),
      availability,
      notes,
      color,
      targetSessions: 1, // Default 1 session, user can change to 2, 3...
      sourceFileName: file.name,
    });
  }

  return results;
}
