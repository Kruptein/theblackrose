<script lang="ts">
import { defineComponent, PropType, ref } from "vue";

import MatchList from "../components/MatchList.vue";

import { fetchMatchFeed, fetchConnections } from "../api/matchfeed";
import { MatchFeedElement, MatchFeedFilter } from "../models/matchfeed";

export default defineComponent({
    name: "MatchFetcher",
    components: { MatchList },
    props: {
        filter: Object as PropType<MatchFeedFilter>,
    },
    async setup(props) {
        const matches = ref<MatchFeedElement[]>([]);
        const connections = ref<string[]>([]);

        const filter = props.filter ?? {};

        const loadMoreData = async (): Promise<void> => {
            matches.value.push(...(await fetchMatchFeed(filter)));
            filter.start = matches.value[matches.value.length - 1].matchInfo.gameCreation;
        };

        const [matchData, connectionData] = await Promise.all([fetchMatchFeed(filter), fetchConnections()]);

        matches.value = matchData;
        connections.value = connectionData;
        filter.start = matches.value[matches.value.length - 1].matchInfo.gameCreation;

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
    <div id="more-data" @click="loadMoreData">Load more data</div>
</template>

<style lang="scss" scoped>
.matches {
    margin: 5em;
    display: flex;
    flex-direction: column;

    #more-data {
        margin: 15px;
        margin-bottom: 50px;
    }
}

.match {
    margin-bottom: 1em;
    border: 2px solid black;
    box-shadow: 2px 2px black;
    cursor: pointer;

    .participant {
        pointer-events: none;
        background-color: #d49190;
        display: flex;
        flex-direction: row;
        padding: 10px;
        height: 65px;

        .champion {
            width: 60px;
            background-repeat: no-repeat;
            background-size: contain;
        }

        .column {
            min-width: 150px;
            height: 60px;
            width: auto;
            margin: 0 5px;
            text-align: center;
        }
    }

    .game-won {
        background-color: #5aabbb;
    }

    &:not(.expand-match) {
        > :not(.is-connection) {
            display: none;
        }
    }
}
</style>
