<script lang="ts">
import { defineComponent } from "vue";

import MatchFetcher from "../components/MatchFetcher.vue";

// eslint-disable-next-line import/no-unused-modules
export default defineComponent({
    name: "MatchFeed",
    components: { MatchFetcher },
});
</script>

<template>
    <div id="welcome">
        <img src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png" />
        <Suspense>
            <template #default>
                <div style="display: contents">
                    <h1>Recent observations</h1>
                    <MatchFetcher />
                </div>
            </template>
            <template #fallback>
                <h1>Waiting for server data</h1>
            </template>
        </Suspense>
    </div>
</template>
