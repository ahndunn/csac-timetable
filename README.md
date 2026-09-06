# CSAC Timetable Studio 🎵📅

Hệ thống tự động xếp lịch tập ban nhạc và câu lạc bộ âm nhạc thông minh từ file Excel vote lịch, giao diện phong cách Google Calendar gọn gàng, trực quan.

## ✨ Tính Năng Nổi Bật

- 🎨 **Giao diện Google Calendar trực quan**: Thiết kế dạng lưới tuần (Thứ Hai đến Chủ Nhật), bảng màu Pastel dịu mắt, không gradient, chuẩn phong cách Google Calendar.
- 📥 **Nạp nhiều file Excel đa tab thông minh**:
  - Hỗ trợ tải lên cùng lúc **nhiều file Excel**, mỗi file có thể chứa **nhiều tab (sheet)** khác nhau.
  - **Tự động nhận diện đa tab**: Khi phát hiện file tải lên có từ 2 tab trở lên, hệ thống sẽ mở **Hộp thoại Chọn tab (`SheetSelectionModal`)** để người dùng tick chọn chính xác các tab/bài hát muốn import.
  - Tự động nạp trực tiếp nếu tất cả các file đều là dạng đơn tab (1 file 1 tab).
- 🧪 **Menu dropdown "Test bằng dữ liệu mẫu"**:
  - 📑 **Excel có nhiều tab (Multi-tab)**: 1 file Excel gồm 5 tab — mở hộp thoại chọn tab để kiểm thử chọn bài.
  - 📁 **Nhiều file Excel (Mỗi file 1 tab)**: 5 file riêng biệt, mỗi file 1 bài — nạp trực tiếp và xếp lịch.
  - 🗂️ **Nhiều file hỗn hợp (1 tab & nhiều tab)**: 2 file gồm các nhóm bài khác nhau — kiểm thử chọn tab đa file.
  - 📥 **Tải file Excel mẫu (.xlsx)**: Tải file mẫu 5 bài hoàn chỉnh về máy tính.
- 🧠 **Thuật toán xếp lịch thông minh (Zero Member Conflict)**:
  - Một thành viên có thể tham gia nhiều bài hát cùng lúc mà không bao giờ bị trùng giờ tập giữa các bài.
  - Tự động tối ưu hóa tỉ lệ thành viên có mặt đạt 100%.
  - Tùy chỉnh số buổi tập/tuần cho từng bài riêng biệt (ví dụ: Bài A cần 1 buổi, Bài B cần 3 buổi).
- ⚠️ **Quản lý & Giải quyết xung đột (Conflict Resolver)**:
  - Tự động phát hiện khi các bài bị xung đột giờ hoặc không thể xếp đủ lịch.
  - Gợi ý các khung giờ tối ưu nhất kèm thông tin vắng mặt/trùng ai và cho phép gán lịch thủ công chỉ với 1 click.
- 📆 **Bộ chọn tuần (Mini Calendar Picker)**: Chọn nhanh tuần cần xếp lịch trực quan trên lịch tháng ở thanh bên trái. Tiêu đề tuần và khoảng ngày tự động đồng bộ.
- 📊 **Xuất file Excel đa trang chuyên nghiệp (.xlsx)**:
  - Sheet 1: **LỊCH TẬP TUẦN** (Dạng lưới thời khóa biểu có màu pastel và tên thành viên).
  - Sheet 2: **CHI TIẾT BÀI HÁT** (Bảng tổng hợp chi tiết theo bài, phòng, thứ, giờ).
  - Sheet 3: **LỊCH CÁ NHÂN** (Lịch tập chi tiết của từng thành viên trong tuần).

## 🚀 Cài Đặt & Chạy Dự Án

### Yêu Cầu
- Node.js >= 18.0
- npm hoặc yarn/pnpm

### Khởi Chạy Môi Trường Phát Triển
```bash
# Cài đặt dependencies
npm install

# Khởi chạy dev server
npm run dev
```

### Build Production & Kiểm Thử
```bash
# Kiểm tra build TypeScript & Vite
npm run build

# Chạy bộ kiểm thử tự động
npx tsx test_system.ts
```

## 🛠️ Công Nghệ Sử Dụng

- **Frontend Core**: React 19 + TypeScript + Vite
- **Styling**: Vanilla CSS (CSS Variables, Flexbox, CSS Grid, Responsive Design)
- **Excel Engine**: `xlsx` (SheetJS) & `exceljs`
- **Icons**: `lucide-react`
- **Testing**: `tsx` automated test suite

