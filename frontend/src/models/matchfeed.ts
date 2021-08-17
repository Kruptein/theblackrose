import type { Match, Participant, Summoner, ParticipantStatsGeneral, ParticipantStatsKills } from "./match";

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
        participant: Participant;
        summoner?: Summoner;
        general: ParticipantStatsGeneral;
        kills: ParticipantStatsKills;
    }[];
}
