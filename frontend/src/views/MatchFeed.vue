<script lang="ts">
import { defineComponent, inject, onMounted, ref } from "vue";
import { AuthPlugin } from "../plugins/auth0";
import { Match, Participant, ParticipantStatsGeneral, ParticipantStatsKills, Summoner } from "../models/match";
import { getQueueFromId } from "../models/queue";
import { getSummonerFromId } from "../models/spells";
import { backendUrl, decimalRound } from "../utils";

interface MatchFeedElement {
    matchInfo: Match;
    participants: {
        participant: Participant;
        summoner: Summoner;
        general: ParticipantStatsGeneral;
        kills: ParticipantStatsKills;
    }[];
}

export default defineComponent({
    name: "MatchFeed",
    setup() {
        const auth = inject<AuthPlugin>("Auth")!;

        const matches = ref<MatchFeedElement[]>([]);
        const connections = ref<string[]>([]);

        const getChampionImage = (participant: Participant): string => {
            return backendUrl(`/ddragon/10.23.1/img/champion/${participant.championId}.png`);
        };

        const getItemImage = (item: number): string => {
            return backendUrl(`/ddragon/10.23.1/img/item/${item}.png`);
        };

        const getSummonerImage = (spell: number): string => {
            return backendUrl(`/ddragon/10.23.1/img/spell/${getSummonerFromId(spell)}.png`);
        };

        const getKda = (stats: ParticipantStatsKills): number => {
            return decimalRound((stats.kills + stats.assists) / Math.max(stats.deaths, 1), 2);
        };

        const getRelativeTime = (timestamp: number): string => {
            const timeDelta = +new Date() - +new Date(timestamp);
            const dataMap: [number, string][] = [
                [365 * 24 * 60 * 60 * 1000, "year"],
                [30 * 24 * 60 * 60 * 1000, "month"],
                [7 * 24 * 60 * 60 * 1000, "week"],
                [24 * 60 * 60 * 1000, "day"],
                [60 * 60 * 1000, "hour"],
                [60 * 1000, "minute"],
                [1000, "second"],
            ];
            for (const [period, name] of dataMap) {
                if (timeDelta >= period) {
                    const times = Math.round(timeDelta / period);
                    return `${times} ${name}${times > 1 ? "s" : ""} ago`;
                }
            }
            return "now";
        };

        onMounted(async () => {
            const token: string = await auth.getTokenSilently();
            const headers = { headers: { Authorization: `Bearer ${token}` } };
            const responses = await Promise.all([
                fetch(backendUrl("/api/matches/"), headers),
                fetch(backendUrl("/api/connections/"), headers),
            ]);
            const connectionData = JSON.parse(await responses[1].json());
            connections.value = connectionData.map((c: [string, number]) => c[0]);
            matches.value = JSON.parse(await responses[0].json());
            console.log(matches.value);
        });

        return {
            connections,
            decimalRound,
            getChampionImage,
            getItemImage,
            getKda,
            getSummonerImage,
            getRelativeTime,
            getQueueFromId,
            matches,
        };
    },
});
</script>

<template>
    <div id="matches">
        <template v-if="matches.length > 0">
            <div id="match" v-for="match of matches" :key="match.matchInfo.gameId">
                <div
                    class="participant"
                    :class="{
                        'game-won': participant.general.win,
                        'is-connection': connections.includes(participant.summoner.name),
                    }"
                    v-for="participant of match.participants"
                    :key="participant.gameId + '-' + participant.participantId"
                >
                    <div
                        :style="{ backgroundImage: `url(${getChampionImage(participant.participant)})` }"
                        class="champion"
                    />
                    <div class="column">
                        {{ getQueueFromId(match.matchInfo.queueId) }}
                        <br />
                        {{ participant.summoner.name }}
                    </div>
                    <div class="column">
                        {{ Math.floor(match.matchInfo.gameDuration / 60) }}
                        minutes
                        <br />
                        {{ getRelativeTime(match.matchInfo.gameCreation + 1000 * match.matchInfo.gameDuration) }}
                    </div>
                    <div class="column font-semi-bold">
                        {{ participant.kills.kills }}/{{ participant.kills.deaths }}/{{ participant.kills.assists }}
                        <br />
                        KDA: {{ getKda(participant.kills) }}
                    </div>
                    <div class="column">
                        {{ decimalRound(participant.general.goldEarned / 1000, 2) }}k gold
                        <br />
                        {{ participant.kills.totalMinionsKilled }} cs
                    </div>
                    <div class="column">
                        <img style="width: 30px; height: 30px" :src="getItemImage(participant.general.item0)" />
                        <img style="width: 30px; height: 30px" :src="getItemImage(participant.general.item1)" />
                        <img style="width: 30px; height: 30px" :src="getItemImage(participant.general.item2)" />
                        <br />
                        <img style="width: 30px; height: 30px" :src="getItemImage(participant.general.item3)" />
                        <img style="width: 30px; height: 30px" :src="getItemImage(participant.general.item4)" />
                        <img style="width: 30px; height: 30px" :src="getItemImage(participant.general.item5)" />
                    </div>
                    <div class="column">
                        <img
                            style="width: 30px; height: 30px"
                            :src="getSummonerImage(participant.participant.spell1Id)"
                        />
                        <br />
                        <img
                            style="width: 30px; height: 30px"
                            :src="getSummonerImage(participant.participant.spell2Id)"
                        />
                    </div>
                </div>
            </div>
        </template>
    </div>
</template>

<style lang="scss" scoped>
#matches {
    margin: 5em;
    display: flex;
    flex-direction: column;
}

#match {
    margin-bottom: 1em;
    border: 2px solid black;
    box-shadow: 2px 2px black;

    .participant {
        background-color: #d49190;
        display: flex;
        flex-direction: row;
        padding: 10px;
        height: 65px;
        cursor: pointer;

        .champion {
            width: 60px;
            background-repeat: no-repeat;
            background-size: contain;
        }

        .column {
            min-width: 150px;
            height: 60px;
            width: auto;
            margin: 0 5px;
            text-align: center;
        }
    }

    .game-won {
        background-color: #5aabbb;
    }

    & > :not(.is-connection) {
        display: none;
    }

    &:hover > :not(.is-connection) {
        display: flex;
    }
}
</style>
