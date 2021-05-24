<script lang="ts">
import { defineComponent, PropType, ref } from "vue";

import { fetchMatchFeed, fetchConnections } from "../api/matchfeed";
import MatchList from "../components/MatchList.vue";
import { MatchFeedElement, MatchFeedFilter } from "../models/matchfeed";

export default defineComponent({
    name: "MatchFetcher",
    components: { MatchList },
    props: {
        filter: Object as PropType<MatchFeedFilter>,
        showMore: { type: Boolean, default: true },
    },
    async setup(props) {
        const matches = ref<MatchFeedElement[]>([]);
        const connections = ref<string[]>([]);

        const filter = props.filter ?? {};

        const loadMoreData = async (): Promise<void> => {
            matches.value.push(...(await fetchMatchFeed(filter)));
            filter.before = matches.value[matches.value.length - 1].matchInfo.gameCreation;
        };

        const [matchData, connectionData] = await Promise.all([fetchMatchFeed(filter), fetchConnections()]);

        matches.value = matchData;
        connections.value = connectionData;
        filter.before = matches.value[matches.value.length - 1].matchInfo.gameCreation;

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
