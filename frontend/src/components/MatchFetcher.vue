<script lang="ts">
import { defineComponent, PropType, ref, watchEffect } from "vue";

import { fetchConnections } from "../api/matchfeed";
import { fetchWithQuery } from "../api/utils";
import MatchList from "../components/MatchList.vue";
import { MatchFeedElement, MatchFeedFilter } from "../models/matchfeed";

export default defineComponent({
    name: "MatchFetcher",
    components: { MatchList },
    props: {
        names: { type: Array as PropType<string[]>, default: () => [] },
        queues: { type: Array as PropType<number[]>, default: () => [] },
        length: { type: Number },
        showMore: { type: Boolean, default: true },
    },
    emits: ["update:filter"],
    async setup(props) {
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

        return {
            connections,
            loadMoreData,
            matches,
        };
    },
});
</script>

<template>
    <MatchList :match-data="matches" :visible-names="connections" />
    <div id="more-data" @click="loadMoreData" v-if="showMore">Load more data</div>
</template>
