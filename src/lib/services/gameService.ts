import type { ImportedGameResult } from "$lib/stores/libraryStore";

function simulateDelay() {
	return new Promise((resolve) => setTimeout(resolve, 900));
}

// TODO: Replace these mock scans with backend commands once library scanning is implemented.
export async function scanSteamGames(): Promise<ImportedGameResult[]> {
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

export async function scanEpicGames(): Promise<ImportedGameResult[]> {
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
