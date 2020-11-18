<script lang="ts">
import { inject } from "vue";
import { AuthPlugin } from "../plugins/auth0";
export default {
    setup() {
        const auth = inject<AuthPlugin>("Auth");
        return { auth };
    },
};
</script>

<template>
    <div id="toppanel">
        <nav>
            <ul>
                <template v-if="!auth.loading.value">
                    <template v-if="!auth.isAuthenticated.value">
                        <li><a href="#" @click="auth.loginWithRedirect">Register / Login</a></li>
                    </template>
                    <template v-else>
                        <li><router-link to="/feed">Recent games</router-link></li>
                        <li><router-link to="/connections">Network</router-link></li>
                        <li @click="auth.logout"><a href="#">Logout</a></li>
                    </template>
                </template>
            </ul>
        </nav>
    </div>
</template>

<style lang="scss" scoped>
#toppanel {
    position: fixed;
    top: 0;
    right: 50px;
    display: flex;
    flex-direction: column;
    background-color: #df4a5a;
    color: #fff;
    border-radius: 0 0 20px 20px;
    padding: 0;
}

nav {
    ul {
        display: flex;
        justify-content: flex-end;
        list-style: none;
        font-size: 25px;
        font-weight: 500;
        padding: 0;
        margin: 0;
    }

    li {
        display: flex;

        & > * {
            padding: 20px;
            text-decoration: none;
            color: inherit;
        }

        &:hover {
            color: #d22537;
            background-color: white;
        }

        &:first-child {
            border-bottom-left-radius: 20px;
        }

        &:last-child {
            border-bottom-right-radius: 20px;
        }
    }
}
</style>
