<script lang="ts">
import { defineComponent, inject, ref } from "vue";
import { AuthPlugin } from "../plugins/auth0";
import { backendUrl } from "../utils";

export default defineComponent({
    name: "AddConnections",
    setup() {
        const summoner = ref("");

        const auth = inject<AuthPlugin>("Auth")!;

        async function onSubmit() {
            const token: string = await auth.getTokenSilently();
            const data = await fetch(backendUrl(`/api/connections/${summoner.value}/`), {
                method: "POST",
                headers: { Authorization: `Bearer ${token}` },
            });
            if (data.status === 201) {
                console.log("Successfully added connection.");
            } else if (data.status === 404) {
                console.log("Given username was not found");
            } else if (data.status === 409) {
                console.log("Given username is already a connection!");
            } else {
                console.log("Something went wrong.");
            }
        }

        return { onSubmit, summoner };
    },
});
</script>

<template>
    <div id="welcome">
        <img
            class="logo img-responsive"
            src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png"
        />

        <div id="connection-nav">
            <ul>
                <li><a href="{{ request.current_route_url() }}">Network Overview</a></li>
                <li><a href="/connections/records/">Network Records</a></li>
                <li><a href="/connections/winrates/">Network Winrates</a></li>
            </ul>
        </div>

        <h1>Add new connection</h1>
        <p class="lead">
            We just need the name of the summoner you want to inquire
            <!-- {% if message %} {{ message }} {% else %} We just need the name of the summoner you want to inquire.{% endif
            %} -->
        </p>
        <!-- {% if state == 'empty' %} -->
        <form style="margin-top: 10px" @submit.prevent="onSubmit">
            <div class="form-group">
                <label for="summoner" class="font-normal">Summoner Name</label>
                <br />
                <input type="text" name="summoner" v-model="summoner" placeholder="summoner name" />
            </div>
            <div class="form-group">
                <button type="submit" name="form.submitted" value="Log In">Inquire</button>
            </div>
        </form>
        <!-- {% elif state == 'pending' %}
        <a href="/friendface" class="font-bold">Return to your connections</a>
        {% elif state == 'processed' %}
        <a href="/connections/{{ summoner }}" class="font-bold">Analyze summoner data</a>
        {% endif %} -->
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
