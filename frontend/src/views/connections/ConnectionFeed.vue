<script setup lang="ts">
import { computed, ref } from "vue";
import { useRoute } from "vue-router";

// import { backendUrl, getAuthHeader } from "../../api/utils";
import ConnectionHeader from "../../components/ConnectionHeader.vue";
import DefaultHeader from "../../components/DefaultHeader.vue";
import Filter from "../../components/Filter.vue";
import MatchFetcher from "../../components/MatchFetcher.vue";
import { friendlyQueueNames } from "../../models/queue";

const route = useRoute();

const isConnectionFeed = computed(() => route.name === "ConnectionFeed");

const names = computed(() => (isConnectionFeed.value ? [route.params.name as string] : []));

const showFilter = ref(false);

const queueDefaults = new Set(Object.keys(friendlyQueueNames).map((k) => Number.parseInt(k)));
const queueFilter = ref<number[]>([...queueDefaults]);

// async function refresh(): Promise<void> {
//     const headers = await getAuthHeader();
//     await fetch(backendUrl(`/api/connection/${route.params.name}/refresh`), headers);
// }

function setFilter(data: number[]): void {
    queueFilter.value = [...data];
}
</script>

<template>
    <main>
        <ConnectionHeader v-if="isConnectionFeed" active="matches" />
        <DefaultHeader v-else />
        <div id="matches">
            <Suspense>
                <template #default>
                    <div :key="1">
                        <div id="matches-title">
                            <h1>Recent observations</h1>
                            <div id="matches-filter-toggle" @click="showFilter = !showFilter">
                                [
                                <font-awesome-icon icon="filter" />
                                ]
                            </div>
                        </div>
                        <Filter v-show="showFilter" :defaults="queueDefaults" @filter="setFilter" />
                        <MatchFetcher :names="names" :queues="queueFilter" />
                    </div>
                </template>
                <template #fallback>
                    <h1>Waiting for server data</h1>
                </template>
            </Suspense>
        </div>
    </main>
</template>

<style lang="scss" scoped>
main {
    margin-top: 200px;
    margin-left: 200px;

    column-gap: 5em;

    display: grid;
    grid-template-areas:
        "header"
        "matches";
    grid-template-rows: auto 1fr;

    #matches {
        grid-area: matches;

        #matches-title {
            display: flex;
            justify-content: center;
            align-items: center;

            #matches-filter-toggle {
                margin-left: 1em;

                &:hover {
                    cursor: pointer;
                    font-style: italic;
                }
            }
        }
    }
}
</style>
