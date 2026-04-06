import type { Game, GameSession } from "$lib/types/Game";

export interface DailyBreakdownEntry {
	game: string;
	duration: number;
}

function getLocalDateKey(value: Date | number) {
	const date = typeof value === "number" ? new Date(value) : value;
	const year = date.getFullYear();
	const month = String(date.getMonth() + 1).padStart(2, "0");
	const day = String(date.getDate()).padStart(2, "0");

	return `${year}-${month}-${day}`;
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

function getDailySessionMinutes(
	session: GameSession,
	targetDateKey: string,
	now: number,
) {
	const effectiveEnd =
		session.endTime && session.endTime > session.startTime
			? session.endTime
			: now;
	const durationMs = normalizeSessionDuration(session, now);

	if (durationMs <= 0 || effectiveEnd <= session.startTime) {
		return 0;
	}

	let cursor = session.startTime;
	let minutes = 0;

	while (cursor < effectiveEnd) {
		const current = new Date(cursor);
		const nextDay = new Date(
			current.getFullYear(),
			current.getMonth(),
			current.getDate() + 1,
		).getTime();
		const segmentEnd = Math.min(effectiveEnd, nextDay);

		if (getLocalDateKey(current) === targetDateKey) {
			minutes += (segmentEnd - cursor) / 60000;
		}

		cursor = segmentEnd;
	}

	return Math.round(minutes);
}

export function getDailyBreakdown(
	date: Date,
	games: Game[],
	referenceDate = new Date(),
): DailyBreakdownEntry[] {
	const targetDateKey = getLocalDateKey(date);
	const now = referenceDate.getTime();

	return games
		.filter((game) => game.inLibrary !== false)
		.map((game) => {
			const duration = (game.storageSessions ?? []).reduce(
				(total, session) =>
					total + getDailySessionMinutes(session, targetDateKey, now),
				0,
			);

			return {
				game: game.title,
				duration,
			};
		})
		.filter((entry) => entry.duration > 0)
		.sort((left, right) => right.duration - left.duration);
}
