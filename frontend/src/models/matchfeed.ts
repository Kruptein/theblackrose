import type {
    Match,
    ParticipantStatsGeneral,
    ParticipantStatsItems,
    ParticipantStatsKda,
    ParticipantStatsProgress,
    ParticipantStatsSpells,
    Summoner,
} from "./match";

export interface MatchFeedFilter {
    names?: string[];
    after?: number;
    before?: number;
    length?: number;
    queues?: number[];
}

export interface MatchFeedElement {
    matchInfo: Match;
    participants: {
        summoner?: Summoner;
        general: ParticipantStatsGeneral;
        items: ParticipantStatsItems;
        kda: ParticipantStatsKda;
        progress: ParticipantStatsProgress;
        spells: ParticipantStatsSpells;
    }[];
}
