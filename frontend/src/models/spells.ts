export function getSummonerFromId(spellId: number): string {
    switch (spellId) {
        case 1:
            return "SummonerBoost";
        case 3:
            return "SummonerExhaust";
        case 4:
            return "SummonerFlash";
        case 6:
            return "SummonerHaste";
        case 7:
            return "SummonerHeal";
        case 11:
            return "SummonerSmite";
        case 12:
            return "SummonerTeleport";
        case 13:
            return "SummonerMana";
        case 14:
            return "SummonerDot";
        case 21:
            return "SummonerBarrier";
        case 30:
            return "SummonerPoroRecall";
        case 31:
            return "SummonerPoroThrow";
        case 32:
            return "SummonerSnowball";
        case 39:
            return "SummonerSnowURFSnowball_Mark";
        default:
            return "Unknown";
    }
}
