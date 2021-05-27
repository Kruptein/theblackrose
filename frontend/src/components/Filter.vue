<script lang="ts">
import { defineComponent, PropType, ref, watchEffect } from "vue";

import { queueMap, queueMode, queuePickType, queuePlayerSize, queueSeriousness } from "../models/queue";

export default defineComponent({
    props: { defaults: { type: Object as PropType<Set<number>>, default: new Set([450]) } },
    emits: ["filter"],
    setup(props, { emit }) {
        const selected = ref<Map<string, Set<number>>>(new Map());
        selected.value.set("map", new Set(bootstrapDefaults(queueMap)));
        selected.value.set("playerSize", new Set(bootstrapDefaults(queuePlayerSize)));
        selected.value.set("pickType", new Set(bootstrapDefaults(queuePickType)));
        selected.value.set("mode", new Set(bootstrapDefaults(queueMode)));
        selected.value.set("seriousness", new Set(bootstrapDefaults(queueSeriousness)));

        let a = false;

        const maps = Object.entries(queueMap);
        const playerSizes = Object.entries(queuePlayerSize);
        const pickTypes = Object.entries(queuePickType);
        const modes = Object.entries(queueMode);
        const seriousnesss = Object.entries(queueSeriousness);

        function bootstrapDefaults(map: Record<string, number[]>): number[] {
            return Object.values(map).reduce(
                (prev, curr) => (curr.some((x) => props.defaults.has(x)) ? [...prev, ...curr] : prev),
                [],
            );
        }

        watchEffect(() => {
            const selectedQueues = [
                ...[...selected.value.values()].reduce((prev, curr) => new Set([...prev].filter((x) => curr.has(x)))),
            ];
            // temporary hack to fix errors
            if (a) emit("filter", selectedQueues);
            a = true;
        });

        function toggleOptions(target: HTMLInputElement, key: string, ids: number[]): void {
            if (!target.checked) {
                for (const id of ids) selected.value.get(key)!.delete(id);
            } else {
                for (const id of ids) selected.value.get(key)!.add(id);
            }
        }

        return { selected, toggleOptions, maps, modes, pickTypes, playerSizes, seriousnesss };
    },
});
</script>

<template>
    <transition name="filter">
        <div id="filter">
            <div class="fs">
                <legend>Map</legend>
                <div class="row" v-for="[map, ids] of maps" :key="'map-' + map">
                    <input
                        type="checkbox"
                        :id="map"
                        name="map"
                        :checked="ids.some((id) => defaults.has(id))"
                        @click="toggleOptions($event.target, 'map', ids)"
                    />
                    <label :for="map">{{ map }}</label>
                </div>
            </div>
            <div class="fs">
                <legend>Mode</legend>
                <div class="row" v-for="[mode, ids] of modes" :key="'mode-' + mode">
                    <input
                        type="checkbox"
                        :id="'mode-' + mode"
                        name="mode"
                        :checked="ids.some((id) => defaults.has(id))"
                        @click="toggleOptions($event.target, 'mode', ids)"
                    />
                    <label :for="'mode-' + mode">{{ mode }}</label>
                </div>
            </div>
            <div class="fs">
                <legend>Pick Type</legend>
                <div class="row" v-for="[pickType, ids] of pickTypes" :key="'pickType-' + pickType">
                    <input
                        type="checkbox"
                        :id="pickType"
                        name="pickType"
                        :checked="ids.some((id) => defaults.has(id))"
                        @click="toggleOptions($event.target, 'pickType', ids)"
                    />
                    <label :for="pickType">{{ pickType }}</label>
                </div>
            </div>
            <div class="fs">
                <legend>Challenge</legend>
                <div class="row" v-for="[seriousness, ids] of seriousnesss" :key="'seriousness-' + seriousness">
                    <input
                        type="checkbox"
                        :id="'challenge-' + seriousness"
                        name="seriousness"
                        :checked="ids.some((id) => defaults.has(id))"
                        @click="toggleOptions($event.target, 'seriousness', ids)"
                    />
                    <label :for="'challenge-' + seriousness">{{ seriousness }}</label>
                </div>
            </div>
            <div class="fs">
                <legend>#Players</legend>
                <div class="row" v-for="[playerSize, ids] of playerSizes" :key="'playerSize-' + playerSize">
                    <input
                        type="checkbox"
                        :id="playerSize"
                        name="playerSize"
                        :checked="ids.some((id) => defaults.has(id))"
                        @click="toggleOptions($event.target, 'playerSize', ids)"
                    />
                    <label :for="playerSize">{{ playerSize }}</label>
                </div>
            </div>
        </div>
    </transition>
</template>

<style scoped lang="scss">
#filter {
    display: flex;
    justify-content: center;
    height: auto;
    transition: all 0.2s;

    flex-wrap: wrap;

    .fs {
        border: solid 1px;
        padding: 1em;
        position: relative;
        display: grid;
        grid-template-rows: repeat(10, 1fr);
        grid-auto-flow: column;
        flex-wrap: wrap;

        min-width: 100px;
        max-height: 15em;

        .row {
            align-self: flex-start;
            width: fit-content;
        }

        legend {
            position: absolute;
            top: -10px;
            left: 0;
            right: 0;
            margin-left: auto;
            margin-right: auto;
            width: 100px;

            background: #bc2131;
        }
    }
}

.filter-enter-to,
.filter-leave-from {
    max-height: 20em;
}

.filter-enter-from,
.filter-leave-to {
    opacity: 0;
    max-height: 0;
}
</style>
