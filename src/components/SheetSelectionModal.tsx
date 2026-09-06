import React, { useState, useMemo } from 'react';
import { FileInspection, parseSelectedSheets } from '../services/excelParser';
import { SongVoteData } from '../types/timetable';
import {
  Layers,
  FileSpreadsheet,
  CheckSquare,
  Square,
  Search,
  Check,
  X,
  Users,
  AlertCircle,
  Info,
} from 'lucide-react';

interface SheetSelectionModalProps {
  inspections: FileInspection[];
  existingCount: number;
  onClose: () => void;
  onConfirm: (selectedSongs: SongVoteData[]) => void;
}

export const SheetSelectionModal: React.FC<SheetSelectionModalProps> = ({
  inspections,
  existingCount,
  onClose,
  onConfirm,
}) => {
  // Initialize all valid sheets as selected by default
  const [selectedKeys, setSelectedKeys] = useState<Set<string>>(() => {
    const initial = new Set<string>();
    for (const file of inspections) {
      for (const sheet of file.sheets) {
        if (sheet.isValid) {
          initial.add(`${file.fileId}::${sheet.sheetName}`);
        }
      }
    }
    return initial;
  });

  const [searchQuery, setSearchQuery] = useState('');

  // Total valid sheets across all files
  const allValidSheetKeys = useMemo(() => {
    const keys: string[] = [];
    for (const file of inspections) {
      for (const sheet of file.sheets) {
        if (sheet.isValid) {
          keys.push(`${file.fileId}::${sheet.sheetName}`);
        }
      }
    }
    return keys;
  }, [inspections]);

  // Filtered files based on search
  const filteredFiles = useMemo(() => {
    if (!searchQuery.trim()) return inspections;
    const query = searchQuery.trim().toLowerCase();

    return inspections
      .map(file => {
        const matchingSheets = file.sheets.filter(
          sheet =>
            sheet.sheetName.toLowerCase().includes(query) ||
            sheet.songName.toLowerCase().includes(query) ||
            sheet.members.some(m => m.toLowerCase().includes(query))
        );
        return {
          ...file,
          sheets: matchingSheets,
        };
      })
      .filter(file => file.sheets.length > 0 || file.fileName.toLowerCase().includes(query));
  }, [inspections, searchQuery]);

  // Toggle a single sheet
  const handleToggleSheet = (key: string) => {
    setSelectedKeys(prev => {
      const next = new Set(prev);
      if (next.has(key)) {
        next.delete(key);
      } else {
        next.add(key);
      }
      return next;
    });
  };

  // Toggle all sheets in a specific file
  const handleToggleFile = (file: FileInspection) => {
    const validFileKeys = file.sheets.filter(s => s.isValid).map(s => `${file.fileId}::${s.sheetName}`);
    const allSelectedInFile = validFileKeys.every(k => selectedKeys.has(k));

    setSelectedKeys(prev => {
      const next = new Set(prev);
      if (allSelectedInFile) {
        validFileKeys.forEach(k => next.delete(k));
      } else {
        validFileKeys.forEach(k => next.add(k));
      }
      return next;
    });
  };

  // Global Select / Deselect all
  const handleSelectAll = () => {
    setSelectedKeys(new Set(allValidSheetKeys));
  };

  const handleDeselectAll = () => {
    setSelectedKeys(new Set());
  };

  // Confirm and parse selected sheets
  const handleConfirm = () => {
    const songs = parseSelectedSheets(inspections, selectedKeys, existingCount);
    onConfirm(songs);
    onClose();
  };

  const selectedCount = selectedKeys.size;
  const multiTabFilesCount = inspections.filter(f => f.hasMultipleSheets).length;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div
        className="modal-dialog sheet-selection-modal"
        onClick={e => e.stopPropagation()}
        style={{ maxWidth: 740, width: '92%' }}
      >
        {/* Header */}
        <div className="modal-header">
          <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
            <div className="sheet-modal-icon-badge">
              <Layers size={20} color="#1a73e8" />
            </div>
            <div>
              <h3 className="modal-header-title">Phát hiện file Excel có nhiều tab</h3>
              <p style={{ margin: 0, fontSize: 12, color: '#64748b' }}>
                Hệ thống tìm thấy {multiTabFilesCount > 0 ? `${multiTabFilesCount} file Excel nhiều tab` : 'các file Excel'}. Vui lòng chọn các tab (bài hát) cần nhập dữ liệu:
              </p>
            </div>
          </div>
          <button className="modal-close-btn" onClick={onClose}>
            <X size={18} />
          </button>
        </div>

        {/* Action Toolbar */}
        <div className="sheet-selection-toolbar">
          <div className="sheet-toolbar-left">
            <div className="sheet-search-box">
              <Search size={14} className="search-icon" />
              <input
                type="text"
                placeholder="Tìm tab, tên bài hoặc thành viên..."
                value={searchQuery}
                onChange={e => setSearchQuery(e.target.value)}
              />
              {searchQuery && (
                <button
                  type="button"
                  className="search-clear-btn"
                  onClick={() => setSearchQuery('')}
                >
                  <X size={12} />
                </button>
              )}
            </div>
          </div>

          <div className="sheet-toolbar-right">
            <span className="sheet-selection-count">
              Đã chọn: <strong>{selectedCount}</strong> / {allValidSheetKeys.length} tab
            </span>
            <button
              type="button"
              className="btn-text-action"
              onClick={handleSelectAll}
            >
              Chọn tất cả
            </button>
            <span style={{ color: '#cbd5e1' }}>|</span>
            <button
              type="button"
              className="btn-text-action"
              onClick={handleDeselectAll}
            >
              Bỏ chọn tất cả
            </button>
          </div>
        </div>

        {/* Modal Body: Grouped List by File */}
        <div className="modal-body sheet-selection-body" style={{ maxHeight: '55vh', overflowY: 'auto' }}>
          {filteredFiles.length === 0 ? (
            <div className="sheet-empty-search">
              <Info size={28} color="#94a3b8" />
              <p>Không tìm thấy tab nào khớp với từ khóa "{searchQuery}"</p>
            </div>
          ) : (
            filteredFiles.map(file => {
              const validFileKeys = file.sheets.filter(s => s.isValid).map(s => `${file.fileId}::${s.sheetName}`);
              const selectedInFileCount = validFileKeys.filter(k => selectedKeys.has(k)).length;
              const allSelectedInFile = validFileKeys.length > 0 && selectedInFileCount === validFileKeys.length;

              return (
                <div key={file.fileId} className="file-inspection-group">
                  {/* File Header Card */}
                  <div className="file-inspection-header">
                    <div className="file-info-left">
                      <FileSpreadsheet size={18} className="file-excel-icon" />
                      <span className="file-name-text" title={file.fileName}>
                        {file.fileName}
                      </span>
                      <span className="file-sheets-pill">
                        {file.sheets.length} tab {file.hasMultipleSheets && '• Multi-tab'}
                      </span>
                    </div>

                    <div className="file-info-right">
                      <span className="file-selected-status">
                        Đã chọn {selectedInFileCount}/{validFileKeys.length} tab
                      </span>
                      <button
                        type="button"
                        className="btn-file-toggle"
                        onClick={() => handleToggleFile(file)}
                      >
                        {allSelectedInFile ? 'Bỏ chọn file này' : 'Chọn hết file này'}
                      </button>
                    </div>
                  </div>

                  {/* Tabs List in this File */}
                  <div className="file-sheets-grid">
                    {file.sheets.map(sheet => {
                      const key = `${file.fileId}::${sheet.sheetName}`;
                      const isSelected = selectedKeys.has(key);

                      return (
                        <div
                          key={sheet.sheetName}
                          className={`sheet-item-card ${isSelected ? 'selected' : ''} ${
                            !sheet.isValid ? 'invalid' : ''
                          }`}
                          onClick={() => {
                            if (sheet.isValid) handleToggleSheet(key);
                          }}
                        >
                          <div className="sheet-item-checkbox">
                            {isSelected ? (
                              <CheckSquare size={18} className="checkbox-icon checked" />
                            ) : (
                              <Square size={18} className="checkbox-icon unchecked" />
                            )}
                          </div>

                          <div className="sheet-item-details">
                            <div className="sheet-item-top">
                              <span className="sheet-tab-name">
                                Tab: <strong>{sheet.sheetName}</strong>
                              </span>
                              {sheet.isValid ? (
                                <span className="sheet-member-badge" title={sheet.members.join(', ')}>
                                  <Users size={12} />
                                  <span>{sheet.members.length} thành viên</span>
                                </span>
                              ) : (
                                <span className="sheet-invalid-badge">
                                  <AlertCircle size={12} />
                                  <span>Không đúng định dạng vote</span>
                                </span>
                              )}
                            </div>

                            <div className="sheet-item-bottom">
                              <span className="sheet-song-name">
                                Bài hát: <strong>{sheet.songName}</strong>
                              </span>
                              {sheet.members.length > 0 && (
                                <span className="sheet-member-preview" title={sheet.members.join(', ')}>
                                  ({sheet.members.slice(0, 3).join(', ')}
                                  {sheet.members.length > 3 ? `, +${sheet.members.length - 3}` : ''})
                                </span>
                              )}
                            </div>
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </div>
              );
            })
          )}
        </div>

        {/* Footer */}
        <div className="modal-footer" style={{ justifyContent: 'space-between' }}>
          <div style={{ fontSize: 12, color: '#64748b', display: 'flex', alignItems: 'center', gap: 6 }}>
            <Info size={14} color="#94a3b8" />
            <span>Các tab không được chọn sẽ được bỏ qua, không nạp vào lịch.</span>
          </div>

          <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
            <button type="button" className="btn-gcal-secondary" onClick={onClose}>
              Hủy
            </button>
            <button
              type="button"
              className="btn-gcal-primary"
              onClick={handleConfirm}
              disabled={selectedCount === 0}
            >
              <Check size={16} />
              <span>Nhập {selectedCount} tab đã chọn</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
