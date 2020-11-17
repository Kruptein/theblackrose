export interface Match {
    gameId: number;
    gameCreation: number;
    gameDuration: number;
    gameMode: string;
    gameType: string;
    gameVersion: string;
    mapId: number;
    platformId: string;
    queueId: number;
    seasonId: number;
}
export interface Participant {
    championId: string;
    gameId: number;
    highestAchievedSeasonTier?: string;
    spell1Id: number;
    spell2Id: number;
    summonerId: number;
    teamId: number;
}
export interface Summoner {
    name: string;
}
export interface ParticipantStatsGeneral {
    champLevel: number;
    goldEarned: number;
    goldSpent: number;
    item0: number;
    item1: number;
    item2: number;
    item3: number;
    item4: number;
    item5: number;
    item6: number;
    win: boolean;
}
export interface ParticipantStatsKills {
    assists: number;
    deaths: number;
    kills: number;
    totalMinionsKilled: number;
}
