<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";

import { backendUrl, getAuthHeader } from "../../api/utils";
import { getChampionImage, getChampionInfo } from "../../ddragon";
import { decimalRound } from "../../utils";

interface LiveData {
    summoner: string;
    champion: number;
    team: number;
    wins: number;
    total: number;
}

onMounted(async () => await getLiveData());

const route = useRoute();
const summonerName = route.params.summoner;
const error = ref("");

const blueTeam = ref<LiveData[]>([]);
const redTeam = ref<LiveData[]>([]);

async function getLiveData(): Promise<void> {
    error.value = "Loading data";
    const headers = await getAuthHeader();
    const response = await fetch(backendUrl(`/api/live/${summonerName}/`), headers);
    if (response.ok) {
        const data: LiveData[] = await response.json();
        parseLiveData(data);
        error.value = "";
    } else {
        blueTeam.value = [];
        redTeam.value = [];
        error.value = await response.text();
    }
}

function parseLiveData(data: LiveData[]): void {
    for (const info of data) {
        if (info.team === 100) blueTeam.value.push(info);
        else if (info.team === 200) redTeam.value.push(info);
    }
}
</script>

<template>
    <div id="welcome">
        <img src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png" />

        <h1>Live game info for {{ summonerName }}</h1>
        <div id="reload" @click="getLiveData">RELOAD DATA</div>
        <div v-if="error">{{ error }}</div>
        <div v-else>
            <h2>Blue</h2>
            <div class="team">
                <div v-for="info in blueTeam" :key="info.summoner">
                    <img :src="`${getChampionImage(info.champion)}`" />
                    <!-- <div class="champion-name">{{ getChampionInfo(info.champion).name }}</div> -->
                    <div class="stats">
                        <template v-if="info.total > 0">
                            {{ info.wins }} / {{ info.total }} ({{ decimalRound((100 * info.wins) / info.total) }}%)
                        </template>
                        <template v-else>unknown agent</template>
                    </div>
                    <div class="summoner-name">{{ info.summoner }}</div>
                </div>
            </div>
            <div id="divider"></div>
            <div class="blue team">
                <div v-for="info in redTeam" :key="info.summoner">
                    <img :src="`${getChampionImage(info.champion)}`" />
                    <!-- <div class="champion-name">{{ getChampionInfo(info.champion).name }}</div> -->
                    <div class="stats">
                        <template v-if="info.total > 0">
                            {{ info.wins }} / {{ info.total }} ({{ decimalRound((100 * info.wins) / info.total) }}%)
                        </template>
                        <template v-else>unknown agent</template>
                    </div>
                    <div class="summoner-name">{{ info.summoner }}</div>
                </div>
            </div>
            <h2>Red</h2>
        </div>
    </div>
</template>

<style scoped lang="scss">
#reload {
    margin-bottom: 15px;

    &:hover {
        cursor: pointer;
        font-weight: bold;
    }
}

.team {
    display: flex;

    img {
        width: 120px;
    }

    .summoner-name {
        font-weight: bold;
        margin: 10px 0;
    }

    .stats {
        margin: 5px 0;
    }

    > div {
        display: flex;
        flex-direction: column;
        margin-right: 25px;
        align-items: center;
        min-width: 175px;
    }
}

#divider {
    background-color: rebeccapurple;
    border-radius: 15px;
    width: 100%;
    height: 10px;
    margin: 10px 0;
}

.blue {
    > div {
        flex-direction: column-reverse;
    }
}
</style>
