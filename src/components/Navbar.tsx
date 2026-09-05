import React from 'react';
import { Upload, Wand2, Download, Sparkles, RefreshCw, FileSpreadsheet, Pencil } from 'lucide-react';

interface NavbarProps {
  weekTitle: string;
  onUpdateWeekTitle: (newTitle: string) => void;
  onOpenUpload: () => void;
  onRunScheduler: () => void;
  onExportExcel: () => void;
  onLoadSample: () => void;
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
  onLoadSample,
  onResetSchedule,
  onDownloadTemplate,
  isSolving,
}) => {
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
        <button
          className="btn-gcal-sample"
          onClick={onLoadSample}
          title="Nạp 5 bài hát mẫu có thành viên trùng nhau để thử nghiệm nhanh"
        >
          <Sparkles size={15} />
          <span>Dữ liệu mẫu</span>
        </button>

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
