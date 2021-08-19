import { backendUrl } from "./api/utils";
import type { ChampionInfo } from "./models/champion";
import { getSummonerFromId } from "./models/spells";
import { patches } from "./state";
import { compareVersions } from "./utils";

let championDataLoaded = false;
let championData: Record<number, ChampionInfo> = {};

export async function loadChampionData(): Promise<void> {
    if (!championDataLoaded) {
        const response = await fetch(ddragonUrl("/data/en_US/champion.json"));
        const json = (await response.json()) as { data: Record<string, ChampionInfo> };
        championData = Object.fromEntries(Object.values(json.data).map((v) => [Number.parseInt(v.key), v]));
        championDataLoaded = true;
    }
}

export function getChampionInfo(championId: number): ChampionInfo {
    if (!championDataLoaded) {
        throw new Error("Champion info was not loaded.");
    }
    console.log(championData);
    return championData[championId];
}

export function getMostRecentPatch(gameVersion?: string): string {
    gameVersion = gameVersion?.split(".").slice(0, 2).join(".") ?? "999";
    const lastPatch = patches.value[patches.value.length - 1];
    let previousPatch: string | undefined;
    for (const patch of patches.value) {
        const cmp = compareVersions(gameVersion, patch);
        if (cmp < 0) return patch;
        else if (cmp === 0) return patch;
        previousPatch = patch;
    }
    return previousPatch ?? lastPatch;
}

function ddragonUrl(url: string, gameVersion?: string): string {
    const patch = getMostRecentPatch(gameVersion);
    return backendUrl(`/ddragon/${patch}${url}`);
}

export function getChampionImage(championId: string, gameVersion?: string): string {
    return ddragonUrl(`/img/champion/${championId}.png`, gameVersion);
}

export function getItemImage(item: number, gameVersion?: string): string {
    return ddragonUrl(`/img/item/${item}.png`, gameVersion);
}

export function getSummonerImage(spell: number, gameVersion?: string): string {
    return ddragonUrl(`/img/spell/${getSummonerFromId(spell)}.png`, gameVersion);
}
