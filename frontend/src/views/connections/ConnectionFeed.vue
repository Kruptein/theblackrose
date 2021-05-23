<script lang="ts">
import { defineComponent } from "vue";
import { useRoute } from "vue-router";

import { backendUrl, getAuthHeader } from "../../api/utils";
import MatchFetcher from "../../components/MatchFetcher.vue";

import ConnectionHeader from "./ConnectionHeader.vue";

// eslint-disable-next-line import/no-unused-modules
export default defineComponent({
    name: "MatchFeed",
    components: { ConnectionHeader, MatchFetcher },

    setup() {
        const route = useRoute();

        async function refresh(): Promise<void> {
            const headers = await getAuthHeader();
            await fetch(backendUrl(`/api/connection/${route.params.name}/refresh`), headers);
        }

        return { refresh };
    },
});
</script>

<template>
    <main>
        <ConnectionHeader active="matches" />
        <div id="matches">
            <Suspense>
                <template #default>
                    <div style="display: contents">
                        <h1>Recent observations</h1>
                        <MatchFetcher :filter="{ names: [$route.params.name] }" />
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
    grid-template-rows: 250px 1fr;

    #matches {
        grid-area: matches;
    }
}
</style>
