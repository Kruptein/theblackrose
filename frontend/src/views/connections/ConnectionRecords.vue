<script setup lang="ts">
import { computed, ref, watchEffect } from "vue";
import { useRoute } from "vue-router";

import { fetchWithQuery } from "../../api/utils";
import ConnectionHeader from "../../components/ConnectionHeader.vue";
import DefaultHeader from "../../components/DefaultHeader.vue";
import Filter from "../../components/Filter.vue";
import Records from "../../components/Records.vue";
import type { MatchFeedElement } from "../../models/matchfeed";
import { queueMode, queueSeriousness } from "../../models/queue";

type Record = { id: number; recordType: number; value: number; name: string; queueId: number; gameId: number };

const route = useRoute();

const loading = ref(true);
const records = ref<Record[]>([]);
const matches = ref<MatchFeedElement[]>([]);

const showFilter = ref(false);

const isConnectionRecords = computed(() => route.name === "ConnectionRecords");

const defaultSelectedQueues = new Set([
    ...queueMode.Normal,
    ...queueMode["Ranked Solo"],
    ...queueMode["Ranked Flex"],
    ...queueMode["Ranked Team"],
    ...queueMode.Clash,
    ...queueMode.ARAM,
]);
for (const queue of queueSeriousness.Bots) defaultSelectedQueues.delete(queue);

const queueFilter = ref<number[]>([...defaultSelectedQueues]);

watchEffect(async () => {
    const [recordsData, matchesData] = await fetchWithQuery<[Record[], MatchFeedElement[]]>("/api/records/", {
        names: isConnectionRecords.value ? [route.params.name as string] : undefined,
        queues: queueFilter.value,
    });
    records.value = recordsData;
    matches.value = matchesData;
    loading.value = false;
});
</script>

<template>
    <main>
        <ConnectionHeader v-if="isConnectionRecords" active="records" />
        <DefaultHeader v-else />

        <div id="records">
            <template v-if="loading">
                <h1>Waiting for server data</h1>
            </template>
            <template v-else>
                <div id="records-title">
                    <h1>Records</h1>
                    <div id="records-filter-toggle" @click="showFilter = !showFilter">
                        [
                        <font-awesome-icon icon="filter" />
                        ]
                    </div>
                </div>
                <Filter v-show="showFilter" :defaults="defaultSelectedQueues" @filter="queueFilter = $event" />
                <Records :matches="matches" :records="records" />
            </template>
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
        "records";
    grid-template-rows: 250px 1fr;

    #records {
        grid-area: records;

        #records-title {
            display: flex;
            justify-content: center;
            align-items: center;

            #records-filter-toggle {
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
