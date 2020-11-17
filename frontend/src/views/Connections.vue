<script lang="ts">
import { defineComponent, inject, onMounted, ref } from "vue";
import { AuthPlugin } from "../plugins/auth0";

export default defineComponent({
    name: "Connections",
    setup() {
        const auth = inject<AuthPlugin>("Auth")!;

        const connections = ref<string[]>([]);

        const getSummonerIconImage = (iconId: number): string => {
            return `http://localhost:9000/ddragon/10.23.1/img/profileicon/${iconId}.png`;
        };

        onMounted(async () => {
            const token: string = await auth.getTokenSilently();
            const response = await fetch("http://localhost:9000/api/connections/", {
                headers: { Authorization: `Bearer ${token}` },
            });
            const data: string[] = JSON.parse(await response.json());
            console.log(data[0]);
            connections.value.push(...data);
        });

        return { connections, getSummonerIconImage };
    },
});
</script>

<template>
    <div id="welcome">
        <img
            class="logo img-responsive"
            src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png"
        />

        <!-- <div id="connection-nav">
            <ul>
                <li><a href="{{ request.current_route_url() }}">Network Overview</a></li>

                <li><a href="/connections/records/">Network Records</a></li>

                <li><a href="/connections/winrates/">Network Winrates</a></li>
            </ul>
        </div> -->

        <h1>Your Connections</h1>
        <template v-if="connections.length > 0">
            <p class="lead">
                <a href="/connections/add" class="font-normal">Add more connections</a>
                to gain more insight.
            </p>
            <div id="connections">
                <template v-for="connection of connections" :key="connection">
                    <!-- <a style="display: block" :href="'/connections/' + connection"> -->
                    <div class="connection">
                        <img :src="getSummonerIconImage(connection[1])" />
                        <div class="connection-name">{{ connection[0] }}</div>
                    </div>
                    <!-- </a> -->
                </template>
            </div>
        </template>
        <template v-else>
            <p class="lead">
                Your network is rather empty.
                <router-link to="/connections/add">Add some connections</router-link>
                to infiltrate their data.
            </p>
        </template>
    </div>
</template>

<style lang="scss" scoped>
.record-title {
    background-color: #df4a5a;
    margin-bottom: -5px;
    margin-top: 10px;
    width: max-content;
    padding: 5px;
    border: solid 2px #c84251;
    box-shadow: 2px 2px #c84251;
}

#connection-nav {
    position: fixed;
    bottom: 50px;
    right: 0;

    ul {
        display: flex;
        flex-flow: column;
        list-style: none;
        font-size: 25px;
        font-weight: 500;
    }

    li {
        background-color: #df4a5a;
        display: flex;

        a {
            &:hover {
                color: #d22537;
                background-color: white;
            }

            display: flex;
            padding: 15px;
            text-decoration: none;
            color: #fff;
            width: 100%;
        }

        &:first-child {
            border-radius: 20px 0 0 0;

            a:hover {
                border-radius: 20px 0 0 0;
            }
        }

        &:last-child {
            border-radius: 0 0 0 20px;

            a:hover {
                border-radius: 0 0 0 20px;
            }
        }
    }
}

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

    /* image height - 2*padding - 2*border */
}
</style>
