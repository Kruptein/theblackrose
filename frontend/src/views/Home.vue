<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: "Home",
});
</script>

<template>
    <div id="welcome">
        <img
            class="logo img-responsive"
            src="https://vignette3.wikia.nocookie.net/leagueoflegends/images/6/6c/Black_Rose.png"
        />
        <h1>
            <span class="font-semi-bold">The Black Rose</span>
            <span class="smaller">A plopkoek project</span>
        </h1>
        <p class="lead">
            Welcome, this is a
            <span class="font-normal">league of legends</span>
            project aimed at staying up to date with the progress of your friends.
        </p>
    </div>
</template>
