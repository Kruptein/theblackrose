<script lang="ts">
import { defineComponent, onMounted, ref } from "vue";

import { backendUrl, getAuthHeader } from "../api/utils";
import Records from "../components/Records.vue";
import { MatchFeedElement } from "../models/matchfeed";
import { getQueueFromId } from "../models/queue";
import { Record, RecordType } from "../models/records";

// eslint-disable-next-line import/no-unused-modules
export default defineComponent({
    components: { Records },
    setup() {
        const loading = ref(true);
        const records = ref<Record[]>([]);
        const matches = ref<MatchFeedElement[]>([]);

        onMounted(async () => {
            const headers = await getAuthHeader();
            const response = await fetch(backendUrl("/api/records/"), headers);
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
    <div id="welcome">
        <img
            class="logo img-responsive"
            src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png"
        />

        <template v-if="loading">
            <h1>Waiting for server data</h1>
        </template>
        <template v-else>
            <Records :matches="matches" :records="records" />
        </template>
    </div>
</template>
