import type { Game, GameSession } from "$lib/types/Game";

export interface ActivityDay {
	kind: "day";
	date: Date;
	isoDate: string;
	hours: number;
	level: 0 | 1 | 2 | 3;
}

export interface EmptyActivityDay {
	kind: "empty";
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

function startOfDay(date: Date) {
	return new Date(date.getFullYear(), date.getMonth(), date.getDate());
}

function isoDate(date: Date) {
	return date.toISOString().slice(0, 10);
}

function formatMinutesAsHours(minutes: number) {
	return Math.round((minutes / 60) * 10) / 10;
}

function activityLevel(hours: number): 0 | 1 | 2 | 3 {
	if (hours <= 0) return 0;
	if (hours <= 2) return 1;
	if (hours <= 5) return 2;
	return 3;
}

function normalizeSessionDuration(session: GameSession, now: number) {
	if (session.duration > 0) {
		return session.duration;
	}

	if (session.endTime && session.endTime > session.startTime) {
		return session.endTime - session.startTime;
	}

	if (!session.endTime && now > session.startTime) {
		return now - session.startTime;
	}

	return 0;
}

function addMinutesToDateBucket(
	activityByDate: Map<string, number>,
	date: Date,
	minutes: number,
) {
	const key = isoDate(date);
	activityByDate.set(key, (activityByDate.get(key) || 0) + minutes);
}

function accumulateSessionAcrossDays(
	activityByDate: Map<string, number>,
	session: GameSession,
	now: number,
) {
	const effectiveEnd =
		session.endTime && session.endTime > session.startTime
			? session.endTime
			: now;
	const durationMs = normalizeSessionDuration(session, now);

	if (durationMs <= 0 || effectiveEnd <= session.startTime) {
		return;
	}

	let cursor = session.startTime;

	while (cursor < effectiveEnd) {
		const current = new Date(cursor);
		const nextDay = new Date(
			current.getFullYear(),
			current.getMonth(),
			current.getDate() + 1,
		).getTime();
		const segmentEnd = Math.min(effectiveEnd, nextDay);
		const minutes = (segmentEnd - cursor) / 60000;

		if (minutes > 0) {
			addMinutesToDateBucket(activityByDate, current, minutes);
		}

		cursor = segmentEnd;
	}
}

export function getInstalledGames(items: Game[]) {
	return items.filter((game) => game.inLibrary !== false);
}

export function calculateGamesPlayed(items: Game[]) {
	return getInstalledGames(items).filter(
		(game) => (game.storageTotalPlaytimeMinutes ?? 0) > 0,
	).length;
}

export function calculateActiveNow(
	items: Game[],
	isGameRunning: boolean,
	activeGameId?: string | null,
) {
	const activeIds = new Set<string>();

	if (isGameRunning && activeGameId) {
		activeIds.add(activeGameId.toLowerCase());
	}

	for (const game of getInstalledGames(items)) {
		if (game.storageSessions?.some((session) => session.endTime === null)) {
			activeIds.add(game.id.toLowerCase());
		}
	}

	return activeIds.size;
}

export function calculateMostPlayed(items: Game[]) {
	return [...getInstalledGames(items)].sort(
		(left, right) =>
			(right.storageTotalPlaytimeMinutes ?? 0) -
			(left.storageTotalPlaytimeMinutes ?? 0),
	)[0];
}

export function generateActivityByDate(
	items: Game[],
	referenceDate = new Date(),
) {
	const activityByDate = new Map<string, number>();
	const now = referenceDate.getTime();

	for (const game of getInstalledGames(items)) {
		for (const session of game.storageSessions ?? []) {
			accumulateSessionAcrossDays(activityByDate, session, now);
		}
	}

	return activityByDate;
}

export function calculateLongestStreak(activityByDate: Map<string, number>) {
	const dates = [...activityByDate.entries()]
		.filter(([, minutes]) => minutes > 0)
		.map(([value]) => value)
		.sort();

	if (!dates.length) {
		return 0;
	}

	let longest = 1;
	let current = 1;

	for (let index = 1; index < dates.length; index += 1) {
		const previous = startOfDay(new Date(dates[index - 1]));
		const next = startOfDay(new Date(dates[index]));
		const diffDays = Math.round(
			(next.getTime() - previous.getTime()) / (24 * 60 * 60 * 1000),
		);

		if (diffDays === 1) {
			current += 1;
			longest = Math.max(longest, current);
			continue;
		}

		current = 1;
	}

	return longest;
}

export function calculateCurrentStreak(
	activityByDate: Map<string, number>,
	referenceDate = new Date(),
) {
	let streak = 0;
	const cursor = startOfDay(referenceDate);

	while (activityByDate.get(isoDate(cursor))) {
		streak += 1;
		cursor.setDate(cursor.getDate() - 1);
	}

	return streak;
}

export function buildActivityCalendar(
	activityByDate: Map<string, number>,
	totalPlaytimeMinutes: number,
	referenceDate = new Date(),
	dayCount = 365,
): ActivityCalendar {
	const today = toDateAtNoon(referenceDate);
	const firstDate = toDateAtNoon(new Date(today));
	firstDate.setDate(today.getDate() - (dayCount - 1));

	const leadingEmptyDays = firstDate.getDay();
	const trailingEmptyDays = (7 - ((leadingEmptyDays + dayCount) % 7)) % 7;
	const cells: Array<ActivityDay | EmptyActivityDay> = [];

	for (let index = 0; index < leadingEmptyDays; index += 1) {
		cells.push({ kind: "empty", id: `leading-${index}` });
	}

	for (let index = 0; index < dayCount; index += 1) {
		const date = toDateAtNoon(new Date(firstDate));
		date.setDate(firstDate.getDate() + index);
		const hours = formatMinutesAsHours(activityByDate.get(isoDate(date)) || 0);

		cells.push({
			kind: "day",
			date,
			isoDate: isoDate(date),
			hours,
			level: activityLevel(hours),
		});
	}

	for (let index = 0; index < trailingEmptyDays; index += 1) {
		cells.push({ kind: "empty", id: `trailing-${index}` });
	}

	const weekCount = Math.ceil(cells.length / 7);
	const monthHeaders: MonthHeader[] = [];
	let previousMonth = "";

	for (let weekIndex = 0; weekIndex < weekCount; weekIndex += 1) {
		const start = weekIndex * 7;
		const currentWeek = cells.slice(start, start + 7);
		const firstDay = currentWeek.find(
			(cell): cell is ActivityDay => cell.kind === "day",
		);
		const label = firstDay
			? firstDay.date.toLocaleDateString("en-US", { month: "short" })
			: "";

		if (label && label !== previousMonth) {
			previousMonth = label;
			monthHeaders.push({ key: `month-${weekIndex}`, label });
			continue;
		}

		monthHeaders.push({ key: `month-${weekIndex}`, label: "" });
	}

	return {
		cells,
		monthHeaders,
		totalHours: formatMinutesAsHours(totalPlaytimeMinutes),
		longestStreak: calculateLongestStreak(activityByDate),
		currentStreak: calculateCurrentStreak(activityByDate, referenceDate),
	};
}

export function formatActivityDate(date: Date) {
	return date.toLocaleDateString("en-US", {
		month: "short",
		day: "numeric",
		year: "numeric",
	});
}
