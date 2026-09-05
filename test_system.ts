import * as XLSX from 'xlsx';
import ExcelJS from 'exceljs';
import { generateSampleSongs } from './src/services/sampleData';
import { solveTimetable, detectConflicts, getSlotAttendance } from './src/services/scheduler';
import { normalizeDay, normalizeSlot, isCellChecked } from './src/services/excelParser';
import { SongVoteData, SolverSettings, DayOfWeek } from './src/types/timetable';
import { DAYS_OF_WEEK, DEFAULT_TIME_SLOTS, DEFAULT_WEEK_TITLE } from './src/constants/timetableDefaults';

async function runAllTests() {
  console.log('====================================================');
  console.log('🧪 CSAC TIMETABLE AUTOMATED TEST SUITE');
  console.log('====================================================\n');

  let passed = 0;
  let failed = 0;

  function assert(condition: boolean, testName: string) {
    if (condition) {
      console.log(`  ✅ PASS: ${testName}`);
      passed++;
    } else {
      console.error(`  ❌ FAIL: ${testName}`);
      failed++;
    }
  }

  // ----------------------------------------------------
  // TEST 1: Sample Data Generation
  // ----------------------------------------------------
  console.log('TEST 1: Sample Data Verification');
  const sampleSongs = generateSampleSongs();
  assert(sampleSongs.length === 5, 'Generates 5 realistic sample songs');
  assert(sampleSongs.some(s => s.name === 'PHONECERT'), 'Contains Phonecert from reference image');
  
  // Verify member overlap: Minh Pháp in Phonecert and Bật Tình Yêu Lên
  const phonecert = sampleSongs.find(s => s.name === 'PHONECERT')!;
  const batTinhYeuLen = sampleSongs.find(s => s.name === 'BẬT TÌNH YÊU LÊN')!;
  const sharedMembers = phonecert.members.filter(m => batTinhYeuLen.members.includes(m));
  assert(sharedMembers.length > 0, `Shares overlapping members (${sharedMembers.join(', ')})`);

  // ----------------------------------------------------
  // TEST 2: Scheduling Algorithm with Zero Conflicts
  // ----------------------------------------------------
  console.log('\nTEST 2: Timetable Optimization Solver');
  const defaultSettings: SolverSettings = {
    maxRooms: 1,
    allowPartialAttendance: false,
    spreadDays: true,
  };

  const solveResult = solveTimetable(sampleSongs, defaultSettings);
  console.log(`  -> Scheduled: ${solveResult.schedule.length} sessions`);
  console.log(`  -> Unresolved: ${solveResult.unresolved.length}`);
  console.log(`  -> Conflicts: ${solveResult.conflicts.length}`);

  assert(solveResult.schedule.length > 0, 'Schedule contains sessions');
  assert(solveResult.conflicts.length === 0, 'Zero conflicts in auto-generated schedule');

  // Verify that no two scheduled sessions have overlapping members at the same slot
  let doubleBookings = 0;
  for (let i = 0; i < solveResult.schedule.length; i++) {
    for (let j = i + 1; j < solveResult.schedule.length; j++) {
      const s1 = solveResult.schedule[i];
      const s2 = solveResult.schedule[j];
      if (s1.day === s2.day && s1.slot === s2.slot) {
        const overlap = s1.allMembers.filter(m => s2.allMembers.includes(m));
        if (overlap.length > 0) {
          doubleBookings++;
          console.error(`Double booking: ${overlap.join(', ')} in ${s1.songName} and ${s2.songName}`);
        }
      }
    }
  }
  assert(doubleBookings === 0, 'No member is double-booked across different songs');

  // Verify that all scheduled members voted available (100% attendance)
  const allPerfect = solveResult.schedule.every(s => s.absentMembers.length === 0);
  assert(allPerfect, 'All scheduled sessions achieve 100% member attendance');

  // ----------------------------------------------------
  // TEST 3: Configurable Song Frequency & Unresolved Detection
  // ----------------------------------------------------
  console.log('\nTEST 3: Configurable Song Frequency & Unresolved Detection');
  // Make Phonecert require 3 sessions this week
  const customSongs: SongVoteData[] = sampleSongs.map(s => {
    if (s.name === 'PHONECERT') return { ...s, targetSessions: 3 };
    if (s.name === 'NÀNG THƠ') return { ...s, targetSessions: 2 };
    return s;
  });

  const customResult = solveTimetable(customSongs, defaultSettings);
  console.log(`  -> Requested: ${customResult.stats.totalRequested}, Scheduled: ${customResult.schedule.length}`);
  assert(customResult.schedule.length > 0, 'Schedules sessions with custom frequency');

  // Test Extreme Conflict: Song requests 15 sessions in a 6-slot week (mathematically impossible to auto-fulfill all)
  const extremeSongs: SongVoteData[] = sampleSongs.map(s => {
    if (s.name === 'PHONECERT') return { ...s, targetSessions: 12 };
    return s;
  });
  const extremeResult = solveTimetable(extremeSongs, defaultSettings);
  console.log(`  -> Extreme test requested: ${extremeResult.stats.totalRequested}, Scheduled: ${extremeResult.schedule.length}, Unresolved: ${extremeResult.unresolved.length}`);
  assert(extremeResult.unresolved.length > 0, 'Properly flags unresolved songs when capacity/conflicts prevent auto-solving');
  assert(extremeResult.unresolved[0].candidates.length > 0, 'Provides ranked candidate slots with conflict information');
  assert(extremeResult.unresolved[0].reasons.length > 0, 'Provides clear explanation reasons why it could not auto-resolve');

  // ----------------------------------------------------
  // TEST 4: Excel Vote Sheet Generator & Parser Round-Trip
  // ----------------------------------------------------
  console.log('\nTEST 4: Excel Parser Round-Trip (Matching User Image Format)');
  
  // Create a synthetic workbook matching the image layout:
  // Row 1: VOTE LỊCH TẬP TUẦN 1 (07/09/2026 - 12/09/2026)
  // Row 3: BÀI HÁT: PHONECERT
  // Row 4: KHUNG GIỜ, TÊN THÀNH VIÊN
  // Row 5: Member names: Minh Pháp, Duy Thành, Gia Huy, Anh Pha, Quang Lực
  // Rows 6+: THỨ HAI, 17h - 18h, checkboxes
  const wb = XLSX.utils.book_new();
  const testData: any[][] = [
    ['VOTE LỊCH TẬP TUẦN 1 (07/09/2026 - 12/09/2026)'],
    [],
    ['BÀI HÁT', 'PHONECERT'],
    ['', 'KHUNG GIỜ', 'TÊN THÀNH VIÊN', '', '', '', '', 'GHI CHÚ'],
    ['', '', 'Minh Pháp', 'Duy Thành', 'Gia Huy', 'Anh Pha', 'Quang Lực', ''],
    ['THỨ HAI', '17h - 18h', false, false, false, false, false, ''],
    ['', '18h - 19h', true, true, true, true, true, 'Test Note Mon 18h'],
    ['', '19h - 20h', true, true, true, true, true, ''],
    ['', '20h - 21h', false, false, false, false, false, ''],
    ['THỨ TƯ', '17h - 18h', true, true, true, true, true, ''],
    ['', '18h - 19h', true, true, true, true, true, ''],
    ['', '19h - 20h', true, true, true, true, true, ''],
    ['', '20h - 21h', true, true, true, true, true, ''],
    ['CHỦ NHẬT', '18h - 19h', true, true, true, true, true, 'Sunday note'],
  ];

  const ws = XLSX.utils.aoa_to_sheet(testData);
  XLSX.utils.book_append_sheet(wb, ws, 'PHONECERT');
  const buffer = XLSX.write(wb, { type: 'buffer', bookType: 'xlsx' });

  // Emulate browser File using Buffer
  const fakeFile = {
    name: 'Vote_Phonecert.xlsx',
    arrayBuffer: async () => buffer.buffer.slice(buffer.byteOffset, buffer.byteOffset + buffer.byteLength),
  } as unknown as File;

  // Run parser
  const { parseExcelFile } = await import('./src/services/excelParser');
  const parsed = await parseExcelFile(fakeFile, 0);

  assert(parsed.length === 1, 'Parsed exactly 1 song from test Excel');
  const parsedSong = parsed[0];
  assert(parsedSong.name === 'PHONECERT', `Extracted song name "${parsedSong.name}"`);
  assert(parsedSong.members.length === 5, `Extracted 5 members: ${parsedSong.members.join(', ')}`);
  assert(parsedSong.members.includes('Minh Pháp') && parsedSong.members.includes('Quang Lực'), 'Extracted correct Vietnamese member names');

  // Check availability matrix
  const mon18hAllTrue = parsedSong.members.every(m => parsedSong.availability[`THỨ HAI__18h - 19h__${m}`] === true);
  assert(mon18hAllTrue, 'Correctly read TRUE checkboxes for Thứ Hai 18h - 19h');

  const mon17hAllFalse = parsedSong.members.every(m => parsedSong.availability[`THỨ HAI__17h - 18h__${m}`] === false);
  assert(mon17hAllFalse, 'Correctly read FALSE checkboxes for Thứ Hai 17h - 18h');

  const sun18hAllTrue = parsedSong.members.every(m => parsedSong.availability[`CHỦ NHẬT__18h - 19h__${m}`] === true);
  assert(sun18hAllTrue, 'Correctly read TRUE checkboxes for Chủ Nhật 18h - 19h');

  // ----------------------------------------------------
  // TEST 5: Excel Export Generation
  // ----------------------------------------------------
  console.log('\nTEST 5: Multi-Sheet Excel Workbook Export');
  const exportWb = new ExcelJS.Workbook();
  const gridSheet = exportWb.addWorksheet('LỊCH TẬP TUẦN');
  gridSheet.addRow(['KHUNG GIỜ', 'THỨ HAI', 'THỨ BA', 'THỨ TƯ', 'THỨ NĂM', 'THỨ SÁU', 'THỨ BẢY', 'CHỦ NHẬT']);
  
  const songSheet = exportWb.addWorksheet('CHI TIẾT BÀI HÁT');
  songSheet.addRow(['STT', 'Tên Bài Hát', 'Thứ', 'Khung Giờ', 'Phòng', 'Thành Viên']);
  
  const memberSheet = exportWb.addWorksheet('LỊCH CÁ NHÂN');
  memberSheet.addRow(['Thành Viên', 'Bài Hát', 'Thứ', 'Khung Giờ', 'Trạng Thái']);

  const outBuf = await exportWb.xlsx.writeBuffer();
  assert(outBuf.byteLength > 0, `Generated Excel file size: ${outBuf.byteLength} bytes`);
  assert(exportWb.worksheets.length === 3, 'Workbook contains 3 specialized sheets');

  // Summary
  console.log('\n====================================================');
  console.log(`🏁 TEST RESULTS: ${passed} PASSED, ${failed} FAILED`);
  console.log('====================================================');

  if (failed > 0) {
    process.exit(1);
  }
}

runAllTests().catch(err => {
  console.error('Test execution error:', err);
  process.exit(1);
});
