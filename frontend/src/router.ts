import { createAuthGuard } from "@auth0/auth0-vue";
import { App } from "vue";
import type { Router, RouteRecordRaw } from "vue-router";
import { createRouter, createWebHistory } from "vue-router";

import Home from "./views/Home.vue";

const ConnectionFeed = (): Promise<typeof import("*.vue")> => import("./views/connections/ConnectionFeed.vue");
const ConnectionRecords = (): Promise<typeof import("*.vue")> => import("./views/connections/ConnectionRecords.vue");

export function initRouter(app: App): Router {
    const authGuard = createAuthGuard(app);

    const routes: Array<RouteRecordRaw> = [
        {
            path: "/",
            name: "Home",
            component: Home,
        },
        {
            path: "/feed",
            name: "MatchFeed",
            beforeEnter: authGuard,
            component: ConnectionFeed,
        },
        {
            path: "/stats",
            name: "Stats",
            beforeEnter: authGuard,
            component: () => import("./views/stats/Stats.vue"),
        },
        {
            path: "/records",
            name: "Records",
            beforeEnter: authGuard,
            component: ConnectionRecords,
        },
        {
            path: "/connection/:name",
            name: "Connection",
            beforeEnter: authGuard,
            component: () => import("./views/connections/Connection.vue"),
        },
        {
            path: "/connection/:name/feed",
            name: "ConnectionFeed",
            beforeEnter: authGuard,
            component: ConnectionFeed,
        },
        {
            path: "/connection/:name/records",
            name: "ConnectionRecords",
            beforeEnter: authGuard,
            component: ConnectionRecords,
        },
        {
            path: "/connections",
            name: "Connections",
            beforeEnter: authGuard,
            component: () => import("./views/connections/Connections.vue"),
        },
        {
            path: "/connections/add",
            name: "AddConnections",
            beforeEnter: authGuard,
            component: () => import("./views/connections/AddConnections.vue"),
        },
        {
            path: "/connection/:name/live",
            name: "LiveGame",
            beforeEnter: authGuard,
            component: () => import("./views/connections/Live.vue"),
        },
    ];

    return createRouter({
        history: createWebHistory(import.meta.env.BASE_URL),
        routes,
    });
}
