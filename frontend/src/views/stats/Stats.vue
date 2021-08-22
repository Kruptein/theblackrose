<script setup lang="ts">
import { onMounted, ref } from "vue";

import { getAuthHeader, backendUrl } from "../../api/utils";
import { getChampionId, getChampionImage, getChampionInfo, getChampionNames } from "../../ddragon";
import { decimalRound } from "../../utils";

const mingames = ref(0);

type Winrate = Record<string, Record<string, Record<number, { wins: number; total: number }>>>;

const winrates = ref<Winrate>({});

const names = getChampionNames();

onMounted(async () => {
    const headers = await getAuthHeader();
    const response = await fetch(backendUrl(`/api/stats/winrates/`), headers);
    winrates.value = JSON.parse(await response.json());
});
</script>

<template>
    <div id="welcome">
        <img src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png" />

        <h1>Winrates</h1>
        <div>Minimum games played filter:</div>
        <input type="number" v-model="mingames" />
        <div style="margin-top: 50px"></div>
        <div v-for="champion in names" class="champion">
            <img :src="`${getChampionImage(getChampionInfo(getChampionId(champion)).id)}`" />
            <div class="data">
                <div class="line"></div>
                <div
                    v-for="(data, summoner) in winrates[getChampionInfo(getChampionId(champion)).id]"
                    class="summoner"
                    :style="{
                        left: `${decimalRound((500 * data[450].wins) / data[450].total)}px`,
                        display: data[450].total < mingames ? 'none' : '',
                    }"
                    :title="`${summoner} [${data[450].wins}/${data[450].total}]`"
                >
                    {{ summoner[0] }}
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.champion {
    display: flex;
    flex-direction: row;
    align-items: center;
    margin-bottom: 25px;

    img {
        width: 50px;
        height: 50px;
        margin-right: 25px;
    }

    .data {
        position: relative;
    }
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
