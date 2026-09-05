import ExcelJS from 'exceljs';
import { ScheduledSession, SongVoteData, DayOfWeek } from '../types/timetable';
import { DAYS_OF_WEEK, DEFAULT_TIME_SLOTS } from '../constants/timetableDefaults';
import { generateSampleSongs } from './sampleData';

// Convert hex color like '#e0f2fe' to ARGB hex string 'FFE0F2FE' for ExcelJS
function hexToARGB(hex: string): string {
  const cleaned = hex.replace('#', '').toUpperCase();
  return `FF${cleaned}`;
}

export async function exportTimetableToExcel(
  schedule: ScheduledSession[],
  songs: SongVoteData[],
  weekTitle: string = 'VOTE LỊCH TẬP TUẦN 1 (07/09/2026 - 13/09/2026)',
  days: DayOfWeek[] = DAYS_OF_WEEK,
  slots: string[] = DEFAULT_TIME_SLOTS
): Promise<void> {
  const workbook = new ExcelJS.Workbook();
  workbook.creator = 'CSAC Timetable Studio';
  workbook.created = new Date();

  // ==========================================
  // SHEET 1: LỊCH TẬP TUẦN (GRID VIEW)
  // ==========================================
  const sheetGrid = workbook.addWorksheet('LỊCH TẬP TUẦN', {
    views: [{ showGridLines: true }],
  });

  // Title row (Column 1 is Khung Giờ, Columns 2-8 are Thứ 2 -> Chủ Nhật = 8 columns A:H)
  sheetGrid.mergeCells('A1', 'H2');
  const titleCell = sheetGrid.getCell('A1');
  titleCell.value = weekTitle.toUpperCase();
  titleCell.font = { name: 'Arial', size: 14, bold: true, color: { argb: 'FF1E293B' } };
  titleCell.alignment = { vertical: 'middle', horizontal: 'center' };
  titleCell.fill = {
    type: 'pattern',
    pattern: 'solid',
    fgColor: { argb: 'FFF1F5F9' },
  };

  // Header row for Days
  const headerRow = sheetGrid.getRow(3);
  headerRow.height = 28;
  headerRow.getCell(1).value = 'KHUNG GIỜ';
  headerRow.getCell(1).font = { name: 'Arial', size: 11, bold: true, color: { argb: 'FF334155' } };
  headerRow.getCell(1).alignment = { vertical: 'middle', horizontal: 'center' };
  headerRow.getCell(1).fill = {
    type: 'pattern',
    pattern: 'solid',
    fgColor: { argb: 'FFE2E8F0' },
  };

  days.forEach((day, idx) => {
    const colIdx = idx + 2;
    const cell = headerRow.getCell(colIdx);
    cell.value = day;
    cell.font = { name: 'Arial', size: 11, bold: true, color: { argb: 'FF334155' } };
    cell.alignment = { vertical: 'middle', horizontal: 'center' };
    cell.fill = {
      type: 'pattern',
      pattern: 'solid',
      fgColor: { argb: 'FFE2E8F0' },
    };
  });

  // Fill in time slots and scheduled sessions
  let currentRowIdx = 4;
  for (const slot of slots) {
    const row = sheetGrid.getRow(currentRowIdx);
    row.height = 65; // Plenty of room for song name and members

    // Col 1: Time Slot
    const slotCell = row.getCell(1);
    slotCell.value = slot;
    slotCell.font = { name: 'Arial', size: 10, bold: true, color: { argb: 'FF475569' } };
    slotCell.alignment = { vertical: 'middle', horizontal: 'center', wrapText: true };
    slotCell.fill = {
      type: 'pattern',
      pattern: 'solid',
      fgColor: { argb: 'FFF8FAFC' },
    };

    // Columns for each day
    days.forEach((day, dayIdx) => {
      const colIdx = dayIdx + 2;
      const cell = row.getCell(colIdx);

      // Find sessions in this slot
      const sessionsInSlot = schedule.filter(s => s.day === day && s.slot === slot);

      if (sessionsInSlot.length > 0) {
        // Multi-song or single song
        const songLines = sessionsInSlot.map(s => {
          const absentText = s.absentMembers.length > 0 ? ` (Vắng: ${s.absentMembers.join(', ')})` : '';
          return `♫ ${s.songName}\n👥 [${s.availableMembers.join(', ')}]${absentText}`;
        }).join('\n---\n');

        cell.value = songLines;
        cell.font = { name: 'Arial', size: 9, color: { argb: 'FF1E293B' } };
        cell.alignment = { vertical: 'middle', horizontal: 'center', wrapText: true };

        // Use the pastel background color of the first session
        const primaryColor = sessionsInSlot[0].color;
        cell.fill = {
          type: 'pattern',
          pattern: 'solid',
          fgColor: { argb: hexToARGB(primaryColor.bg) },
        };
      } else {
        cell.value = '';
        cell.alignment = { vertical: 'middle', horizontal: 'center' };
      }

      // Thin borders
      cell.border = {
        top: { style: 'thin', color: { argb: 'FFE2E8F0' } },
        left: { style: 'thin', color: { argb: 'FFE2E8F0' } },
        bottom: { style: 'thin', color: { argb: 'FFE2E8F0' } },
        right: { style: 'thin', color: { argb: 'FFE2E8F0' } },
      };
    });

    slotCell.border = {
      top: { style: 'thin', color: { argb: 'FFE2E8F0' } },
      left: { style: 'thin', color: { argb: 'FFE2E8F0' } },
      bottom: { style: 'thin', color: { argb: 'FFE2E8F0' } },
      right: { style: 'thin', color: { argb: 'FFE2E8F0' } },
    };

    currentRowIdx++;
  }

  // Set column widths for Grid
  sheetGrid.getColumn(1).width = 16;
  days.forEach((_, idx) => {
    sheetGrid.getColumn(idx + 2).width = 24;
  });

  // ==========================================
  // SHEET 2: CHI TIẾT THEO BÀI HÁT
  // ==========================================
  const sheetSongs = workbook.addWorksheet('CHI TIẾT BÀI HÁT', {
    views: [{ showGridLines: true }],
  });

  const detailHeaders = [
    'STT',
    'Tên Bài Hát',
    'Thứ',
    'Khung Giờ',
    'Phòng',
    'Chuyên Cần',
    'Thành Viên Tham Gia',
    'Thành Viên Vắng',
    'Ghi Chú',
  ];

  const detailHeaderRow = sheetSongs.getRow(1);
  detailHeaderRow.height = 26;
  detailHeaders.forEach((h, idx) => {
    const cell = detailHeaderRow.getCell(idx + 1);
    cell.value = h;
    cell.font = { name: 'Arial', size: 10, bold: true, color: { argb: 'FFFFFFFF' } };
    cell.alignment = { vertical: 'middle', horizontal: 'center' };
    cell.fill = {
      type: 'pattern',
      pattern: 'solid',
      fgColor: { argb: 'FF475569' },
    };
  });

  schedule.forEach((sess, idx) => {
    const row = sheetSongs.getRow(idx + 2);
    row.height = 24;
    row.getCell(1).value = idx + 1;
    row.getCell(2).value = sess.songName;
    row.getCell(3).value = sess.day;
    row.getCell(4).value = sess.slot;
    row.getCell(5).value = `Phòng ${sess.room}`;
    row.getCell(6).value = `${sess.availableMembers.length}/${sess.allMembers.length}`;
    row.getCell(7).value = sess.availableMembers.join(', ');
    row.getCell(8).value = sess.absentMembers.length > 0 ? sess.absentMembers.join(', ') : 'Không có';
    row.getCell(9).value = sess.note || '';

    // Style row
    for (let c = 1; c <= 9; c++) {
      const cell = row.getCell(c);
      cell.font = { name: 'Arial', size: 10 };
      cell.alignment = { vertical: 'middle', horizontal: c <= 5 ? 'center' : 'left' };
      cell.border = {
        top: { style: 'thin', color: { argb: 'FFE2E8F0' } },
        left: { style: 'thin', color: { argb: 'FFE2E8F0' } },
        bottom: { style: 'thin', color: { argb: 'FFE2E8F0' } },
        right: { style: 'thin', color: { argb: 'FFE2E8F0' } },
      };
      if (c === 2) {
        cell.fill = {
          type: 'pattern',
          pattern: 'solid',
          fgColor: { argb: hexToARGB(sess.color.chipBg) },
        };
      }
    }
  });

  sheetSongs.getColumn(1).width = 6;
  sheetSongs.getColumn(2).width = 24;
  sheetSongs.getColumn(3).width = 14;
  sheetSongs.getColumn(4).width = 16;
  sheetSongs.getColumn(5).width = 12;
  sheetSongs.getColumn(6).width = 14;
  sheetSongs.getColumn(7).width = 32;
  sheetSongs.getColumn(8).width = 22;
  sheetSongs.getColumn(9).width = 26;

  // ==========================================
  // SHEET 3: LỊCH TẬP CÁ NHÂN (TỪNG THÀNH VIÊN)
  // ==========================================
  const sheetMembers = workbook.addWorksheet('LỊCH CÁ NHÂN', {
    views: [{ showGridLines: true }],
  });

  const memberHeaders = ['Thành Viên', 'Bài Hát Tham Gia', 'Thứ', 'Khung Giờ', 'Trạng Thái'];
  const memberHeaderRow = sheetMembers.getRow(1);
  memberHeaderRow.height = 26;
  memberHeaders.forEach((h, idx) => {
    const cell = memberHeaderRow.getCell(idx + 1);
    cell.value = h;
    cell.font = { name: 'Arial', size: 10, bold: true, color: { argb: 'FFFFFFFF' } };
    cell.alignment = { vertical: 'middle', horizontal: 'center' };
    cell.fill = {
      type: 'pattern',
      pattern: 'solid',
      fgColor: { argb: 'FF64748B' },
    };
  });

  // Extract all unique members
  const allMembersSet = new Set<string>();
  songs.forEach(s => s.members.forEach(m => allMembersSet.add(m)));
  const allMembers = Array.from(allMembersSet).sort();

  let memberRowIdx = 2;
  for (const m of allMembers) {
    const memberSessions = schedule.filter(s => s.allMembers.includes(m));
    if (memberSessions.length === 0) continue;

    for (const sess of memberSessions) {
      const row = sheetMembers.getRow(memberRowIdx);
      row.height = 22;
      const isPresent = sess.availableMembers.includes(m);

      row.getCell(1).value = m;
      row.getCell(2).value = sess.songName;
      row.getCell(3).value = sess.day;
      row.getCell(4).value = sess.slot;
      row.getCell(5).value = isPresent ? 'Có mặt' : 'Vắng mặt (Bận)';

      for (let c = 1; c <= 5; c++) {
        const cell = row.getCell(c);
        cell.font = { name: 'Arial', size: 10 };
        cell.alignment = { vertical: 'middle', horizontal: c >= 3 ? 'center' : 'left' };
        cell.border = {
          top: { style: 'thin', color: { argb: 'FFE2E8F0' } },
          left: { style: 'thin', color: { argb: 'FFE2E8F0' } },
          bottom: { style: 'thin', color: { argb: 'FFE2E8F0' } },
          right: { style: 'thin', color: { argb: 'FFE2E8F0' } },
        };
      }

      if (!isPresent) {
        row.getCell(5).font = { name: 'Arial', size: 10, color: { argb: 'FFE11D48' }, bold: true };
      } else {
        row.getCell(5).font = { name: 'Arial', size: 10, color: { argb: 'FF16A34A' } };
      }

      memberRowIdx++;
    }
  }

  sheetMembers.getColumn(1).width = 20;
  sheetMembers.getColumn(2).width = 24;
  sheetMembers.getColumn(3).width = 14;
  sheetMembers.getColumn(4).width = 16;
  sheetMembers.getColumn(5).width = 18;

  // Trigger browser download
  const buffer = await workbook.xlsx.writeBuffer();
  const blob = new Blob([buffer], {
    type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
  });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  const safeTitle = weekTitle.replace(/[^a-zA-Z0-9_\u00C0-\u024F\u1EA0-\u1EF9]/g, '_').substring(0, 40);
  link.download = `Lich_Tap_CSAC_${safeTitle || 'Tuan'}.xlsx`;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
}

// ==========================================
// FUNCTION: DOWNLOAD SAMPLE EXCEL WORKBOOK (5 SONGS)
// ==========================================
export async function downloadExcelTemplate(sampleSongs?: SongVoteData[]): Promise<void> {
  const workbook = new ExcelJS.Workbook();
  workbook.creator = 'CSAC Timetable Studio';
  workbook.created = new Date();

  const days: DayOfWeek[] = DAYS_OF_WEEK; // Thứ 2 -> Chủ Nhật
  const slots = DEFAULT_TIME_SLOTS;

  // If sampleSongs not passed, use generateSampleSongs()
  const songsToExport = sampleSongs && sampleSongs.length > 0 ? sampleSongs : generateSampleSongs();

  for (const song of songsToExport) {
    const sheetName = `Vote_${song.name.replace(/[^a-zA-Z0-9_\u00C0-\u024F\u1EA0-\u1EF9]/g, '_')}`.substring(0, 31);
    const sheet = workbook.addWorksheet(sheetName, {
      views: [{ showGridLines: true }],
    });

    const maxColLetter = String.fromCharCode(65 + 2 + song.members.length); // Col A, B, + members, + GHI CHU

    // Row 1-2: Title
    sheet.mergeCells(`A1:${maxColLetter}2`);
    const titleCell = sheet.getCell('A1');
    titleCell.value = song.weekTitle || 'VOTE LỊCH TẬP TUẦN 1 (07/09/2026 - 13/09/2026)';
    titleCell.font = { name: 'Arial', size: 14, bold: true, color: { argb: 'FF1F2937' } };
    titleCell.alignment = { vertical: 'middle', horizontal: 'center' };
    titleCell.fill = {
      type: 'pattern',
      pattern: 'solid',
      fgColor: { argb: 'FFFEF08A' }, // Soft yellow header matching screenshot
    };

    // Row 3: BÀI HÁT
    sheet.mergeCells('A3:B3');
    sheet.getCell('A3').value = 'BÀI HÁT';
    sheet.getCell('A3').font = { name: 'Arial', size: 11, bold: true };
    sheet.getCell('A3').alignment = { vertical: 'middle', horizontal: 'center' };

    const endMemberCol = String.fromCharCode(65 + 1 + song.members.length);
    sheet.mergeCells(`C3:${endMemberCol}3`);
    sheet.getCell('C3').value = song.name;
    sheet.getCell('C3').font = { name: 'Arial', size: 12, bold: true, color: { argb: 'FF1E40AF' } };
    sheet.getCell('C3').alignment = { vertical: 'middle', horizontal: 'center' };

    // Row 4: KHUNG GIỜ, TÊN THÀNH VIÊN, GHI CHÚ
    sheet.getCell('B4').value = 'KHUNG GIỜ';
    sheet.getCell('B4').font = { name: 'Arial', size: 10, bold: true };
    sheet.getCell('B4').alignment = { vertical: 'middle', horizontal: 'center' };

    sheet.mergeCells(`C4:${endMemberCol}4`);
    sheet.getCell('C4').value = 'TÊN THÀNH VIÊN';
    sheet.getCell('C4').font = { name: 'Arial', size: 10, bold: true };
    sheet.getCell('C4').alignment = { vertical: 'middle', horizontal: 'center' };

    const notesColLetter = maxColLetter;
    sheet.getCell(`${notesColLetter}4`).value = 'GHI CHÚ';
    sheet.getCell(`${notesColLetter}4`).font = { name: 'Arial', size: 10, bold: true };
    sheet.getCell(`${notesColLetter}4`).alignment = { vertical: 'middle', horizontal: 'center' };

    // Row 5: Member names
    song.members.forEach((m, idx) => {
      const colIdx = idx + 3; // Col C is 3
      const cell = sheet.getRow(5).getCell(colIdx);
      cell.value = m;
      cell.font = { name: 'Arial', size: 10, bold: true };
      cell.alignment = { vertical: 'middle', horizontal: 'center' };
    });

    let rowIdx = 6;
    days.forEach((day, dayIdx) => {
      const isOrangeDay = dayIdx % 2 === 0;
      const dayFillColor = isOrangeDay ? 'FFFFEDD5' : 'FFFFFFFF';
      const startDayRow = rowIdx;

      slots.forEach((slot) => {
        const row = sheet.getRow(rowIdx);
        row.height = 22;

        row.getCell(2).value = slot;
        row.getCell(2).font = { name: 'Arial', size: 10 };
        row.getCell(2).alignment = { vertical: 'middle', horizontal: 'center' };

        // Fill member vote check values
        song.members.forEach((m, mIdx) => {
          const key = `${day}__${slot}__${m}`;
          const isYes = !!song.availability[key];
          const cell = row.getCell(mIdx + 3);
          cell.value = isYes;
          cell.font = { name: 'Arial', size: 10 };
          cell.alignment = { vertical: 'middle', horizontal: 'center' };
        });

        // Note
        const noteKey = `${day}__${slot}`;
        const noteText = song.notes[noteKey] || '';
        const noteColIdx = song.members.length + 3;
        if (noteText) {
          row.getCell(noteColIdx).value = noteText;
        }

        // Set row background and thin borders
        const totalCols = song.members.length + 3;
        for (let c = 1; c <= totalCols; c++) {
          const cell = row.getCell(c);
          cell.fill = {
            type: 'pattern',
            pattern: 'solid',
            fgColor: { argb: dayFillColor },
          };
          cell.border = {
            top: { style: 'thin', color: { argb: 'FFD1D5DB' } },
            left: { style: 'thin', color: { argb: 'FFD1D5DB' } },
            bottom: { style: 'thin', color: { argb: 'FFD1D5DB' } },
            right: { style: 'thin', color: { argb: 'FFD1D5DB' } },
          };
        }

        rowIdx++;
      });

      // Merge Day cell vertically
      sheet.mergeCells(`A${startDayRow}:A${rowIdx - 1}`);
      const dayCell = sheet.getCell(`A${startDayRow}`);
      dayCell.value = day;
      dayCell.font = { name: 'Arial', size: 11, bold: true };
      dayCell.alignment = { vertical: 'middle', horizontal: 'center' };
    });

    // Style header rows 1-5
    const totalCols = song.members.length + 3;
    for (let r = 1; r <= 5; r++) {
      for (let c = 1; c <= totalCols; c++) {
        sheet.getRow(r).getCell(c).border = {
          top: { style: 'thin', color: { argb: 'FF9CA3AF' } },
          left: { style: 'thin', color: { argb: 'FF9CA3AF' } },
          bottom: { style: 'thin', color: { argb: 'FF9CA3AF' } },
          right: { style: 'thin', color: { argb: 'FF9CA3AF' } },
        };
      }
    }

    sheet.getColumn(1).width = 16;
    sheet.getColumn(2).width = 15;
    song.members.forEach((_, idx) => {
      sheet.getColumn(idx + 3).width = 15;
    });
    sheet.getColumn(totalCols).width = 22;
  }

  const buffer = await workbook.xlsx.writeBuffer();
  const blob = new Blob([buffer], {
    type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
  });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = 'Du_Lieu_Mau_Vote_CSAC_5_Bai.xlsx';
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
}

