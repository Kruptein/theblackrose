<script lang="ts">
import { defineComponent, onMounted, ref } from "vue";
import { useRoute } from "vue-router";

import { backendUrl, getAuthHeader } from "../../api/utils";

// eslint-disable-next-line import/no-unused-modules
export default defineComponent({
    name: "Live",
    setup() {
        const route = useRoute();
        const summonerName = route.params.summoner;
        const liveStuff = ref<{ summoner: string; champion: number; wins: number; total: number }[]>();

        onMounted(async () => {
            const headers = await getAuthHeader();
            const response = await fetch(backendUrl(`/api/live/${summonerName}/`), headers);
            liveStuff.value = JSON.parse(await response.json());
        });

        return { liveStuff, summonerName };
    },
});
</script>

<template>
    <div id="welcome">
        <img src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png" />

        <h1>Live game info for</h1>
        <div v-for="champ in liveStuff" :key="champ.summoner">
            <div v-if="champ.total > 0">{{ champ.summoner }} - {{ champ.wins }} / {{ champ.total }}</div>
        </div>
    </div>
</template>
