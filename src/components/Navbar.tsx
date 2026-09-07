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
  Menu,
  MoreVertical,
  X,
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
  onToggleSidebar?: () => void;
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
  onToggleSidebar,
}) => {
  const [isSampleOpen, setIsSampleOpen] = useState(false);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const sampleDropdownRef = useRef<HTMLDivElement>(null);
  const mobileMenuRef = useRef<HTMLDivElement>(null);

  // Close dropdown on outside click or Esc
  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (sampleDropdownRef.current && !sampleDropdownRef.current.contains(e.target as Node)) {
        setIsSampleOpen(false);
      }
      if (mobileMenuRef.current && !mobileMenuRef.current.contains(e.target as Node)) {
        setIsMobileMenuOpen(false);
      }
    };

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        setIsSampleOpen(false);
        setIsMobileMenuOpen(false);
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
        {onToggleSidebar && (
          <button
            type="button"
            className="navbar-hamburger-btn"
            onClick={onToggleSidebar}
            title="Mở menu danh sách bài hát & cài đặt"
            aria-label="Menu"
          >
            <Menu size={20} />
          </button>
        )}

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

      {/* Desktop Actions */}
      <div className="navbar-actions navbar-actions-desktop">
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

      {/* Mobile Actions (Visible on screen < 768px) */}
      <div className="navbar-actions navbar-actions-mobile">
        <button
          className="btn-gcal-primary btn-mobile-solve"
          onClick={onRunScheduler}
          disabled={isSolving}
          title="Tự động xếp lịch"
        >
          <Wand2 size={15} />
          <span>{isSolving ? 'Xếp...' : 'Xếp lịch'}</span>
        </button>

        <div className="mobile-menu-container" ref={mobileMenuRef}>
          <button
            type="button"
            className={`btn-gcal-secondary btn-mobile-more ${isMobileMenuOpen ? 'active' : ''}`}
            onClick={e => {
              e.stopPropagation();
              setIsMobileMenuOpen(prev => !prev);
            }}
            title="Menu tác vụ khác"
            aria-label="Thao tác khác"
          >
            <MoreVertical size={18} />
          </button>

          {isMobileMenuOpen && (
            <div className="mobile-menu-dropdown" onClick={e => e.stopPropagation()}>
              <button
                type="button"
                className="mobile-menu-item"
                onClick={() => {
                  setIsMobileMenuOpen(false);
                  onOpenUpload();
                }}
              >
                <Upload size={16} color="#1a73e8" />
                <span>Tải file Excel lên</span>
              </button>

              <button
                type="button"
                className="mobile-menu-item"
                onClick={() => {
                  setIsMobileMenuOpen(false);
                  onExportExcel();
                }}
              >
                <Download size={16} color="#059669" />
                <span>Xuất file Excel</span>
              </button>

              <div className="mobile-menu-divider" />
              <div className="mobile-menu-label">DỮ LIỆU MẪU</div>

              <button
                type="button"
                className="mobile-menu-item"
                onClick={() => {
                  setIsMobileMenuOpen(false);
                  onLoadSampleMultiTab();
                }}
              >
                <Layers size={16} color="#2563eb" />
                <span>Test file Excel nhiều tab</span>
              </button>

              <button
                type="button"
                className="mobile-menu-item"
                onClick={() => {
                  setIsMobileMenuOpen(false);
                  onLoadSampleSingleTab();
                }}
              >
                <Files size={16} color="#0284c7" />
                <span>Test 5 file (mỗi file 1 tab)</span>
              </button>

              <button
                type="button"
                className="mobile-menu-item"
                onClick={() => {
                  setIsMobileMenuOpen(false);
                  onLoadSampleMixed();
                }}
              >
                <FolderSync size={16} color="#7c3aed" />
                <span>Test file hỗn hợp</span>
              </button>

              <button
                type="button"
                className="mobile-menu-item"
                onClick={() => {
                  setIsMobileMenuOpen(false);
                  onDownloadTemplate();
                }}
              >
                <FileSpreadsheet size={16} color="#10b981" />
                <span>Tải file Excel mẫu (.xlsx)</span>
              </button>

              <div className="mobile-menu-divider" />

              <button
                type="button"
                className="mobile-menu-item danger"
                onClick={() => {
                  setIsMobileMenuOpen(false);
                  onResetSchedule();
                }}
              >
                <RefreshCw size={16} />
                <span>Xóa / Làm mới lịch</span>
              </button>
            </div>
          )}
        </div>
      </div>
    </header>
  );
};
