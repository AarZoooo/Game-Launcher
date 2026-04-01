import { pageLabels } from "$lib/data/labels";
import { pickGameExecutable, scanLocalGames } from "$lib/services/tauriService";
import { games } from "$lib/stores/libraryStore";
import type { ImportedGameResult } from "$lib/types/Game";
import type { ScanPlatform } from "$lib/types/UI";

function simulateDelay() {
	return new Promise((resolve) => setTimeout(resolve, 900));
}

async function mockSteamResults(): Promise<ImportedGameResult[]> {
	await simulateDelay();

	return [
		{
			id: "steam-portal-2",
			title: "Portal 2",
			path: "D:\\SteamLibrary\\steamapps\\common\\Portal 2\\portal2.exe",
			platform: "steam",
		},
		{
			id: "steam-dark-souls-3",
			title: "Dark Souls III",
			path: "E:\\SteamLibrary\\steamapps\\common\\DARK SOULS III\\Game\\DarkSoulsIII.exe",
			platform: "steam",
		},
		{
			id: "steam-hades",
			title: "Hades",
			path: "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Hades\\Hades.exe",
			platform: "steam",
		},
	];
}

async function mockEpicResults(): Promise<ImportedGameResult[]> {
	await simulateDelay();

	return [
		{
			id: "epic-alan-wake-2",
			title: "Alan Wake 2",
			path: "D:\\Epic Games\\AlanWake2\\AlanWake2.exe",
			platform: "epic",
		},
		{
			id: "epic-fortnite",
			title: "Fortnite",
			path: "E:\\Epic Games\\Fortnite\\FortniteGame\\Binaries\\Win64\\FortniteClient-Win64-Shipping.exe",
			platform: "epic",
		},
		{
			id: "epic-satisfactory",
			title: "Satisfactory",
			path: "C:\\Epic Games\\SatisfactoryEarlyAccess\\FactoryGame.exe",
			platform: "epic",
		},
	];
}

export async function scanImportCandidates(
	platform: ScanPlatform,
): Promise<ImportedGameResult[]> {
	if (platform === "steam") return mockSteamResults();
	if (platform === "epic") return mockEpicResults();
	return scanLocalGames();
}

export async function pickManualImportCandidate() {
	return pickGameExecutable();
}

export async function addImportedGamesToLibrary(imports: ImportedGameResult[]) {
	await games.addImportedGames(imports);
	return pageLabels.messages.addedLocalGames(imports.length);
}

export function buildImportAddedMessage(title: string) {
	return pageLabels.messages.addedToLibrary(title);
}
