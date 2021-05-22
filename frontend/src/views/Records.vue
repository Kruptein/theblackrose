<script lang="ts">
import { defineComponent, onMounted, ref } from "vue";

import { backendUrl, getAuthHeader } from "../api/utils";
import MatchList from "../components/MatchList.vue";
import { MatchFeedElement } from "../models/matchfeed";
import { getQueueFromId } from "../models/queue";
import { RecordType } from "../models/records";

type Record = { id: number; recordType: number; value: number; name: string; queueId: number; gameId: number };

// eslint-disable-next-line import/no-unused-modules
export default defineComponent({
    name: "Records",
    components: { MatchList },
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

        <!-- <template v-if="loading">
            <h1>Waiting for server data</h1>
        </template>
        <template v-else>
            <div v-for="record in records" class="record-title" :key="record.recordType">
                <span class="font-semi-bold">{{ RecordType[record.recordType] }}</span>
                in {{ getQueueFromId(record.queueId) }} by {{ record.name }}:
                <span class="font-semi-bold">{{ record.value }}</span>
            </div>
        </template> -->
        <Suspense>
            <template #default>
                <div style="display: contents">
                    <h1>Records</h1>
                    <div v-for="(record, i) in records" class="record-title" :key="record.id">
                        <span class="font-semi-bold">{{ RecordType[record.recordType] }}</span>
                        in {{ getQueueFromId(record.queueId) }} by {{ record.name }}
                        :
                        <span class="font-semi-bold">{{ record.value }}</span>
                        <span class="gameId" style="font-size: xx-small">[{{ record.gameId }}]</span>
                        <MatchList :match-data="[matches[i]]" :visible-names="[record.name]" />
                    </div>
                </div>
            </template>
            <template #fallback>
                <h1>Waiting for server data</h1>
            </template>
        </Suspense>
    </div>
</template>

<style lang="scss" scoped>
.gameId {
    display: none;
}

.record-title:hover > .gameId {
    display: block;
}
</style>
