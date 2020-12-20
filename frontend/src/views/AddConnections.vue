<script lang="ts">
import { defineComponent, ref } from "vue";

import { backendUrl, getAuthHeader } from "../api/utils";

export default defineComponent({
    name: "AddConnections",
    setup() {
        const summoner = ref("");

        const message = ref("We just need the name of the summoner you want to inquire.");

        async function onSubmit() {
            message.value = "Submitting to server";
            const headers = await getAuthHeader();
            const data = await fetch(backendUrl(`/api/connections/${summoner.value}/`), headers);
            if (data.status === 201) {
                message.value = "Successfully added connection. Matches are being processed, this can take some time.";
            } else if (data.status === 404) {
                message.value = "Given username was not found!";
            } else if (data.status === 409) {
                message.value = "Given username is already a connection!";
            } else {
                message.value = "Something went wrong.";
            }
        }

        return { onSubmit, message, summoner };
    },
});
</script>

<template>
    <div id="welcome">
        <img src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png" />

        <h1>Add new connection</h1>
        <p>
            {{ message }}
        </p>
        <form style="margin-top: 10px" @submit.prevent="onSubmit">
            <div>
                <label for="summoner">Summoner Name</label>
                <br />
                <input type="text" name="summoner" v-model="summoner" placeholder="summoner name" />
            </div>
            <div>
                <button type="submit" name="form.submitted" value="Log In">Inquire</button>
            </div>
        </form>
    </div>
</template>
