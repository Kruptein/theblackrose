<script setup lang="ts">
import { computed, onMounted, reactive } from "vue";
import { useRoute } from "vue-router";

import { backendUrl, getAuthHeader } from "../../api/utils";
import ConnectionHeader from "../../components/ConnectionHeader.vue";
import MatchFetcher from "../../components/MatchFetcher.vue";
import type { Summoner } from "../../models/match";
import { friendlyQueueNames } from "../../models/queue";
import { connectionStore } from "../../store/connections";
import { decimalRound } from "../../utils";

interface QuickStats {
    totalPlayed: number;
    seasonPlayed: number;
    totalWin: number;
    seasonWin: number;
    totalKills: number;
    seasonKills: number;
    totalDeaths: number;
    seasonDeaths: number;
    totalAssists: number;
    seasonAssists: number;
}

const route = useRoute();
const routeName = route.params.name as string;

const queueDefaults = new Set(Object.keys(friendlyQueueNames).map((k) => Number.parseInt(k)));

const stats = reactive({
    totalPlayed: 0,
    seasonPlayed: 0,
    totalWin: 0,
    seasonWin: 0,
    totalKills: 0,
    seasonKills: 0,
    totalDeaths: 0,
    seasonDeaths: 0,
    totalAssists: 0,
    seasonAssists: 0,
});

const connection = computed(() => connectionStore.getConnection(route.params.name as string));

onMounted(async () => {
    for (const k of Object.keys(stats)) {
        stats[k as keyof QuickStats] = 0;
    }

    const headers = await getAuthHeader();
    const response = await fetch(backendUrl(`/api/summoners/${route.params.name}/?stats=true`), headers);
    const data: { core: Summoner; quickStats?: QuickStats } = await response.json();
    connectionStore.addConnections(data.core);

    for (const k of Object.keys(stats) as [keyof QuickStats]) {
        stats[k] = data.quickStats?.[k] ?? 0;
    }
});
</script>

<template>
    <main>
        <ConnectionHeader active="overview" :apiLoad="false" />

        <div id="stats">
            <div class="header">Quick Stats</div>
            <div></div>
            <div class="stats-label">All time</div>
            <div class="stats-label">This season</div>
            <div class="stats-label"># Games played</div>
            <div>{{ stats.totalPlayed }}</div>
            <div>{{ stats.seasonPlayed }}</div>
            <div class="stats-label"># wins</div>
            <div>{{ stats.totalWin }}</div>
            <div>{{ stats.seasonWin }}</div>
            <div class="stats-label"># losses</div>
            <div>{{ stats.totalPlayed - stats.totalWin }}</div>
            <div>{{ stats.seasonPlayed - stats.seasonWin }}</div>
            <div class="stats-label">Win rate</div>
            <div>{{ decimalRound(100 * (stats.totalWin / stats.totalPlayed)) }}%</div>
            <div>{{ decimalRound(100 * (stats.seasonWin / stats.seasonPlayed)) }}%</div>
            <div class="stats-label"># KDA</div>
            <div>{{ decimalRound((stats.totalKills + stats.totalAssists) / (stats.totalDeaths || 1)) }}</div>
            <div>{{ decimalRound((stats.seasonKills + stats.seasonAssists) / (stats.seasonDeaths || 1)) }}</div>
        </div>

        <div id="lastgame" v-if="connection !== undefined">
            <div class="header">Last 3 games</div>
            <Suspense>
                <MatchFetcher :names="[routeName]" :length="3" :queues="[...queueDefaults]" :showMore="false" />
            </Suspense>
        </div>
    </main>
</template>

<style scoped lang="scss">
.header {
    font-size: 35px;
}

main {
    margin-top: 200px;
    margin-left: 200px;

    column-gap: 5em;

    display: grid;
    grid-template-areas:
        "header header"
        "stats  lastgame";
    grid-template-rows: 250px 1fr;

    #stats {
        grid-area: stats;
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        grid-template-rows: repeat(6, 60px);

        font-size: 20px;

        .header {
            grid-column: 1 / -1;
        }

        .stats-label {
            font-weight: bold;
        }
    }
}
</style>

<style lang="scss">
#lastgame {
    display: flex;
    flex-direction: column;
    align-items: flex-start;

    .matches {
        margin: 0;
    }
}
</style>
