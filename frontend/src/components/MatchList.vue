<script lang="ts">
import { defineComponent, PropType, ref } from "vue";

import { fetchMatchFeed, fetchConnections } from "../api/matchfeed";
import { getChampionImage, getItemImage, getSummonerImage } from "../ddragon";
import { Participant, ParticipantStatsKills } from "../models/match";
import { MatchFeedElement, MatchFeedFilter } from "../models/matchfeed";
import { getQueueFromId } from "../models/queue";
import { getSummonerFromId } from "../models/spells";
import { patches } from "../state";
import { backendUrl, decimalRound } from "../utils";

export default defineComponent({
    name: "MatchList",
    props: {
        filter: Object as PropType<MatchFeedFilter>,
    },
    async setup(props) {
        const matches = ref<MatchFeedElement[]>([]);
        const connections = ref<string[]>([]);

        const filter = props.filter ?? {};

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

        const toggleMatch = (event: { target: HTMLDivElement }): void => {
            if (event.target.classList.contains("expand-match")) {
                event.target.classList.remove("expand-match");
            } else {
                event.target.classList.add("expand-match");
            }
        };

        const loadMoreData = async (): Promise<void> => {
            matches.value.push(...(await fetchMatchFeed(filter)));
            filter.start = matches.value[matches.value.length - 1].matchInfo.gameCreation;
        };

        const [matchData, connectionData] = await Promise.all([fetchMatchFeed(filter), fetchConnections()]);

        matches.value = matchData;
        connections.value = connectionData;
        filter.start = matches.value[matches.value.length - 1].matchInfo.gameCreation;

        return {
            connections,
            decimalRound,
            getChampionImage,
            getItemImage,
            getKda,
            getSummonerImage,
            getRelativeTime,
            getQueueFromId,
            loadMoreData,
            matches,
            toggleMatch,
        };
    },
});
</script>

<template>
    <div id="matches">
        <template v-if="matches.length > 0">
            <div id="match" v-for="match of matches" :key="match.matchInfo.gameId" @click="toggleMatch">
                <div
                    class="participant"
                    :class="{
                        'game-won': participant.general.win,
                        'is-connection': connections.includes(participant.summoner.name),
                    }"
                    v-for="participant of match.participants"
                    :key="participant.participant.gameId + '-' + participant.participant.id"
                >
                    <div
                        :style="{
                            backgroundImage: `url(${getChampionImage(
                                participant.participant.championId,
                                match.matchInfo.gameVersion,
                            )})`,
                        }"
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
                    <div class="column">
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
                        <img
                            style="width: 30px; height: 30px"
                            :src="getItemImage(participant.general.item0, match.matchInfo.gameVersion)"
                        />
                        <img
                            style="width: 30px; height: 30px"
                            :src="getItemImage(participant.general.item1, match.matchInfo.gameVersion)"
                        />
                        <img
                            style="width: 30px; height: 30px"
                            :src="getItemImage(participant.general.item2, match.matchInfo.gameVersion)"
                        />
                        <br />
                        <img
                            style="width: 30px; height: 30px"
                            :src="getItemImage(participant.general.item3, match.matchInfo.gameVersion)"
                        />
                        <img
                            style="width: 30px; height: 30px"
                            :src="getItemImage(participant.general.item4, match.matchInfo.gameVersion)"
                        />
                        <img
                            style="width: 30px; height: 30px"
                            :src="getItemImage(participant.general.item5, match.matchInfo.gameVersion)"
                        />
                    </div>
                    <div class="column">
                        <img
                            style="width: 30px; height: 30px"
                            :src="getSummonerImage(participant.participant.spell1Id, match.matchInfo.gameVersion)"
                        />
                        <br />
                        <img
                            style="width: 30px; height: 30px"
                            :src="getSummonerImage(participant.participant.spell2Id, match.matchInfo.gameVersion)"
                        />
                    </div>
                </div>
            </div>
            <div id="more-data" @click="loadMoreData">Load more data</div>
        </template>
    </div>
</template>

<style lang="scss" scoped>
#matches {
    margin: 5em;
    display: flex;
    flex-direction: column;

    #more-data {
        margin: 15px;
        margin-bottom: 50px;
    }
}

#match {
    margin-bottom: 1em;
    border: 2px solid black;
    box-shadow: 2px 2px black;
    cursor: pointer;

    .participant {
        pointer-events: none;
        background-color: #d49190;
        display: flex;
        flex-direction: row;
        padding: 10px;
        height: 65px;

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

    &:not(.expand-match) {
        > :not(.is-connection) {
            display: none;
        }
    }
}
</style>
