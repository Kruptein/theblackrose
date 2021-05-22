<script lang="ts">
import { computed, defineComponent, onMounted, reactive } from "vue";
import { useRoute } from "vue-router";

import { backendUrl, getAuthHeader } from "../api/utils";
import { getSummonerIconImage } from "../common";
import MatchFetcher from "../components/MatchFetcher.vue";
import { Summoner } from "../models/match";
import { connectionStore } from "../store/connections";
import { decimalRound } from "../utils";

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

// eslint-disable-next-line import/no-unused-modules
export default defineComponent({
    components: { MatchFetcher },
    setup() {
        const route = useRoute();

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
            const response = await fetch(backendUrl(`/api/summoners/${route.params.name}/`), headers);
            const data: { core: Summoner; quickStats?: QuickStats } = JSON.parse(await response.json());
            connectionStore.addConnections(data.core);

            for (const k of Object.keys(stats) as [keyof QuickStats]) {
                stats[k] = data.quickStats?.[k] ?? 0;
            }
        });

        return {
            decimalRound,
            getSummonerIconImage,

            connection,
            stats,
        };
    },
});
</script>

<template>
    <main>
        <div id="connection-name">
            <div id="icon" v-if="connection !== undefined">
                <img :src="getSummonerIconImage(connection.profileIconId)" />
            </div>
            <div id="name">
                <div>Surveillance report for</div>
                <template v-if="connection === undefined">... LOADING ...</template>
                <template v-else>{{ connection.name }} [{{ connection.summonerLevel }}]</template>
            </div>
        </div>

        <div id="stats">
            <div class="header">Quick Stats</div>
            <div></div>
            <div class="stats-label">All time</div>
            <div class="stats-label">This season</div>
            <div class="stats-label"># Played played</div>
            <div>{{ stats.totalPlayed }}</div>
            <div>{{ stats.seasonPlayed }}</div>
            <div class="stats-label"># wins</div>
            <div>{{ stats.totalWin }}</div>
            <div>{{ stats.seasonWin }}</div>
            <div class="stats-label"># losses</div>
            <div>{{ stats.totalPlayed - stats.totalWin }}</div>
            <div>{{ stats.seasonPlayed - stats.seasonWin }}</div>
            <div class="stats-label">Win rate</div>
            <div>{{ decimalRound(100 * (stats.totalWin / stats.totalPlayed), 2) }}%</div>
            <div>{{ decimalRound(100 * (stats.seasonWin / stats.seasonPlayed), 2) }}%</div>
            <div class="stats-label"># KDA</div>
            <div>{{ decimalRound((stats.totalKills + stats.totalAssists) / (stats.totalDeaths || 1), 2) }}</div>
            <div>{{ decimalRound((stats.seasonKills + stats.seasonAssists) / (stats.seasonDeaths || 1), 2) }}</div>
        </div>

        <div id="lastgame" v-if="connection !== undefined">
            <div class="header">Last 3 games</div>
            <Suspense>
                <MatchFetcher :filter="{ names: [$route.params.name], length: 3 }" :showMore="false" />
            </Suspense>
        </div>
    </main>
    <nav aria-label="connection menu"></nav>
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
        "name      name"
        "stats     lastgame";

    #connection-name {
        grid-area: name;
        font-size: 50px;
        font-weight: bold;
        display: flex;
        justify-content: flex-start;
        align-items: center;

        #icon {
            img {
                width: 150px;
            }
            margin-right: 2em;
        }

        #name {
            position: relative;

            div {
                position: absolute;
                top: -20px;
                left: -50px;

                font-weight: normal;
                font-size: 25px;
                font-style: italic;
            }
        }
    }

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
