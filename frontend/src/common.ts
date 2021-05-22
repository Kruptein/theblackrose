import { backendUrl } from "./api/utils";
import { getMostRecentPatch } from "./ddragon";

export const getSummonerIconImage = (iconId: number): string => {
    return backendUrl(`/ddragon/${getMostRecentPatch()}/img/profileicon/${iconId}.png`);
};
