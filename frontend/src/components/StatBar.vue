<script lang="ts" setup>
import { computed, ref } from "@vue/reactivity";

import { decimalRound } from "../utils";

export type WrData = [string, { wins: number; total: number }][];
const props = defineProps<{
    title: string;
    data: WrData;
}>();

const highestValue = ref(0);

const percentages = computed(() => {
    const p = props.data.map((d) => [d[0], decimalRound((100 * d[1].wins) / d[1].total)]) as [string, number][];
    return p.sort((a, b) => a[1] - b[1]);
});

const heights = computed(() => {
    const heightMap: Record<number, number[]> = {};
    const summMap: Record<string, number> = {};
    for (const [i, [summ, perc]] of percentages.value.entries()) {
        if (i === 0) {
            summMap[summ] = 0;
            heightMap[0] = [perc];
        } else {
            for (let j = 0; ; j++) {
                if (heightMap[j] === undefined) {
                    heightMap[j] = [perc];
                    summMap[summ] = j;
                    highestValue.value = j;
                    break;
                }
                if (perc - heightMap[j].at(-1)! >= 2.5) {
                    heightMap[j].push(perc);
                    summMap[summ] = j;
                    break;
                }
            }
        }
    }
    return summMap;
});
</script>

<template>
    <div class="data" :style="{ height: `${50 + 18 + highestValue * 15}px` }">
        <div class="name">{{ title }}</div>
        <div class="line"></div>
        <template v-for="[summoner, wr] in data">
            <div
                class="summoner"
                :style="{
                    left: `${decimalRound((1000 * wr.wins) / wr.total) - 2}px`,
                    top: 0,
                }"
            >
                |
            </div>
            <div
                class="summoner"
                :style="{
                    left: `${decimalRound((1000 * wr.wins) / wr.total) - 8}px`,
                    top: `${18 + 15 * heights[summoner]}px`,
                }"
                :title="`${summoner} [${wr.wins}/${wr.total}]`"
            >
                {{ summoner.slice(0, 2) }}
            </div>
        </template>
    </div>
</template>

<style scoped lang="scss">
.name {
    margin: 5px 0;
    width: 200px;
    text-align: left;
}

.line {
    height: 15px;
    width: 1000px;
    background: rgb(255, 0, 0);
    background: linear-gradient(90deg, rgba(255, 0, 0, 1) 0%, rgba(0, 255, 0, 1) 100%);
}

.summoner {
    position: absolute;
    margin-top: 26px;

    &:hover {
        font-weight: bold;
        z-index: 99;
    }
}
</style>
