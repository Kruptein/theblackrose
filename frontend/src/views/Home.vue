<template>
    <div class="home">
        <img alt="Vue logo" src="../assets/logo.png" />
        <HelloWorld msg="Welcome to Your Vue.js + TypeScript App" />

        <!-- Check that the SDK client is not currently loading before accessing is methods -->
        <div v-if="!$auth.loading.value">
            <!-- show login when not authenticated -->
            <button v-if="!$auth.isAuthenticated.value" @click="$auth.loginWithRedirect">Log in</button>
            <!-- show logout when authenticated -->
            <button v-if="$auth.isAuthenticated.value" @click="$auth.logout">Log out</button>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import HelloWorld from "@/components/HelloWorld.vue"; // @ is an alias to /src

export default defineComponent({
    name: "Home",
    components: {
        HelloWorld,
    },
});
</script>
