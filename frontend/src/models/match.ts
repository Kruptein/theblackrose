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
    gameId: number;
    summonerId: number;
    championId: number;
    gameEndedInEarlySurrender: boolean;
    gameEndedInSurrender: boolean;
    individualPosition: string;
    teamEarlySurrendered: boolean;
    teamId: number;
    teamPosition: string;
    win: boolean;
}

export interface ParticipantStatsItems {
    gameId: number;
    summonerId: number;

    consumablesPurchased: number;
    detectorWardsPlaced: number;

    item0: number;
    item1: number;
    item2: number;
    item3: number;
    item4: number;
    item5: number;
    item6: number;

    itemsPurches: number;
    sightWardsBoughtInGame: number;
    visionWardsBoughtInGame: number;
}

export interface ParticipantStatsKda {
    gameId: number;
    summonerId: number;
    assists: number;
    deaths: number;
    doubleKills: number;
    dragonKills: number;
    firstBloodAssist: boolean;
    firstBloodKill: boolean;
    firstTowerAssist: boolean;
    firstTowerKill: boolean;
    inhibitorKills: number;
    inhibitorTakedowns: number;
    inhibitorsLost: number;
    killingSprees: number;
    kills: number;
    largestKillingSpree: number;
    largestMultiKill: number;
    neutralMinionsKilled: number;
    nexusKills: number;
    nexusTakedowns: number;
    nexusLost: number;
    objectivesStolen: number;
    objectivesStolenAssists: number;
    pentaKills: number;
    quadraKills: number;
    totalMinionsKilled: number;
    tripleKills: number;
    turretKills: number;
    turretTakedowns: number;
    turretsLost: number;
    unrealKills: number;
    wardsKilled: number;
}

export interface ParticipantStatsProgress {
    gameId: number;
    summonerId: number;
    bountyLevel: number;
    champExperience: number;
    champLevel: number;
    championTransform: number;
    goldEarned: number;
    goldSpent: number;
    longestTimeSpentLiving: number;
    timeCCing_others: number;
    timePlayed: number;
    totalTimeCcDealt: number;
    totalTimeSpentDead: number;
    visionScore: number;
    wardsPlaced: number;
}

export interface ParticipantStatsSpells {
    gameId: number;
    summonerId: number;

    spell1Casts: number;
    spell2Casts: number;
    spell3Casts: number;
    spell4Casts: number;
    summoner1Casts: number;
    summoner1Id: number;
    summoner2Casts: number;
    summoner2Id: number;
}
