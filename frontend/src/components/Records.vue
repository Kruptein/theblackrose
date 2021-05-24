<script lang="ts">
import { defineComponent, PropType } from "vue";

import MatchList from "../components/MatchList.vue";
import { MatchFeedElement } from "../models/matchfeed";
import { getQueueFromId } from "../models/queue";
import { Record, RecordType } from "../models/records";

// eslint-disable-next-line import/no-unused-modules
export default defineComponent({
    name: "Records",
    components: { MatchList },
    props: {
        records: { type: Object as PropType<Record[]>, required: true },
        matches: { type: Object as PropType<MatchFeedElement[]>, required: true },
    },
    setup() {
        return { getQueueFromId, RecordType };
    },
});
</script>

<template>
    <div style="display: contents">
        <h1>Records</h1>
        <div v-for="(record, i) in records" class="record-title" :key="record.id">
            <span class="font-semi-bold">{{ RecordType[record.recordType] }}</span>
            by {{ record.name }}
            :
            <span class="font-semi-bold">{{ record.value }}</span>
            <span class="gameId">[{{ record.gameId }}]</span>
            <MatchList :match-data="[matches[i]]" :visible-names="[record.name]" />
        </div>
    </div>
</template>

<style lang="scss" scoped>
.record-title {
    position: relative;

    .gameId {
        position: absolute;
        left: 50%;
        display: none;
        font-size: xx-small;
    }

    &:hover > .gameId {
        display: block;
    }
}

.font-semi-bold {
    font-size: 25px;
    font-weight: bold;
}
</style>

<style lang="scss">
.record-title {
    .matches {
        margin-top: 2em;
        margin-bottom: 5em;
    }

    &:last-of-type {
        .matches {
            padding-bottom: 10em;
        }
    }
}
</style>
