<script lang="ts">
import { defineComponent } from "vue";
import { useRoute } from "vue-router";

import MatchFetcher from "../components/MatchFetcher.vue";

import { backendUrl, getAuthHeader } from "../api/utils";

export default defineComponent({
    name: "MatchFeed",
    components: { MatchFetcher },

    setup() {
        const route = useRoute();

        const refresh = async (): Promise<void> => {
            const headers = await getAuthHeader();
            console.log(await fetch(backendUrl(`/api/connection/${route.params.name}/refresh`), headers));
        };

        return { refresh };
    },
});
</script>

<template>
    <div id="welcome">
        <img src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png" />
        <div @click="refresh">Refresh</div>
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
</template>
