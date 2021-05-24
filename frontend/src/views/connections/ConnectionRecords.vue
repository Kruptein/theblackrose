<script lang="ts">
import { defineComponent, onMounted, ref } from "vue";
import { useRoute } from "vue-router";

import { backendUrl, getAuthHeader } from "../../api/utils";
import Records from "../../components/Records.vue";
import { MatchFeedElement } from "../../models/matchfeed";
import { getQueueFromId } from "../../models/queue";
import { RecordType } from "../../models/records";

import ConnectionHeader from "./ConnectionHeader.vue";

type Record = { id: number; recordType: number; value: number; name: string; queueId: number; gameId: number };

// eslint-disable-next-line import/no-unused-modules
export default defineComponent({
    components: { ConnectionHeader, Records },
    setup() {
        const route = useRoute();

        const loading = ref(true);
        const records = ref<Record[]>([]);
        const matches = ref<MatchFeedElement[]>([]);

        onMounted(async () => {
            const headers = await getAuthHeader();
            const response = await fetch(backendUrl(`/api/records/?names=${route.params.name}`), headers);
            const [recordsData, matchesData] = JSON.parse(await response.json());
            records.value = recordsData;
            matches.value = matchesData;
            loading.value = false;
        });

        return { getQueueFromId, loading, matches, records, RecordType };
    },
});
</script>

<template>
    <main>
        <ConnectionHeader active="records" />

        <div id="records">
            <template v-if="loading">
                <h1>Waiting for server data</h1>
            </template>
            <template v-else>
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
    }
}
</style>
