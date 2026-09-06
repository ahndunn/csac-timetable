import React, { useState } from 'react';
import { SongVoteData } from '../types/timetable';
import { inspectExcelFiles, parseSelectedSheets, FileInspection } from '../services/excelParser';
import { SheetSelectionModal } from './SheetSelectionModal';
import { generateMultiTabSampleFile, generateSingleTabSampleFiles } from '../services/sampleData';
import { UploadCloud, FileSpreadsheet, Check, X, AlertCircle, Layers, Sparkles, Files } from 'lucide-react';

interface UploadModalProps {
  onClose: () => void;
  onAddSongs: (newSongs: SongVoteData[]) => void;
  onDownloadTemplate?: () => void;
  existingCount: number;
}

export const UploadModal: React.FC<UploadModalProps> = ({
  onClose,
  onAddSongs,
  onDownloadTemplate,
  existingCount,
}) => {
  const [isDragging, setIsDragging] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [parsedSongs, setParsedSongs] = useState<SongVoteData[]>([]);
  const [multiTabInspections, setMultiTabInspections] = useState<FileInspection[] | null>(null);

  const handleFiles = async (files: FileList | File[] | null) => {
    if (!files || files.length === 0) return;
    setIsLoading(true);
    setErrorMsg(null);

    try {
      const fileList = Array.from(files).filter(
        f => f.name.endsWith('.xlsx') || f.name.endsWith('.xls')
      );

      if (fileList.length === 0) {
        setErrorMsg('Vui lòng chọn file Excel có đuôi .xlsx hoặc .xls');
        setIsLoading(false);
        return;
      }

      const inspections = await inspectExcelFiles(fileList);

      // Check if any file has multiple sheets
      const hasMultiTab = inspections.some(f => f.hasMultipleSheets);

      if (hasMultiTab) {
        // Trigger multi-tab sheet selection dialog
        setMultiTabInspections(inspections);
      } else {
        // Single-tab files: parse all valid sheets directly
        const allKeys = new Set<string>();
        for (const f of inspections) {
          for (const s of f.sheets) {
            if (s.isValid) allKeys.add(`${f.fileId}::${s.sheetName}`);
          }
        }
        const songs = parseSelectedSheets(inspections, allKeys, existingCount + parsedSongs.length);
        if (songs.length === 0) {
          setErrorMsg('Không tìm thấy dữ liệu hợp lệ trong file. Vui lòng kiểm tra định dạng file Excel.');
        } else {
          setParsedSongs(prev => [...prev, ...songs]);
        }
      }
    } catch (err: unknown) {
      console.error(err);
      setErrorMsg('Đã có lỗi khi đọc file Excel. Vui lòng kiểm tra lại cấu trúc file.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
    handleFiles(e.dataTransfer.files);
  };

  const handleFileInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    handleFiles(e.target.files);
  };

  const handleConfirm = () => {
    if (parsedSongs.length > 0) {
      onAddSongs(parsedSongs);
      onClose();
    }
  };

  const handleTestMultiTab = () => {
    const file = generateMultiTabSampleFile();
    handleFiles([file]);
  };

  const handleTestSingleTab = () => {
    const files = generateSingleTabSampleFiles();
    handleFiles(files);
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-dialog" onClick={e => e.stopPropagation()}>
        <div className="modal-header">
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <FileSpreadsheet size={20} color="#1a73e8" />
            <h3 className="modal-header-title">Tải lên các file Excel vote lịch tập</h3>
          </div>
          <button className="modal-close-btn" onClick={onClose}>
            <X size={18} />
          </button>
        </div>

        <div className="modal-body">
          <div style={{ fontSize: 13, color: '#5f6368', lineHeight: 1.4 }}>
            Hệ thống hỗ trợ tải <strong>nhiều file Excel (.xlsx, .xls)</strong> cùng lúc. Cấu trúc mỗi file giống như mẫu: có tên bài hát, danh sách thành viên với checkbox rảnh, các thứ từ Thứ Hai đến Thứ Bảy/Chủ Nhật.
          </div>

          {onDownloadTemplate && (
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                padding: '10px 14px',
                backgroundColor: '#eff6ff',
                border: '1px solid #bfdbfe',
                borderRadius: 8,
                fontSize: 13,
                color: '#1e40af',
              }}
            >
              <span>Bạn muốn có file Excel dữ liệu mẫu (5 bài) để thử nghiệm?</span>
              <button
                type="button"
                className="btn-gcal-secondary"
                onClick={onDownloadTemplate}
                style={{
                  fontSize: 12,
                  padding: '5px 12px',
                  backgroundColor: '#ffffff',
                  borderColor: '#93c5fd',
                  color: '#1d4ed8',
                  display: 'flex',
                  alignItems: 'center',
                  gap: 6,
                }}
              >
                <FileSpreadsheet size={14} />
                <span>Tải file Excel mẫu</span>
              </button>
            </div>
          )}

          {/* Quick test buttons */}
          <div style={{ display: 'flex', gap: 8, alignItems: 'center', flexWrap: 'wrap' }}>
            <span style={{ fontSize: 12, color: '#64748b' }}>Thử nghiệm nhanh:</span>
            <button
              type="button"
              className="btn-gcal-sample"
              style={{ fontSize: 12, padding: '5px 10px', height: 'auto', borderRadius: 6 }}
              onClick={handleTestMultiTab}
              title="Thử nghiệm nạp 1 file Excel 5 tab để trải nghiệm hộp thoại chọn tab"
            >
              <Layers size={13} />
              <span>Test file nhiều tab</span>
            </button>
            <button
              type="button"
              className="btn-gcal-secondary"
              style={{ fontSize: 12, padding: '5px 10px', height: 'auto', borderRadius: 6 }}
              onClick={handleTestSingleTab}
              title="Thử nghiệm nạp 5 file Excel mỗi file 1 tab"
            >
              <Files size={13} />
              <span>Test 5 file (mỗi file 1 tab)</span>
            </button>
          </div>

          {/* Drag & Drop Zone */}
          <label
            className={`upload-dropzone ${isDragging ? 'dragover' : ''}`}
            onDragOver={e => {
              e.preventDefault();
              setIsDragging(true);
            }}
            onDragLeave={() => setIsDragging(false)}
            onDrop={handleDrop}
          >
            <input
              type="file"
              multiple
              accept=".xlsx, .xls"
              style={{ display: 'none' }}
              onChange={handleFileInputChange}
            />
            <div className="upload-icon">
              <UploadCloud size={36} />
            </div>
            <div className="upload-title">
              Kéo thả các file Excel vào đây hoặc bấm để chọn
            </div>
            <div className="upload-subtitle">
              Hỗ trợ định dạng .xlsx, .xls (có thể chọn nhiều file)
            </div>
          </label>

          {isLoading && (
            <div style={{ textAlign: 'center', fontSize: 13, color: 'var(--primary-blue)' }}>
              Đang phân tích dữ liệu từ file Excel...
            </div>
          )}

          {errorMsg && (
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 8,
                padding: '8px 12px',
                backgroundColor: '#fee2e2',
                border: '1px solid #fca5a5',
                borderRadius: 6,
                color: '#b91c1c',
                fontSize: 13,
              }}
            >
              <AlertCircle size={16} />
              <span>{errorMsg}</span>
            </div>
          )}

          {/* Parsed Songs Preview */}
          {parsedSongs.length > 0 && (
            <div>
              <div style={{ fontSize: 13, fontWeight: 600, color: '#374151', marginBottom: 8 }}>
                Các bài hát đã đọc thành công ({parsedSongs.length}):
              </div>

              <div className="uploaded-files-list">
                {parsedSongs.map((s, idx) => (
                  <div key={idx} className="file-row-item">
                    <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                      <div
                        style={{
                          width: 12,
                          height: 12,
                          borderRadius: 3,
                          backgroundColor: s.color.border,
                        }}
                      />
                      <strong style={{ fontSize: 13, color: '#1f2937' }}>{s.name}</strong>
                      <span style={{ fontSize: 11, color: '#6b7280' }}>
                        ({s.members.length} thành viên: {s.members.join(', ')})
                      </span>
                    </div>

                    <button
                      onClick={() => setParsedSongs(prev => prev.filter((_, i) => i !== idx))}
                      style={{ background: 'none', border: 'none', cursor: 'pointer', color: '#9ca3af' }}
                    >
                      <X size={14} />
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>

        <div className="modal-footer">
          <button className="btn-gcal-secondary" onClick={onClose}>
            Hủy
          </button>
          <button
            className="btn-gcal-primary"
            onClick={handleConfirm}
            disabled={parsedSongs.length === 0}
          >
            Thêm {parsedSongs.length} bài hát vào hệ thống
          </button>
        </div>
      </div>

      {multiTabInspections && (
        <SheetSelectionModal
          inspections={multiTabInspections}
          existingCount={existingCount + parsedSongs.length}
          onClose={() => setMultiTabInspections(null)}
          onConfirm={newSongs => {
            setParsedSongs(prev => [...prev, ...newSongs]);
            setMultiTabInspections(null);
          }}
        />
      )}
    </div>
  );
};
