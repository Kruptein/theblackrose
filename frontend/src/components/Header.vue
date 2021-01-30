<script lang="ts">
import { inject, ref, onMounted, Ref } from "vue";
import { backendUrl, getAuthHeader } from "../api/utils";
import { Notification } from "../models/notifications";
import { AuthPlugin } from "../plugins/auth0";

export default {
    setup() {
        const auth = inject<AuthPlugin>("Auth")!;
        const showNotifications = ref(false);
        const notifications: Ref<Notification[]> = ref([]);

        const getNotifications = async () => {
            const headers = await getAuthHeader();
            const response = await fetch(backendUrl("/api/notifications/"), headers);
            notifications.value = JSON.parse(await response.json());
        };

        onMounted(() => {
            setInterval(getNotifications, 15 * 60 * 1000);
            getNotifications();
        });

        return { auth, backendUrl, notifications, showNotifications };
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
                        <li
                            @click="showNotifications = !showNotifications"
                            :class="{ hasNotifications: notifications.length > 0, showNotifications }"
                        >
                            <img id="feather" :src="backendUrl('/ddragon/feather.png')" />
                        </li>
                        <li><router-link to="/feed">Recent games</router-link></li>
                        <li><router-link to="/records">Records</router-link></li>
                        <li><router-link to="/connections">Network</router-link></li>
                        <li @click="auth.logout" :class="{ showNotifications }">
                            <a href="#">Logout</a>
                        </li>
                    </template>
                </template>
            </ul>
        </nav>
        <div v-if="showNotifications" id="notifications">
            <div class="notification" v-for="notification of notifications" :key="notification.id">
                {{ notification.title }}
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

        #feather {
            width: 25px;

            &:hover {
                cursor: pointer;
            }
        }

        & > * {
            padding: 20px;
            text-decoration: none;
            color: inherit;
        }

        &:hover,
        &.hasNotifications {
            color: #d22537;
            background-color: white;
        }

        :not(&.showNotifications) {
            &:first-child {
                border-bottom-left-radius: 20px;
            }

            &:last-child {
                border-bottom-right-radius: 20px;
            }
        }
    }
}

#notifications {
    padding: 10px;
    background-color: #fff;
    color: #df4a5a;
    overflow: auto;
    max-height: 50vh;

    .notification {
        padding: 10px 0;
    }
}
</style>
