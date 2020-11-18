<script lang="ts">
import { defineComponent } from "vue";
import MatchList from "../components/MatchList.vue";

export default defineComponent({
    name: "MatchFeed",
    components: { MatchList },
});
</script>

<template>
    <Suspense>
        <template #default>
            <MatchList />
        </template>
        <template #fallback>
            <div id="welcome">
                <img src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png" />

                <h1>Waiting for server data</h1>
            </div>
        </template>
    </Suspense>
</template>
