import { getSummonerFromId } from "./models/spells";
import { patches } from "./state";
import { backendUrl, compareVersions } from "./utils";

function getMostRecentPatch(gameVersion: string): string {
    gameVersion = gameVersion
        .split(".")
        .slice(0, 2)
        .join(".");
    const lastPatch = patches.value[patches.value.length - 1];
    let previousPatch: string | undefined;
    for (const patch of patches.value) {
        const cmp = compareVersions(gameVersion, patch);
        if (cmp > 0) return previousPatch ?? lastPatch;
        else if (cmp === 0) return patch;
        previousPatch = patch;
    }
    return previousPatch ?? lastPatch;
}

function ddragonUrl(url: string, gameVersion: string): string {
    const patch = getMostRecentPatch(gameVersion);
    return backendUrl(`/ddragon/${patch}${url}`);
}

export function getChampionImage(championId: string, gameVersion: string): string {
    return ddragonUrl(`/img/champion/${championId}.png`, gameVersion);
}

export function getItemImage(item: number, gameVersion: string): string {
    return ddragonUrl(`/img/item/${item}.png`, gameVersion);
}

export function getSummonerImage(spell: number, gameVersion: string): string {
    return ddragonUrl(`/img/spell/${getSummonerFromId(spell)}.png`, gameVersion);
}
