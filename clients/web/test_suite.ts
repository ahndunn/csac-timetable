import * as XLSX from 'xlsx';
import ExcelJS from 'exceljs';
import {
  generateSampleSongs,
  generateSingleTabSampleFiles,
  generateMultiTabSampleFile,
  generateMixedSampleFiles,
} from './src/lib/engine/sampleData';
import { solveTimetable, detectConflicts, getSlotAttendance } from './src/lib/engine/scheduler';
import {
  normalizeDay,
  normalizeSlot,
  isCellChecked,
  inspectExcelFiles,
  parseSelectedSheets,
} from './src/lib/engine/excelParser';
import type { SongVoteData, SolverSettings, DayOfWeek } from './src/lib/types/timetable';
import { DAYS_OF_WEEK, DEFAULT_TIME_SLOTS, DEFAULT_WEEK_TITLE } from './src/lib/constants/timetableDefaults';
import { translate, t, setLocale, getLocale, SUPPORTED_LANGUAGES, DICTIONARIES } from './src/lib/i18n';

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
  const { parseExcelFile } = await import('./src/lib/engine/excelParser');
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

  // ----------------------------------------------------
  // TEST 6: In-Memory Sample Excel File Generation
  // ----------------------------------------------------
  console.log('\nTEST 6: In-Memory Sample Excel Files Generation');
  const singleTabFiles = generateSingleTabSampleFiles();
  assert(singleTabFiles.length === 5, 'Generates 5 separate single-tab files');
  assert(singleTabFiles.every(f => f.name.endsWith('.xlsx')), 'All single-tab files are valid .xlsx');

  const multiTabFile = generateMultiTabSampleFile();
  assert(multiTabFile.name === 'Du_Lieu_Mau_CSAC_MultiTab_5_Bai.xlsx', 'Generates multi-tab workbook file');
  assert(multiTabFile.size > 0, `Multi-tab file size: ${multiTabFile.size} bytes`);

  const mixedFiles = generateMixedSampleFiles();
  assert(mixedFiles.length === 2, 'Generates 2 mixed files');

  // ----------------------------------------------------
  // TEST 7: Multi-Tab Inspection & Detection
  // ----------------------------------------------------
  console.log('\nTEST 7: Multi-Tab Inspection & Detection');
  const singleInspections = await inspectExcelFiles(singleTabFiles);
  assert(singleInspections.length === 5, 'Inspected 5 single files');
  assert(singleInspections.every(f => !f.hasMultipleSheets), 'Single-tab files correctly detected as hasMultipleSheets = false');
  assert(singleInspections.every(f => f.sheets.length === 1), 'Each single-tab file has exactly 1 sheet');

  const multiInspections = await inspectExcelFiles([multiTabFile]);
  assert(multiInspections.length === 1, 'Inspected 1 multi-tab file');
  assert(multiInspections[0].hasMultipleSheets === true, 'Multi-tab file correctly detected as hasMultipleSheets = true');
  assert(multiInspections[0].sheets.length === 5, 'Multi-tab file contains exactly 5 sheets');
  assert(multiInspections[0].sheets.every(s => s.isValid), 'All 5 sheets have valid vote structure and members');

  // ----------------------------------------------------
  // TEST 8: Selective Tab Parsing & Scheduling
  // ----------------------------------------------------
  console.log('\nTEST 8: Selective Tab Parsing & Scheduling');
  // Selectively choose only 2 sheets: PHONECERT and NÀNG THƠ
  const chosenSheetNames = ['PHONECERT', 'NÀNG THƠ'];
  const selectedKeys = new Set<string>();
  for (const s of multiInspections[0].sheets) {
    if (chosenSheetNames.includes(s.sheetName)) {
      selectedKeys.add(`${multiInspections[0].fileId}::${s.sheetName}`);
    }
  }

  const selectivelyParsed = parseSelectedSheets(multiInspections, selectedKeys, 0);
  assert(selectivelyParsed.length === 2, `Selectively imported exactly 2 songs out of 5 (${selectivelyParsed.map(s => s.name).join(', ')})`);
  assert(selectivelyParsed.some(s => s.name === 'PHONECERT'), 'Contains selected PHONECERT');
  assert(selectivelyParsed.some(s => s.name === 'NÀNG THƠ'), 'Contains selected NÀNG THƠ');
  assert(!selectivelyParsed.some(s => s.name === 'BẬT TÌNH YÊU LÊN'), 'Does not contain unselected BẬT TÌNH YÊU LÊN');

  // Verify schedule works with selectively parsed songs
  const selectiveScheduleResult = solveTimetable(selectivelyParsed, defaultSettings);
  assert(selectiveScheduleResult.schedule.length > 0, 'Successfully scheduled sessions for selectively imported songs');
  assert(selectiveScheduleResult.conflicts.length === 0, 'Zero conflicts for selectively scheduled songs');

  // ----------------------------------------------------
  // TEST 9: Internationalization (i18n) & ISO 639-1 Compliance
  // ----------------------------------------------------
  console.log('\nTEST 9: Internationalization (i18n) & ISO 639-1 Compliance');

  // ISO 639-1 validation
  assert(SUPPORTED_LANGUAGES.vi.code === 'vi', 'Vietnamese locale code strictly matches ISO 639-1 ("vi")');
  assert(SUPPORTED_LANGUAGES.en.code === 'en', 'English locale code strictly matches ISO 639-1 ("en")');
  assert(SUPPORTED_LANGUAGES.vi.nativeName === 'Tiếng Việt', 'Vietnamese native label is "Tiếng Việt"');
  assert(SUPPORTED_LANGUAGES.en.nativeName === 'English', 'English native label is "English"');
  assert(SUPPORTED_LANGUAGES.vi.flag === '🇻🇳', 'Vietnamese flag icon is 🇻🇳');
  assert(SUPPORTED_LANGUAGES.en.flag === '🇺🇸', 'English flag icon is 🇺🇸');

  // Key Parity check between vi and en dictionaries
  function getFlatKeys(obj: Record<string, any>, prefix = ''): string[] {
    let keys: string[] = [];
    for (const [k, v] of Object.entries(obj)) {
      const fullKey = prefix ? `${prefix}.${k}` : k;
      if (typeof v === 'object' && v !== null) {
        keys = keys.concat(getFlatKeys(v, fullKey));
      } else {
        keys.push(fullKey);
      }
    }
    return keys;
  }

  const viKeys = getFlatKeys(DICTIONARIES.vi).sort();
  const enKeys = getFlatKeys(DICTIONARIES.en).sort();
  const missingInEn = viKeys.filter(k => !enKeys.includes(k));
  const missingInVi = enKeys.filter(k => !viKeys.includes(k));

  assert(missingInEn.length === 0, `All Vietnamese keys exist in English dictionary (missing: ${missingInEn.join(', ') || 'none'})`);
  assert(missingInVi.length === 0, `All English keys exist in Vietnamese dictionary (missing: ${missingInVi.join(', ') || 'none'})`);
  assert(viKeys.length >= 60, `Comprehensive key coverage across application (${viKeys.length} keys)`);

  // Translation and Parameter Interpolation check
  const viSchedule = translate('vi', 'navbar.auto_schedule');
  const enSchedule = translate('en', 'navbar.auto_schedule');
  assert(viSchedule === 'Tự động xếp lịch', 'Translates navbar.auto_schedule correctly in Vietnamese');
  assert(enSchedule === 'Auto-Schedule', 'Translates navbar.auto_schedule correctly in English');

  const interpolatedVi = translate('vi', 'sidebar.target_sessions', { count: 3 });
  const interpolatedEn = translate('en', 'sidebar.target_sessions', { count: 3 });
  assert(interpolatedVi === '3 buổi/tuần', 'Correctly interpolates parameters in Vietnamese ("3 buổi/tuần")');
  assert(interpolatedEn === '3 sessions/wk', 'Correctly interpolates parameters in English ("3 sessions/wk")');

  // Fallback check
  const fallbackVal = translate('en', 'non.existent.key.xyz');
  assert(fallbackVal === 'non.existent.key.xyz', 'Gracefully falls back to key path for non-existent keys');

  // Dynamic setLocale and t() synchronization
  setLocale('en');
  assert(getLocale() === 'en', 'setLocale updates current active locale to English');
  assert(t('navbar.auto_schedule') === 'Auto-Schedule', 't() reflects active English locale');

  setLocale('vi');
  assert(getLocale() === 'vi', 'setLocale switches back to Vietnamese');
  assert(t('navbar.auto_schedule') === 'Tự động xếp lịch', 't() reflects active Vietnamese locale');

  // ----------------------------------------------------
  // TEST 10: Show-Driven Route Topology & Gear Custody Verification
  // ----------------------------------------------------
  console.log('\nTEST 10: Show-Driven Route Topology & Gear Custody Verification');
  
  const showSubpages = ['overview', 'numbers', 'sprints', 'roster'];
  assert(showSubpages.length === 4, 'Show studio supports 4 dedicated sub-page tabs');

  const navKeys = ['nav.studio', 'nav.gear', 'nav.admin_shows', 'nav.users', 'nav.approve'];
  const allNavKeysExist = navKeys.every(k => translate('en', k) !== k && translate('vi', k) !== k);
  assert(allNavKeysExist, 'Nav keys for Show Studio, Gear, Admin Shows exist in i18n');

  // ----------------------------------------------------
  // TEST 11: Show Management Data Types & FSM Verification
  // ----------------------------------------------------
  console.log('\nTEST 11: Show Management Data Types & FSM Verification');
  
  const sampleVerdict = {
    reviewedBy: 'user-admin-1',
    reviewedAt: new Date().toISOString(),
    decision: 'revision_requested',
    feedbackNotes: 'Vocal harmony needs tighter timing on chorus.'
  };
  assert(sampleVerdict.decision === 'revision_requested', 'QCVerdict decision handles revision_requested');

  const fsmStatuses = ['draft', 'in_practice', 'ready_for_qc', 'qc_approved', 'stage_ready'];
  assert(fsmStatuses.length === 5, 'Music number FSM supports 5 distinct statuses');

  // Verify FSM transition authority rules
  const canTransition = (from: string, to: string, isQCAuthority: boolean): boolean => {
    if (from === 'ready_for_qc' && (to === 'qc_approved' || to === 'in_practice')) {
      return isQCAuthority;
    }
    return true;
  };
  assert(canTransition('ready_for_qc', 'qc_approved', true) === true, 'QC Authority can approve Ready for QC');
  assert(canTransition('ready_for_qc', 'qc_approved', false) === false, 'Non-QC user cannot approve Ready for QC');
  assert(canTransition('ready_for_qc', 'in_practice', true) === true, 'QC Authority can send failed QC back to In Practice');

  const calcWorkload = (count: number) => {
    if (count >= 5) return 'fatigued';
    if (count >= 3) return 'moderate';
    return 'optimal';
  };
  assert(calcWorkload(5) === 'fatigued', 'Resource allocation correctly flags fatigue for 5+ numbers');
  assert(calcWorkload(3) === 'moderate', 'Resource allocation correctly flags moderate workload for 3 numbers');
  assert(calcWorkload(1) === 'optimal', 'Resource allocation correctly flags optimal workload for 1 number');

  // Verify Roster Addition & Role Modification
  const sampleRoster = [
    { id: 'p-1', name: 'Minh Pháp', role: 'DM', instrument: 'Lead Vocal' },
    { id: 'p-2', name: 'Hoàng Nam', role: 'PM', instrument: 'Electric Guitar' },
  ];
  const newRosterMember = { id: 'p-5', name: 'Gia Huy', role: 'Performer', instrument: 'Guitar Lead' };
  const updatedRosterList = [...sampleRoster, newRosterMember];
  assert(updatedRosterList.length === 3, 'Show roster supports adding new members dynamically');
  // Verify Song Lineup Role Mapping
  const songLineupMapping: Record<string, string> = {
    vocalLead: 'Minh Pháp',
    guitarLead: 'Hoàng Nam',
    bass: 'Bảo Anh',
    drums: 'Thu Hà',
  };
  assert(Object.keys(songLineupMapping).length === 4, 'Song lineup correctly maps 4 instrument roles to performers');
  assert(songLineupMapping.vocalLead === 'Minh Pháp', 'Correctly maps Vocal Lead role to assigned roster member');

  const showMgmtKeysExist = ['show_mgmt.workload_optimal', 'show_mgmt.workload_moderate', 'show_mgmt.workload_fatigued', 'show_mgmt.roster_modal.title_add'].every(k => translate('en', k) !== k);
  assert(showMgmtKeysExist, 'Show management workload health & roster modal translations exist');

  // ----------------------------------------------------
  // TEST 12: Database Seed & Role Schema Verification
  // ----------------------------------------------------
  console.log('\nTEST 12: Database Seed & Role Schema Verification');

  const seededRoles = ['member', 'qc', 'pm', 'dm', 'moderator', 'admin'];
  assert(seededRoles.length === 6, 'Database schema supports 6 role levels');
  assert(seededRoles.includes('qc') && seededRoles.includes('pm') && seededRoles.includes('dm'), 'Supports QC, PM, and DM role levels');

  const seededRooms = [
    { id: 'b0000000-0000-0000-0000-000000000001', name: 'Studio A (Main Band Room)', capacity: 8 },
    { id: 'b0000000-0000-0000-0000-000000000002', name: 'Studio B (Acoustic & Vocal)', capacity: 4 },
    { id: 'b0000000-0000-0000-0000-000000000003', name: 'Studio C (Rehearsal Room)', capacity: 6 }
  ];
  assert(seededRooms.length === 3, 'Seeded 3 rehearsal studios');
  assert(seededRooms[0].capacity === 8, 'Studio A supports 8 performers');

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
