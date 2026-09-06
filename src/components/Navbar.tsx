import React, { useState, useRef, useEffect } from 'react';
import {
  Upload,
  Wand2,
  Download,
  Sparkles,
  RefreshCw,
  FileSpreadsheet,
  Pencil,
  ChevronDown,
  Layers,
  Files,
  FolderSync,
} from 'lucide-react';

interface NavbarProps {
  weekTitle: string;
  onUpdateWeekTitle: (newTitle: string) => void;
  onOpenUpload: () => void;
  onRunScheduler: () => void;
  onExportExcel: () => void;
  onLoadSampleSingleTab: () => void;
  onLoadSampleMultiTab: () => void;
  onLoadSampleMixed: () => void;
  onResetSchedule: () => void;
  onDownloadTemplate: () => void;
  isSolving?: boolean;
}

export const Navbar: React.FC<NavbarProps> = ({
  weekTitle,
  onUpdateWeekTitle,
  onOpenUpload,
  onRunScheduler,
  onExportExcel,
  onLoadSampleSingleTab,
  onLoadSampleMultiTab,
  onLoadSampleMixed,
  onResetSchedule,
  onDownloadTemplate,
  isSolving,
}) => {
  const [isSampleOpen, setIsSampleOpen] = useState(false);
  const sampleDropdownRef = useRef<HTMLDivElement>(null);

  // Close dropdown on outside click or Esc
  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (sampleDropdownRef.current && !sampleDropdownRef.current.contains(e.target as Node)) {
        setIsSampleOpen(false);
      }
    };

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        setIsSampleOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, []);

  return (
    <header className="navbar">
      <div className="navbar-left">
        {/* Editable Document / Week Title */}
        <div className="title-edit-wrapper">
          <input
            type="text"
            className="title-edit-input"
            value={weekTitle}
            onChange={e => onUpdateWeekTitle(e.target.value)}
            placeholder="Nhập tiêu đề lịch tập..."
            title="Bấm để chỉnh sửa tên lịch tập (tiêu đề này sẽ được cập nhật khi xuất file Excel)"
          />
          <Pencil size={13} className="title-edit-icon" />
        </div>
      </div>

      <div className="navbar-actions">
        {/* Dropdown: Test bằng dữ liệu mẫu */}
        <div className="sample-dropdown-container" ref={sampleDropdownRef}>
          <button
            type="button"
            className={`btn-gcal-sample ${isSampleOpen ? 'active' : ''}`}
            onClick={e => {
              e.stopPropagation();
              setIsSampleOpen(prev => !prev);
            }}
            title="Thử nghiệm xếp lịch bằng các bộ dữ liệu mẫu (mỗi file 1 tab hoặc file nhiều tab)"
          >
            <Sparkles size={15} />
            <span>Test bằng dữ liệu mẫu</span>
            <ChevronDown
              size={14}
              style={{
                marginLeft: 2,
                transform: isSampleOpen ? 'rotate(180deg)' : 'none',
                transition: 'transform 0.2s',
              }}
            />
          </button>

          {isSampleOpen && (
            <div className="sample-dropdown-menu" onClick={e => e.stopPropagation()}>
              <div className="sample-dropdown-header">
                CHỌN KỊCH BẢN TEST DỮ LIỆU MẪU
              </div>

              {/* Case 1: Multi-tab */}
              <button
                type="button"
                className="sample-dropdown-item"
                onClick={() => {
                  setIsSampleOpen(false);
                  onLoadSampleMultiTab();
                }}
              >
                <div className="sample-dropdown-icon multi-tab">
                  <Layers size={18} />
                </div>
                <div className="sample-dropdown-text">
                  <div className="sample-dropdown-title">
                    Excel có nhiều tab (Multi-tab)
                    <span className="sample-badge-highlight">Hiện chọn tab</span>
                  </div>
                  <div className="sample-dropdown-desc">
                    1 file Excel gồm 5 tab — Mở hộp thoại chọn tab để bạn check chọn bài cần import
                  </div>
                </div>
              </button>

              {/* Case 2: Single-tab files */}
              <button
                type="button"
                className="sample-dropdown-item"
                onClick={() => {
                  setIsSampleOpen(false);
                  onLoadSampleSingleTab();
                }}
              >
                <div className="sample-dropdown-icon single-tab">
                  <Files size={18} />
                </div>
                <div className="sample-dropdown-text">
                  <div className="sample-dropdown-title">
                    Nhiều file Excel (Mỗi file 1 tab)
                  </div>
                  <div className="sample-dropdown-desc">
                    5 file Excel rời rạc (.xlsx), mỗi file chứa 1 tab bài hát — Nạp trực tiếp & xếp lịch
                  </div>
                </div>
              </button>

              {/* Case 3: Mixed files */}
              <button
                type="button"
                className="sample-dropdown-item"
                onClick={() => {
                  setIsSampleOpen(false);
                  onLoadSampleMixed();
                }}
              >
                <div className="sample-dropdown-icon mixed">
                  <FolderSync size={18} />
                </div>
                <div className="sample-dropdown-text">
                  <div className="sample-dropdown-title">
                    Nhiều file hỗn hợp (1 tab & nhiều tab)
                  </div>
                  <div className="sample-dropdown-desc">
                    2 file Excel gồm các nhóm bài khác nhau — Kiểm thử chọn tab giữa nhiều file
                  </div>
                </div>
              </button>

              <div className="sample-dropdown-divider" />

              {/* Download template */}
              <button
                type="button"
                className="sample-dropdown-item sample-dropdown-download"
                onClick={() => {
                  setIsSampleOpen(false);
                  onDownloadTemplate();
                }}
              >
                <div className="sample-dropdown-icon download">
                  <FileSpreadsheet size={18} />
                </div>
                <div className="sample-dropdown-text">
                  <div className="sample-dropdown-title">
                    Tải file Excel mẫu (.xlsx) về máy
                  </div>
                  <div className="sample-dropdown-desc">
                    File Excel 5 bài hoàn chỉnh để bạn mở bằng Microsoft Excel xem thử
                  </div>
                </div>
              </button>
            </div>
          )}
        </div>

        <button
          className="btn-gcal-secondary"
          onClick={onDownloadTemplate}
          title="Tải về file Excel dữ liệu mẫu (.xlsx) 5 bài để điền hoặc xem thử"
        >
          <FileSpreadsheet size={15} />
          <span>Tải file mẫu</span>
        </button>

        <button
          className="btn-gcal-secondary"
          onClick={onOpenUpload}
          title="Tải lên các file Excel vote lịch tập của nhóm"
        >
          <Upload size={15} />
          <span>Tải file lên</span>
        </button>

        <button
          className="btn-gcal-primary"
          onClick={onRunScheduler}
          disabled={isSolving}
          title="Giải thuật tự động phân bổ lịch tập không trùng thành viên"
        >
          <Wand2 size={15} />
          <span>{isSolving ? 'Đang xếp...' : 'Xếp lịch'}</span>
        </button>

        <button
          className="btn-gcal-secondary"
          onClick={onExportExcel}
          title="Xuất lịch tập hoàn chỉnh ra file Excel (.xlsx)"
        >
          <Download size={15} />
          <span>Xuất Excel</span>
        </button>

        <button
          className="nav-arrow-btn"
          onClick={onResetSchedule}
          title="Xóa / Làm mới lịch"
        >
          <RefreshCw size={15} />
        </button>
      </div>
    </header>
  );
};
