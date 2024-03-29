<script setup lang="ts">
import { ref, watch } from "vue";

import { getChampionImage, getItemImage, getSummonerImage } from "../ddragon";
import { ParticipantStatsKda } from "../models/match";
import type { MatchFeedElement } from "../models/matchfeed";
import { getQueueFromId } from "../models/queue";
import { decimalRound } from "../utils";

const props = defineProps<{
    matchData?: MatchFeedElement[];
    visibleNames?: string[];
}>();

const matches = ref<MatchFeedElement[]>(props.matchData ?? []);
const connections = ref<string[]>(props.visibleNames ?? []);

watch(
    () => props.matchData,
    (matchData) => {
        matches.value = matchData ?? [];
    },
);

const getKda = (stats: ParticipantStatsKda): number => {
    return decimalRound((stats.kills + stats.assists) / Math.max(stats.deaths, 1));
};

const getAbsoluteTime = (timestamp: number): string => {
    const date = new Date(timestamp);
    return `${date.getDate()}/${date.getMonth() + 1}/${date.getFullYear()}`;
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

const toggleMatch = (event: Event): void => {
    const target = event.target as HTMLDivElement;
    if (target.classList.contains("expand-match")) {
        target.classList.remove("expand-match");
    } else {
        target.classList.add("expand-match");
    }
};
</script>

<template>
    <div class="matches">
        <template v-if="matches.length > 0">
            <div class="match" v-for="match of matches" :key="match.matchInfo.gameId" @click="toggleMatch">
                <div class="match-common">
                    <div>{{ getQueueFromId(match.matchInfo.queueId) }}</div>
                    <div>{{ Math.floor(match.matchInfo.gameDuration / 60) }} minutes</div>
                    <div class="relative">
                        {{ getRelativeTime(match.matchInfo.gameCreation + 1000 * match.matchInfo.gameDuration) }}
                    </div>
                    <div class="absolute">
                        {{ getAbsoluteTime(match.matchInfo.gameCreation + 1000 * match.matchInfo.gameDuration) }}
                    </div>
                </div>
                <div class="match-participants">
                    <div
                        class="participant"
                        :class="{
                            'game-won': participant.general.win,
                            'is-connection':
                                connections.length === 0 ||
                                connections.includes(
                                    participant.summoner ? participant.summoner.name : '<Unknown Summoner>',
                                ),
                        }"
                        v-for="participant of match.participants"
                        :key="participant.general.gameId + '-' + participant.general.summonerId"
                    >
                        <div
                            :style="{
                                backgroundImage: `url(${getChampionImage(
                                    participant.general.championId,
                                    match.matchInfo.gameVersion,
                                )})`,
                            }"
                            class="champion"
                        />
                        <div class="column">
                            {{ participant.summoner?.name ?? "Unknown Summoner" }}
                        </div>
                        <div class="column">
                            {{ participant.kda.kills }}/{{ participant.kda.deaths }}/{{ participant.kda.assists }}
                            <br />
                            KDA: {{ getKda(participant.kda) }}
                        </div>
                        <div class="column">
                            {{ decimalRound(participant.progress.goldEarned / 1000) }}k gold
                            <br />
                            {{ participant.kda.totalMinionsKilled }} cs
                        </div>
                        <div class="column items">
                            <img
                                style="width: 30px; height: 30px"
                                :src="getItemImage(participant.items.item0, match.matchInfo.gameVersion)"
                            />
                            <img
                                style="width: 30px; height: 30px"
                                :src="getItemImage(participant.items.item1, match.matchInfo.gameVersion)"
                            />
                            <img
                                style="width: 30px; height: 30px"
                                :src="getItemImage(participant.items.item2, match.matchInfo.gameVersion)"
                            />
                            <br />
                            <img
                                style="width: 30px; height: 30px"
                                :src="getItemImage(participant.items.item3, match.matchInfo.gameVersion)"
                            />
                            <img
                                style="width: 30px; height: 30px"
                                :src="getItemImage(participant.items.item4, match.matchInfo.gameVersion)"
                            />
                            <img
                                style="width: 30px; height: 30px"
                                :src="getItemImage(participant.items.item5, match.matchInfo.gameVersion)"
                            />
                        </div>
                        <div class="column summoners">
                            <img
                                style="width: 30px; height: 30px"
                                :src="getSummonerImage(participant.spells.summoner1Id, match.matchInfo.gameVersion)"
                            />
                            <br />
                            <img
                                style="width: 30px; height: 30px"
                                :src="getSummonerImage(participant.spells.summoner2Id, match.matchInfo.gameVersion)"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </template>
    </div>
</template>

<style lang="scss" scoped>
.matches {
    margin: 5em;
    display: flex;
    flex-direction: column;
}

.match {
    display: flex;
    flex-direction: column;
    align-items: flex-end;

    .match-common {
        display: flex;
        color: #d22537;
        background-color: white;
        padding: 0.3em 1em;

        border: solid 2px black;
        border-bottom: 0;
        box-shadow: 2px 2px black;
        border-top-left-radius: 0.5em;

        div:not(:first-child) {
            margin-left: 2em;
        }

        .absolute {
            display: none;
        }
        .relative {
            display: block;
        }

        &:hover {
            .absolute {
                display: block;
            }
            .relative {
                display: none;
            }
        }
    }

    .match-participants {
        margin-bottom: 1em;
        border: 2px solid black;
        box-shadow: 2px 2px black;
        cursor: pointer;

        &:not(.expand-match) {
            > :not(.is-connection) {
                display: none;
            }
        }

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
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;

                min-width: 150px;
                height: 60px;
                width: auto;
                margin: 0 5px;
                text-align: center;

                &.items,
                &.summoners {
                    display: block;
                }
            }
        }

        .game-won {
            background-color: #5aabbb;
        }
    }
}
</style>
