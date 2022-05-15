<script setup lang="ts">
import { onMounted, ref } from "vue";

import { getAuthHeader, backendUrl } from "../../api/utils";
import { getChampionId, getChampionImage, getChampionNames } from "../../ddragon";
import StatBar, { WrData } from "../../components/StatBar.vue";
import { computed } from "@vue/reactivity";

const mingames = ref(0);

type Winrate = Record<number, Record<string, Record<number, { wins: number; total: number }>>>;

const allWinrates = ref<Winrate>({});

const names = getChampionNames();

onMounted(async () => {
    const headers = await getAuthHeader();
    const response = await fetch(backendUrl(`/api/stats/winrates/`), headers);
    allWinrates.value = await response.json();
});

const filteredWinrates = computed(() => {
    if (Object.keys(allWinrates.value).length === 0) return {};

    const wrData: Record<string, WrData> = {};
    for (const [champion, chData] of Object.entries(allWinrates.value)) {
        wrData[champion] = [];
        for (const [summoner, data] of Object.entries(chData)) {
            const summed = Object.entries(data).reduce(
                (acc, curr) => ({ wins: acc.wins + curr[1].wins, total: acc.total + curr[1].total }),
                { total: 0, wins: 0 },
            );
            if (summed.total >= mingames.value) wrData[champion].push([summoner, summed]);
        }
    }
    return wrData;
});
</script>

<template>
    <div id="welcome">
        <img src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png" />

        <h1>Winrates</h1>
        <div>Minimum games played filter:</div>
        <input type="number" v-model="mingames" min="0" />
        <div style="margin-top: 50px"></div>
        <div v-for="champion in names" class="champion">
            <img :src="`${getChampionImage(getChampionId(champion))}`" />
            <StatBar :title="champion" :data="filteredWinrates[getChampionId(champion)] ?? []" />
        </div>
    </div>
</template>

<style lang="scss" scoped>
.champion {
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    margin-bottom: 25px;

    img {
        width: 50px;
        height: 50px;
        margin-right: 25px;
    }

    .data {
        position: relative;
    }

    &:last-child {
        padding-bottom: 100px;
    }
}
</style>
