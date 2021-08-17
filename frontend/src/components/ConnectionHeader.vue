<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";

import { backendUrl, getAuthHeader } from "../api/utils";
import { getSummonerIconImage } from "../common";
import { Summoner } from "../models/match";
import { connectionStore } from "../store/connections";

const props = withDefaults(defineProps<{ active: string; apiLoad?: boolean }>(), { apiLoad: true });

const route = useRoute();

const connection = computed(() => connectionStore.getConnection(route.params.name as string));

onMounted(async () => {
    if (props.apiLoad) {
        const headers = await getAuthHeader();

        const response = await fetch(backendUrl(`/api/summoners/${route.params.name}/`), headers);
        const data: { core: Summoner } = JSON.parse(await response.json());
        connectionStore.addConnections(data.core);
    }
});

async function refresh(): Promise<void> {
    const headers = await getAuthHeader();
    await fetch(backendUrl(`/api/connection/${route.params.name}/refresh`), headers);
}
</script>

<template>
    <div id="connection-header">
        <div id="icon" v-if="connection !== undefined">
            <img :src="getSummonerIconImage(connection.profileIconId)" />
        </div>
        <div id="connection-name">
            <div>Surveillance report for</div>
            <template v-if="connection === undefined">... LOADING ...</template>
            <template v-else>{{ connection.name }} [{{ connection.summonerLevel }}]</template>
        </div>
        <nav aria-label="Connection navigation" v-if="connection !== undefined">
            <div :class="{ active: active === 'overview' }">
                <router-link :to="'/connection/' + connection.name">Overview</router-link>
            </div>
            <div :class="{ active: active === 'matches' }">
                <router-link :to="'/connection/' + connection.name + '/feed'">Matches</router-link>
            </div>
            <div :class="{ active: active === 'records' }">
                <router-link :to="'/connection/' + connection.name + '/records'">Records</router-link>
            </div>
            <!-- <div :class="{ active: active === 'stats' }">
                <router-link :to="'/connection/' + connection.name + '/feed'">Stats</router-link>
            </div> -->
        </nav>
    </div>
</template>

<style scoped lang="scss">
#connection-header {
    grid-area: header;
    display: grid;
    grid-template-areas:
        "icon name"
        "icon nav";

    column-gap: 5em;
    row-gap: 1em;

    justify-content: flex-start;
    align-items: center;

    #icon {
        grid-area: icon;

        img {
            width: 150px;
        }
    }

    #connection-name {
        grid-area: name;
        font-size: 50px;
        font-weight: bold;
        position: relative;

        justify-self: flex-start;
        align-self: flex-end;

        div {
            position: absolute;
            top: -20px;
            left: -2em;

            font-weight: normal;
            font-size: 25px;
            font-style: italic;
        }
    }

    nav {
        align-self: flex-start;
        display: flex;
        font-style: italic;

        a {
            text-decoration: none;
            color: inherit;

            &:hover {
                font-weight: bold;
            }
        }

        div {
            margin-right: 1em;

            &::before {
                content: "[";
            }

            &::after {
                content: "]";
            }
        }

        .active {
            font-style: normal;
            font-weight: bold;

            &::before {
                content: "<";
            }

            &::after {
                content: ">";
            }
        }
    }
}
</style>
