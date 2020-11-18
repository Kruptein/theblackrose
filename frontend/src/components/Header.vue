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
                        <!-- <li id="alert-nav"> -->
                        <!-- <img src="{{ request.static_url('leaguenetwork:static/img/feather.png') }}" /> -->
                        <!-- </li> -->
                    </template>
                </template>
            </ul>
        </nav>
        <div id="notification-panel">
            <span class="font-semi-bold">Notifications</span>
            <div id="notification-panel-content">
                <div data-nid="-1">No new notifications!</div>
            </div>
        </div>
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
            border-radius: 0 0 0 20px;
        }

        &:last-child {
            border-radius: 0 0 20px 0;
        }
    }
}

#alert-nav {
    padding-top: 10px;
    padding-left: 5px;
    padding-right: 5px;

    &:hover {
        cursor: pointer;
        background-color: white !important;
    }

    img {
        height: 30px;
    }
}

#notification-panel {
    display: none;
    padding: 10px;
    background-color: #fff;
    color: #df4a5a;
    border-radius: 0 0 20px 20px;
    z-index: 20;
}

#notification-panel-content {
    display: flex;
    flex-direction: column;
}

.notification-panel-content-item {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    padding-top: 5px;
    padding-bottom: 5px;
    padding-right: 5px;
    max-width: 500px;

    &:not(:last-child) {
        border-bottom: solid 1px;
    }

    img:hover {
        height: 20px;
        cursor: pointer;
    }
}
</style>
