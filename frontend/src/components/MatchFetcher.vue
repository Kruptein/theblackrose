<script setup lang="ts">
import { ref, watchEffect } from "vue";

import { fetchConnections } from "../api/matchfeed";
import { fetchWithQuery } from "../api/utils";
import MatchList from "../components/MatchList.vue";
import type { MatchFeedElement, MatchFeedFilter } from "../models/matchfeed";

const props = withDefaults(
    defineProps<{
        names?: string[];
        queues?: number[];
        length?: number;
        showMore?: boolean;
    }>(),
    {
        names: () => [],
        queues: () => [],
        showMore: true,
    },
);
defineEmits(["update:filter"]);

const matches = ref<MatchFeedElement[]>([]);
const connections = ref<string[]>([]);

const filter: Partial<MatchFeedFilter> = {};

let resolveFunc: () => void;

watchEffect(async () => {
    if (props.queues?.length === 0) return;

    const [matchData, connectionData] = await Promise.all([
        fetchWithQuery<MatchFeedElement[]>("/api/matches/", {
            names: props.names,
            queues: props.queues,
            length: props.length,
        }),
        fetchConnections(),
    ]);

    matches.value = matchData;
    connections.value = connectionData;
    filter.before = matches.value[matches.value.length - 1].matchInfo.gameCreation;

    resolveFunc();
});

await new Promise<void>((res) => (resolveFunc = res));

const loadMoreData = async (): Promise<void> => {
    matches.value.push(
        ...(await fetchWithQuery<MatchFeedElement[]>("/api/matches/", {
            names: props.names,
            queues: props.queues,
            length: props.length,
            ...filter,
        })),
    );
    filter.before = matches.value[matches.value.length - 1].matchInfo.gameCreation;
};
</script>

<template>
    <MatchList :match-data="matches" :visible-names="connections" />
    <div id="more-data" @click="loadMoreData" v-if="showMore">Load more data</div>
</template>

<style lang="scss">
#more-data {
    margin-bottom: 50px;

    &:hover {
        cursor: pointer;
    }
}
</style>
