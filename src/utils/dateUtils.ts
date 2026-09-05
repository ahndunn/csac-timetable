// Utility functions for week-based date calculations and formatting

export function getMonday(date: Date): Date {
  const d = new Date(date);
  const day = d.getDay(); // 0 is Sunday, 1 is Monday...
  const diff = d.getDate() - day + (day === 0 ? -6 : 1);
  d.setDate(diff);
  d.setHours(0, 0, 0, 0);
  return d;
}

export function addWeeks(date: Date, weeks: number): Date {
  const d = new Date(date);
  d.setDate(d.getDate() + weeks * 7);
  return d;
}

export function getWeekDays(monday: Date): Date[] {
  const days: Date[] = [];
  for (let i = 0; i < 7; i++) {
    const d = new Date(monday);
    d.setDate(monday.getDate() + i);
    days.push(d);
  }
  return days;
}

export function pad2(n: number): string {
  return n < 10 ? `0${n}` : `${n}`;
}

export function formatDateDDMM(date: Date): string {
  return `${pad2(date.getDate())}/${pad2(date.getMonth() + 1)}`;
}

export function formatDateFull(date: Date): string {
  return `${pad2(date.getDate())}/${pad2(date.getMonth() + 1)}/${date.getFullYear()}`;
}

export function formatWeekRange(monday: Date): string {
  const sunday = new Date(monday);
  sunday.setDate(monday.getDate() + 6);
  return `${formatDateFull(monday)} - ${formatDateFull(sunday)}`;
}

export function isSameDay(d1: Date, d2: Date): boolean {
  return (
    d1.getFullYear() === d2.getFullYear() &&
    d1.getMonth() === d2.getMonth() &&
    d1.getDate() === d2.getDate()
  );
}

export function isSameWeek(d1: Date, monday: Date): boolean {
  const m1 = getMonday(d1);
  const m2 = getMonday(monday);
  return isSameDay(m1, m2);
}

// Generate weeks matrix for a month in mini calendar
export interface CalendarCell {
  date: Date;
  isCurrentMonth: boolean;
  isToday: boolean;
  dayNumber: number;
}

export function getMonthMatrix(year: number, month: number): CalendarCell[][] {
  const firstDayOfMonth = new Date(year, month, 1);
  const startMonday = getMonday(firstDayOfMonth);

  const today = new Date();
  const weeks: CalendarCell[][] = [];
  let currentMonday = new Date(startMonday);

  // Generate 5-6 weeks
  for (let w = 0; w < 6; w++) {
    const week: CalendarCell[] = [];
    for (let d = 0; d < 7; d++) {
      const cellDate = new Date(currentMonday);
      cellDate.setDate(currentMonday.getDate() + d);
      week.push({
        date: cellDate,
        isCurrentMonth: cellDate.getMonth() === month,
        isToday: isSameDay(cellDate, today),
        dayNumber: cellDate.getDate(),
      });
    }

    // Stop if the next week is completely in the next month
    if (w >= 4 && week.every(c => !c.isCurrentMonth && c.date.getMonth() > month)) {
      break;
    }

    weeks.push(week);
    currentMonday.setDate(currentMonday.getDate() + 7);
  }

  return weeks;
}
