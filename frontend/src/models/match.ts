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
    id: number;
    championId: string;
    gameId: number;
    highestAchievedSeasonTier?: string;
    spell1Id: number;
    spell2Id: number;
    summonerId: number;
    teamId: number;
}
export interface Summoner {
    accountId: string;
    lastMatchQueryTime: string;
    name: string;
    profileIconId: number;
    puuid: string;
    revisionDate: number;
    summonerId: string;
    summonerLevel: number;
    updateInProgress: boolean;
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
