export interface ActivityDay {
  kind: 'day';
  date: Date;
  isoDate: string;
  hours: number;
  level: 0 | 1 | 2 | 3;
}

export interface EmptyActivityDay {
  kind: 'empty';
  id: string;
}

export interface MonthHeader {
  key: string;
  label: string;
}

export interface ActivityCalendar {
  cells: Array<ActivityDay | EmptyActivityDay>;
  monthHeaders: MonthHeader[];
  totalHours: number;
  longestStreak: number;
  currentStreak: number;
}

function toDateAtNoon(date: Date) {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate(), 12);
}

function isoDate(date: Date) {
  return date.toISOString().slice(0, 10);
}

function activityLevel(hours: number): 0 | 1 | 2 | 3 {
  if (hours <= 0) return 0;
  if (hours <= 2) return 1;
  if (hours <= 5) return 2;
  return 3;
}

function buildMockHours() {
  return Array.from({ length: 365 }, (_, index) => {
    if ((index + 3) % 19 === 0 || index % 31 === 28) return 0;

    const seed = (index * 17 + (index % 5) * 9 + 11) % 13;

    if (seed < 5) return 0;
    if (seed < 9) return 1 + (index % 2);
    if (seed < 12) return 3 + (index % 2);
    return 6;
  });
}

function computeLongestStreak(hours: number[]) {
  let longest = 0;
  let streak = 0;

  for (const value of hours) {
    if (value > 0) {
      streak += 1;
      longest = Math.max(longest, streak);
    } else {
      streak = 0;
    }
  }

  return longest;
}

function computeCurrentStreak(hours: number[]) {
  let streak = 0;

  for (let index = hours.length - 1; index >= 0; index -= 1) {
    if (hours[index] <= 0) break;
    streak += 1;
  }

  return streak;
}

export const mockPlayActivityHours = buildMockHours();

export function buildActivityCalendar(
  hours: number[] = mockPlayActivityHours,
  referenceDate = new Date()
): ActivityCalendar {
  const today = toDateAtNoon(referenceDate);
  const firstDate = toDateAtNoon(new Date(today));
  firstDate.setDate(today.getDate() - (hours.length - 1));

  const leadingEmptyDays = firstDate.getDay();
  const trailingEmptyDays = (7 - ((leadingEmptyDays + hours.length) % 7)) % 7;
  const cells: Array<ActivityDay | EmptyActivityDay> = [];

  for (let index = 0; index < leadingEmptyDays; index += 1) {
    cells.push({ kind: 'empty', id: `leading-${index}` });
  }

  const days = hours.map((value, index) => {
    const date = toDateAtNoon(new Date(firstDate));
    date.setDate(firstDate.getDate() + index);

    return {
      kind: 'day' as const,
      date,
      isoDate: isoDate(date),
      hours: value,
      level: activityLevel(value)
    };
  });

  cells.push(...days);

  for (let index = 0; index < trailingEmptyDays; index += 1) {
    cells.push({ kind: 'empty', id: `trailing-${index}` });
  }

  const weekCount = Math.ceil(cells.length / 7);
  const monthHeaders: MonthHeader[] = [];
  let previousMonth = '';

  for (let weekIndex = 0; weekIndex < weekCount; weekIndex += 1) {
    const start = weekIndex * 7;
    const currentWeek = cells.slice(start, start + 7);
    const firstDay = currentWeek.find((cell): cell is ActivityDay => cell.kind === 'day');
    const label = firstDay
      ? firstDay.date.toLocaleDateString('en-US', { month: 'short' })
      : '';

    if (label && label !== previousMonth) {
      previousMonth = label;
      monthHeaders.push({ key: `month-${weekIndex}`, label });
      continue;
    }

    monthHeaders.push({ key: `month-${weekIndex}`, label: '' });
  }

  return {
    cells,
    monthHeaders,
    totalHours: hours.reduce((total, value) => total + value, 0),
    longestStreak: computeLongestStreak(hours),
    currentStreak: computeCurrentStreak(hours)
  };
}

export function formatActivityDate(date: Date) {
  return date.toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric'
  });
}
