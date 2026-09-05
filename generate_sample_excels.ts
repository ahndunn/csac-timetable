import * as XLSX from 'xlsx';
import * as fs from 'fs';
import * as path from 'path';

const outDir = path.resolve('sample_excel');
if (!fs.existsSync(outDir)) {
  fs.mkdirSync(outDir, { recursive: true });
}

function makeExcelFile(
  filename: string,
  songName: string,
  members: string[],
  yesSlots: { day: string; slot: string }[],
  note?: string
) {
  const days = ['THỨ HAI', 'THỨ BA', 'THỨ TƯ', 'THỨ NĂM', 'THỨ SÁU', 'THỨ BẢY'];
  const slots = ['17h - 18h', '18h - 19h', '19h - 20h', '20h - 21h'];

  const rows: any[][] = [];
  rows.push(['VOTE LỊCH TẬP TUẦN 1 (07/09/2026 - 12/09/2026)']);
  rows.push([]);
  rows.push(['BÀI HÁT', songName]);
  rows.push(['', 'KHUNG GIỜ', 'TÊN THÀNH VIÊN', ...Array(members.length - 1).fill(''), 'GHI CHÚ']);
  rows.push(['', '', ...members, '']);

  for (const day of days) {
    slots.forEach((slot, sIdx) => {
      const isYes = yesSlots.some(y => y.day === day && y.slot === slot);
      const row: any[] = [
        sIdx === 0 ? day : '',
        slot,
        ...members.map(() => isYes),
        isYes && note ? note : '',
      ];
      rows.push(row);
    });
  }

  const wb = XLSX.utils.book_new();
  const ws = XLSX.utils.aoa_to_sheet(rows);

  // Set col widths
  ws['!cols'] = [
    { wch: 14 },
    { wch: 14 },
    ...members.map(() => ({ wch: 14 })),
    { wch: 20 },
  ];

  XLSX.utils.book_append_sheet(wb, ws, songName.substring(0, 30));
  XLSX.writeFile(wb, path.join(outDir, filename));
  console.log(`Created ${filename}`);
}

// 1. Phonecert (matches user screenshot)
makeExcelFile(
  'Vote_PHONECERT.xlsx',
  'PHONECERT',
  ['Minh Pháp', 'Duy Thành', 'Gia Huy', 'Anh Pha', 'Quang Lực'],
  [
    { day: 'THỨ HAI', slot: '18h - 19h' },
    { day: 'THỨ HAI', slot: '19h - 20h' },
    { day: 'THỨ TƯ', slot: '17h - 18h' },
    { day: 'THỨ TƯ', slot: '18h - 19h' },
    { day: 'THỨ TƯ', slot: '19h - 20h' },
    { day: 'THỨ TƯ', slot: '20h - 21h' },
    { day: 'THỨ SÁU', slot: '18h - 19h' },
    { day: 'THỨ SÁU', slot: '19h - 20h' },
    { day: 'THỨ SÁU', slot: '20h - 21h' },
  ],
  'Ưu tiên khớp nhạc cụ'
);

// 2. Bật Tình Yêu Lên
makeExcelFile(
  'Vote_BAT_TINH_YEU_LEN.xlsx',
  'BẬT TÌNH YÊU LÊN',
  ['Minh Pháp', 'Anh Pha', 'Bảo Ngọc', 'Thu Hà'],
  [
    { day: 'THỨ HAI', slot: '19h - 20h' },
    { day: 'THỨ HAI', slot: '20h - 21h' },
    { day: 'THỨ BA', slot: '18h - 19h' },
    { day: 'THỨ BA', slot: '19h - 20h' },
    { day: 'THỨ NĂM', slot: '17h - 18h' },
    { day: 'THỨ NĂM', slot: '18h - 19h' },
    { day: 'THỨ BẢY', slot: '19h - 20h' },
  ]
);

// 3. Nàng Thơ
makeExcelFile(
  'Vote_NANG_THO.xlsx',
  'NÀNG THƠ',
  ['Duy Thành', 'Gia Huy', 'Hoàng Long', 'Kim Ngân'],
  [
    { day: 'THỨ HAI', slot: '17h - 18h' },
    { day: 'THỨ BA', slot: '19h - 20h' },
    { day: 'THỨ BA', slot: '20h - 21h' },
    { day: 'THỨ NĂM', slot: '19h - 20h' },
    { day: 'THỨ NĂM', slot: '20h - 21h' },
    { day: 'THỨ BẢY', slot: '17h - 18h' },
    { day: 'THỨ BẢY', slot: '18h - 19h' },
  ],
  'Tập mộc acoustic'
);
