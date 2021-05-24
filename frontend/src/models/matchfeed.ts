import { Match, Participant, Summoner, ParticipantStatsGeneral, ParticipantStatsKills } from "./match";

export interface MatchFeedFilter {
    names?: string[];
    start?: number;
    length?: number;
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
