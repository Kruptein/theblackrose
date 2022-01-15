<script setup lang="ts">
import { ref, onMounted, Ref } from "vue";

import { backendUrl, getAuthHeader } from "../api/utils";
import { Notification } from "../models/notifications";
import { authClient, authState } from "../auth/core";

const showNotifications = ref(false);
const notifications: Ref<Notification[]> = ref([]);

async function getNotifications(): Promise<void> {
    if (authState.isAuthenticated) {
        const headers = await getAuthHeader();
        const response = await fetch(backendUrl("/api/notifications/"), headers);
        notifications.value = await response.json();
    }
}

async function removeNotification(sliceId: number): Promise<void> {
    if (authState.isAuthenticated) {
        const notificationId = notifications.value[sliceId].id;
        const headers = await getAuthHeader();
        const response = await fetch(backendUrl(`/api/notifications/${notificationId}/`), {
            method: "delete",
            ...headers,
        });
        if (response.status === 200) {
            notifications.value = notifications.value.filter((n) => n.id !== notificationId);
        }
    }
}

onMounted(() => {
    setInterval(getNotifications, 15 * 60 * 1000);
    getNotifications();
});
</script>

<template>
    <div id="toppanel">
        <nav>
            <ul>
                <template v-if="!authState.loading">
                    <template v-if="!authState.isAuthenticated">
                        <li><a href="#" @click="authClient.loginWithRedirect()">Register / Login</a></li>
                    </template>
                    <template v-else>
                        <li
                            @click="showNotifications = !showNotifications"
                            :class="{ hasNotifications: notifications.length > 0, showNotifications }"
                        >
                            <img id="feather" :src="backendUrl('/ddragon/feather.png')" />
                        </li>
                        <li><router-link to="/feed">Recent games</router-link></li>
                        <li><router-link to="/stats">Stats</router-link></li>
                        <li><router-link to="/records">Records</router-link></li>
                        <li><router-link to="/connections">Network</router-link></li>
                        <li @click="authClient.logout()" :class="{ showNotifications }">
                            <a href="#">Logout</a>
                        </li>
                    </template>
                </template>
            </ul>
        </nav>
        <template v-if="showNotifications">
            <div id="notifications">
                <div
                    class="notification"
                    v-for="[i, notification] of notifications.slice(0, 10).entries()"
                    :key="notification.id"
                >
                    <div class="remove" title="Remove notification" @click="removeNotification(i)">X</div>
                    <div class="content">
                        <div class="title">{{ notification.title }}</div>
                        <div class="message">{{ notification.message }}</div>
                    </div>
                </div>
                <div v-if="notifications.length === 0" id="empty">No new notifications :)</div>
            </div>
            <div id="notifications-footer"></div>
        </template>
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
    border: solid 2px white;
    border-top: 0;
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

        &:not(&.showNotifications) {
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
    max-height: 50vh;
    border-bottom: 0;
    border-top: dashed 2px white;

    display: flex;
    flex-direction: column;
    align-items: stretch;

    #empty {
        padding-top: 20px;
    }

    .notification {
        display: flex;
        align-items: center;

        .remove {
            padding: 15px 10px;
            display: none;
            font-weight: bold;
            background-color: #df4a5a;
            user-select: none;
        }

        &:hover {
            border-top: solid 2px white;
            border-bottom: solid 2px white;

            .remove {
                display: block;

                &:hover {
                    cursor: pointer;
                }
            }

            .content {
                color: #d22537;
                background-color: #fff;
            }
        }

        .content {
            padding: 15px;
            display: flex;
            flex-direction: column;
            flex: 1;

            .title {
                align-self: flex-start;
                font-weight: bold;
            }

            .message {
                align-self: flex-end;
            }
        }
    }
}

#notifications-footer {
    height: 20px;
}
</style>
