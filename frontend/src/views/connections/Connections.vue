<script lang="ts">
import { computed, defineComponent, onMounted, ref } from "vue";

import { backendUrl, getAuthHeader } from "../../api/utils";
import { getSummonerIconImage } from "../../common";
import { Connection } from "../../models/connections";
import { connectionStore } from "../../store/connections";
import { alphSort } from "../../utils";

// eslint-disable-next-line import/no-unused-modules
export default defineComponent({
    name: "Connections",
    setup() {
        const loading = ref(true);

        const connections = computed(() =>
            [...connectionStore.getConnections().values()].sort((a, b) => alphSort(a.name, b.name)),
        );

        onMounted(async () => {
            const headers = await getAuthHeader();
            const response = await fetch(backendUrl("/api/connections/"), headers);
            const data: Connection[] = JSON.parse(await response.json());
            connectionStore.addConnections(...data);
            loading.value = false;
        });

        return { connections, getSummonerIconImage, loading };
    },
});
</script>

<template>
    <div id="welcome">
        <img
            class="logo img-responsive"
            src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png"
        />

        <template v-if="loading && connections.length === 0">
            <h1>Waiting for server data</h1>
        </template>
        <template v-else-if="connections.length > 0">
            <h1>Your Connections</h1>
            <p>
                <router-link to="/connections/add" class="font-normal">Add more connections</router-link>
                to gain more insight.
            </p>
            <div id="connections">
                <template v-for="connection of connections" :key="connection">
                    <router-link :to="'/connection/' + connection.name" class="connection">
                        <img :src="getSummonerIconImage(connection.profileIconId)" />
                        <div class="connection-name">{{ connection.name }}</div>
                    </router-link>
                </template>
            </div>
        </template>
        <template v-else>
            <p>
                Your network is rather empty.
                <router-link to="/connections/add">Add some connections</router-link>
                to infiltrate their data.
            </p>
        </template>
    </div>
</template>

<style lang="scss" scoped>
#connections {
    margin-top: 20px;
    display: grid;
    grid-gap: 15px;
    grid-auto-flow: row dense;
    grid-template-columns: repeat(4, 250px);
}

.connection {
    background-color: #df4a5a;
    border-radius: 30px;
    padding: 0;
    display: flex;
    text-decoration: none;
    color: inherit;

    &:hover {
        background-color: white;
        color: #df4a5a;
        cursor: pointer;
    }

    img {
        min-width: 64px;
        height: 64px;
        border-radius: 30px 0 0 30px;
    }

    .connection-name {
        border: double 1px white;
        border-left: 0;
        width: 100%;
        border-radius: 0 30px 30px 0;
        padding: 10px;
        text-align: center;
        font-size: 20px;
        font-weight: 600;
        line-height: 42px;
    }
}
</style>
