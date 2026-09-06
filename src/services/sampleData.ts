import * as XLSX from 'xlsx';
import { SongVoteData, PASTEL_PALETTE, DayOfWeek } from '../types/timetable';
import { DAYS_OF_WEEK, DEFAULT_TIME_SLOTS, DEFAULT_WEEK_TITLE } from '../constants/timetableDefaults';

export function createEmptyVoteMatrix(
  days: DayOfWeek[],
  slots: string[],
  members: string[]
): Record<string, boolean> {
  const availability: Record<string, boolean> = {};
  for (const day of days) {
    for (const slot of slots) {
      for (const member of members) {
        availability[`${day}__${slot}__${member}`] = false;
      }
    }
  }
  return availability;
}

export function generateSampleSongs(): SongVoteData[] {
  // 5 Songs with overlapping members as described in the user prompt:
  // "People can be in multiple songs. Song A will be once this week but song B need three time this week."
  
  const days = DAYS_OF_WEEK; // Full week: Thứ 2 -> Chủ Nhật
  const slots = DEFAULT_TIME_SLOTS;

  // Song 1: PHONECERT (from user screenshot)
  const phonecertMembers = ['Minh Pháp', 'Duy Thành', 'Gia Huy', 'Anh Pha', 'Quang Lực'];
  const phonecertAvail = createEmptyVoteMatrix(days, slots, phonecertMembers);
  // Availability based on the image (Wed 17h-21h, Fri 17h-21h are highlighted in the screenshot, plus some scattered)
  const phonecertYesSlots = [
    // Thứ Tư all available
    { day: 'THỨ TƯ' as DayOfWeek, slot: '17h - 18h' },
    { day: 'THỨ TƯ' as DayOfWeek, slot: '18h - 19h' },
    { day: 'THỨ TƯ' as DayOfWeek, slot: '19h - 20h' },
    { day: 'THỨ TƯ' as DayOfWeek, slot: '20h - 21h' },
    // Thứ Sáu all available
    { day: 'THỨ SÁU' as DayOfWeek, slot: '18h - 19h' },
    { day: 'THỨ SÁU' as DayOfWeek, slot: '19h - 20h' },
    { day: 'THỨ SÁU' as DayOfWeek, slot: '20h - 21h' },
    // Thứ Hai
    { day: 'THỨ HAI' as DayOfWeek, slot: '18h - 19h' },
    { day: 'THỨ HAI' as DayOfWeek, slot: '19h - 20h' },
    // Chủ Nhật
    { day: 'CHỦ NHẬT' as DayOfWeek, slot: '18h - 19h' },
    { day: 'CHỦ NHẬT' as DayOfWeek, slot: '19h - 20h' },
  ];
  for (const s of phonecertYesSlots) {
    for (const m of phonecertMembers) {
      phonecertAvail[`${s.day}__${s.slot}__${m}`] = true;
    }
  }

  // Song 2: BẬT TÌNH YÊU LÊN (Shares Minh Pháp, Anh Pha)
  const batTinhYeuLenMembers = ['Minh Pháp', 'Anh Pha', 'Bảo Ngọc', 'Thu Hà'];
  const batTinhYeuLenAvail = createEmptyVoteMatrix(days, slots, batTinhYeuLenMembers);
  const batTinhYeuLenYesSlots = [
    { day: 'THỨ HAI' as DayOfWeek, slot: '19h - 20h' },
    { day: 'THỨ HAI' as DayOfWeek, slot: '20h - 21h' },
    { day: 'THỨ BA' as DayOfWeek, slot: '18h - 19h' },
    { day: 'THỨ BA' as DayOfWeek, slot: '19h - 20h' },
    { day: 'THỨ NĂM' as DayOfWeek, slot: '17h - 18h' },
    { day: 'THỨ NĂM' as DayOfWeek, slot: '18h - 19h' },
    { day: 'THỨ BẢY' as DayOfWeek, slot: '19h - 20h' },
    { day: 'CHỦ NHẬT' as DayOfWeek, slot: '19h - 20h' },
  ];
  for (const s of batTinhYeuLenYesSlots) {
    for (const m of batTinhYeuLenMembers) {
      batTinhYeuLenAvail[`${s.day}__${s.slot}__${m}`] = true;
    }
  }

  // Song 3: NÀNG THƠ (Shares Duy Thành, Gia Huy)
  const nangThoMembers = ['Duy Thành', 'Gia Huy', 'Hoàng Long', 'Kim Ngân'];
  const nangThoAvail = createEmptyVoteMatrix(days, slots, nangThoMembers);
  const nangThoYesSlots = [
    { day: 'THỨ HAI' as DayOfWeek, slot: '17h - 18h' },
    { day: 'THỨ BA' as DayOfWeek, slot: '19h - 20h' },
    { day: 'THỨ BA' as DayOfWeek, slot: '20h - 21h' },
    { day: 'THỨ NĂM' as DayOfWeek, slot: '19h - 20h' },
    { day: 'THỨ NĂM' as DayOfWeek, slot: '20h - 21h' },
    { day: 'THỨ BẢY' as DayOfWeek, slot: '17h - 18h' },
    { day: 'THỨ BẢY' as DayOfWeek, slot: '18h - 19h' },
    { day: 'CHỦ NHẬT' as DayOfWeek, slot: '17h - 18h' },
  ];
  for (const s of nangThoYesSlots) {
    for (const m of nangThoMembers) {
      nangThoAvail[`${s.day}__${s.slot}__${m}`] = true;
    }
  }

  // Song 4: CẮT ĐÔI NỖI SẦU (Shares Quang Lực, Duy Thành, Bảo Ngọc)
  const catDoiNoiSauMembers = ['Quang Lực', 'Duy Thành', 'Bảo Ngọc', 'Tuấn Kiệt'];
  const catDoiNoiSauAvail = createEmptyVoteMatrix(days, slots, catDoiNoiSauMembers);
  const catDoiNoiSauYesSlots = [
    { day: 'THỨ TƯ' as DayOfWeek, slot: '17h - 18h' }, // Contends with Phonecert on Wed 17h
    { day: 'THỨ TƯ' as DayOfWeek, slot: '18h - 19h' }, // Contends with Phonecert
    { day: 'THỨ NĂM' as DayOfWeek, slot: '18h - 19h' },
    { day: 'THỨ NĂM' as DayOfWeek, slot: '19h - 20h' },
    { day: 'THỨ BẢY' as DayOfWeek, slot: '18h - 19h' },
    { day: 'THỨ BẢY' as DayOfWeek, slot: '20h - 21h' },
    { day: 'CHỦ NHẬT' as DayOfWeek, slot: '20h - 21h' },
  ];
  for (const s of catDoiNoiSauYesSlots) {
    for (const m of catDoiNoiSauMembers) {
      catDoiNoiSauAvail[`${s.day}__${s.slot}__${m}`] = true;
    }
  }

  // Song 5: NGÀY MAI NGƯỜI TA LẤY CHỒNG (Shares Minh Pháp, Thu Hà, Hoàng Long)
  const ngayMaiMembers = ['Minh Pháp', 'Hoàng Long', 'Thu Hà', 'Đức Anh'];
  const ngayMaiAvail = createEmptyVoteMatrix(days, slots, ngayMaiMembers);
  const ngayMaiYesSlots = [
    { day: 'THỨ BA' as DayOfWeek, slot: '17h - 18h' },
    { day: 'THỨ NĂM' as DayOfWeek, slot: '20h - 21h' },
    { day: 'THỨ SÁU' as DayOfWeek, slot: '17h - 18h' },
    { day: 'THỨ SÁU' as DayOfWeek, slot: '18h - 19h' }, // Contends with Phonecert Fri 18h
    { day: 'THỨ BẢY' as DayOfWeek, slot: '19h - 20h' },
    { day: 'CHỦ NHẬT' as DayOfWeek, slot: '18h - 19h' },
  ];
  for (const s of ngayMaiYesSlots) {
    for (const m of ngayMaiMembers) {
      ngayMaiAvail[`${s.day}__${s.slot}__${m}`] = true;
    }
  }

  return [
    {
      id: 'song-1',
      name: 'PHONECERT',
      weekTitle: DEFAULT_WEEK_TITLE,
      members: phonecertMembers,
      availability: phonecertAvail,
      notes: { 'THỨ TƯ__18h - 19h': 'Ưu tiên khớp nhạc cụ' },
      color: PASTEL_PALETTE[0], // Sky Blue
      targetSessions: 2, // As user specified: song can be 2 or 3 times a week
      sourceFileName: 'Vote_Phonecert.xlsx',
    },
    {
      id: 'song-2',
      name: 'BẬT TÌNH YÊU LÊN',
      weekTitle: DEFAULT_WEEK_TITLE,
      members: batTinhYeuLenMembers,
      availability: batTinhYeuLenAvail,
      notes: {},
      color: PASTEL_PALETTE[1], // Mint
      targetSessions: 1,
      sourceFileName: 'Vote_Bat_Tinh_Yeu_Len.xlsx',
    },
    {
      id: 'song-3',
      name: 'NÀNG THƠ',
      weekTitle: DEFAULT_WEEK_TITLE,
      members: nangThoMembers,
      availability: nangThoAvail,
      notes: { 'THỨ BẢY__18h - 19h': 'Tập mộc acoustic' },
      color: PASTEL_PALETTE[2], // Lavender
      targetSessions: 1,
      sourceFileName: 'Vote_Nang_Tho.xlsx',
    },
    {
      id: 'song-4',
      name: 'CẮT ĐÔI NỖI SẦU',
      weekTitle: DEFAULT_WEEK_TITLE,
      members: catDoiNoiSauMembers,
      availability: catDoiNoiSauAvail,
      notes: {},
      color: PASTEL_PALETTE[3], // Peach
      targetSessions: 2,
      sourceFileName: 'Vote_Cat_Doi_Noi_Sau.xlsx',
    },
    {
      id: 'song-5',
      name: 'NGÀY MAI NGƯỜI TA LẤY CHỒNG',
      weekTitle: DEFAULT_WEEK_TITLE,
      members: ngayMaiMembers,
      availability: ngayMaiAvail,
      notes: {},
      color: PASTEL_PALETTE[4], // Rose
      targetSessions: 1,
      sourceFileName: 'Vote_Ngay_Mai.xlsx',
    },
  ];
}

function songToExcelRows(song: SongVoteData) {
  const days = DAYS_OF_WEEK;
  const slots = DEFAULT_TIME_SLOTS;
  const members = song.members;

  const rows: (string | boolean | number)[][] = [];
  rows.push([song.weekTitle || DEFAULT_WEEK_TITLE]);
  rows.push([]);
  rows.push(['BÀI HÁT', song.name]);
  rows.push(['', 'KHUNG GIỜ', 'TÊN THÀNH VIÊN', ...Array(Math.max(0, members.length - 1)).fill(''), 'GHI CHÚ']);
  rows.push(['', '', ...members, '']);

  for (const day of days) {
    slots.forEach((slot, sIdx) => {
      const note = song.notes[`${day}__${slot}`] || '';
      const row: (string | boolean | number)[] = [
        sIdx === 0 ? day : '',
        slot,
        ...members.map(m => !!song.availability[`${day}__${slot}__${m}`]),
        note,
      ];
      rows.push(row);
    });
  }
  return rows;
}

export function createExcelFileFromSongs(
  fileName: string,
  songs: SongVoteData[]
): File {
  const wb = XLSX.utils.book_new();

  for (const song of songs) {
    const rows = songToExcelRows(song);
    const ws = XLSX.utils.aoa_to_sheet(rows);
    const sheetName = (song.name || 'Sheet1').replace(/[/\\?*[\]]/g, '_').substring(0, 31);
    XLSX.utils.book_append_sheet(wb, ws, sheetName);
  }

  const wbout = XLSX.write(wb, { bookType: 'xlsx', type: 'array' });
  return new File([wbout], fileName, {
    type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
  });
}

// Case 1: Multiple Excel files, each file has 1 tab
export function generateSingleTabSampleFiles(): File[] {
  const songs = generateSampleSongs();
  return songs.map(song => {
    const cleanName = song.name.replace(/\s+/g, '_');
    return createExcelFileFromSongs(`Vote_${cleanName}.xlsx`, [song]);
  });
}

// Case 2: One Excel file with multiple tabs (5 tabs)
export function generateMultiTabSampleFile(): File {
  const songs = generateSampleSongs();
  return createExcelFileFromSongs('Du_Lieu_Mau_CSAC_MultiTab_5_Bai.xlsx', songs);
}

// Case 3: Mixed files (File 1 has 2 tabs, File 2 has 3 tabs)
export function generateMixedSampleFiles(): File[] {
  const songs = generateSampleSongs();
  const file1 = createExcelFileFromSongs('Vote_Nhom_A_2_Tab.xlsx', [songs[0], songs[1]]);
  const file2 = createExcelFileFromSongs('Vote_Nhom_B_3_Tab.xlsx', [songs[2], songs[3], songs[4]]);
  return [file1, file2];
}
