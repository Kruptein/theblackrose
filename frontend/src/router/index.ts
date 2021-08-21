import type { RouteRecordRaw } from "vue-router";
import { createRouter, createWebHistory } from "vue-router";

import auth0 from "../plugins/auth0";
import Home from "../views/Home.vue";

const ConnectionFeed = (): Promise<typeof import("*.vue")> => import("../views/connections/ConnectionFeed.vue");
const ConnectionRecords = (): Promise<typeof import("*.vue")> => import("../views/connections/ConnectionRecords.vue");

const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "Home",
        component: Home,
    },
    {
        path: "/feed",
        name: "MatchFeed",
        beforeEnter: auth0.routeGuard,
        component: ConnectionFeed,
    },
    {
        path: "/stats",
        name: "Stats",
        beforeEnter: auth0.routeGuard,
        component: () => import("../views/stats/Stats.vue"),
    },
    {
        path: "/records",
        name: "Records",
        beforeEnter: auth0.routeGuard,
        component: ConnectionRecords,
    },
    {
        path: "/connection/:name",
        name: "Connection",
        beforeEnter: auth0.routeGuard,
        component: () => import("../views/connections/Connection.vue"),
    },
    {
        path: "/connection/:name/feed",
        name: "ConnectionFeed",
        beforeEnter: auth0.routeGuard,
        component: ConnectionFeed,
    },
    {
        path: "/connection/:name/records",
        name: "ConnectionRecords",
        beforeEnter: auth0.routeGuard,
        component: ConnectionRecords,
    },
    {
        path: "/connections",
        name: "Connections",
        beforeEnter: auth0.routeGuard,
        component: () => import("../views/connections/Connections.vue"),
    },
    {
        path: "/connections/add",
        name: "AddConnections",
        beforeEnter: auth0.routeGuard,
        component: () => import("../views/connections/AddConnections.vue"),
    },
    {
        path: "/live/:summoner",
        name: "LiveGame",
        beforeEnter: auth0.routeGuard,
        component: () => import("../views/live/Live.vue"),
    },
];

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes,
});

export default router;
