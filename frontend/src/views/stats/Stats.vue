<script setup lang="ts">
import { onMounted, ref } from "vue";

import { getAuthHeader, backendUrl } from "../../api/utils";
import { getChampionId, getChampionInfo, getChampionNames } from "../../ddragon";
import { decimalRound } from "../../utils";

type Winrate = Record<string, Record<string, Record<number, { wins: number; total: number }>>>;

const winrates = ref<Winrate>({});

const names = getChampionNames();

onMounted(async () => {
    const headers = await getAuthHeader();
    const response = await fetch(backendUrl(`/api/stats/winrates/`), headers);
    winrates.value = JSON.parse(await response.json());
});

// function getWinrate(data: Record<number, { wins: number; total: number }>): number {

// }
</script>

<template>
    <div id="welcome">
        <img src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png" />

        <h1>Hello</h1>
        <div v-for="champion in names" class="champion">
            <h2>{{ champion }}</h2>
            <div class="line"></div>
            <div
                v-for="(data, summoner) in winrates[getChampionInfo(getChampionId(champion)).id]"
                class="summoner"
                :style="{ left: `${decimalRound((500 * data[450].wins) / data[450].total)}px` }"
                :title="`${summoner} [${data[450].wins}/${data[450].total}]`"
            >
                {{ summoner[0] }}
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.champion {
    position: relative;
}
.line {
    height: 15px;
    width: 500px;
    background: rgb(255, 0, 0);
    background: linear-gradient(90deg, rgba(255, 0, 0, 1) 0%, rgba(0, 255, 0, 1) 100%);
}

.summoner {
    position: absolute;

    &:hover {
        font-weight: bold;
        z-index: 99;
    }
}
</style>
