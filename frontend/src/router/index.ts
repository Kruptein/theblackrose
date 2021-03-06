import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";

import auth0 from "../plugins/auth0";
import Home from "../views/Home.vue";

const ConnectionFeed = (): Promise<typeof import("*.vue")> =>
    import(/* webpackChunkName: "connectionfeed" */ "../views/connections/ConnectionFeed.vue");
const ConnectionRecords = (): Promise<typeof import("*.vue")> =>
    import(/* webpackChunkName: "connectionrecords" */ "../views/connections/ConnectionRecords.vue");

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
        path: "/records",
        name: "Records",
        beforeEnter: auth0.routeGuard,
        component: ConnectionRecords,
    },
    {
        path: "/connection/:name",
        name: "Connection",
        beforeEnter: auth0.routeGuard,
        component: () => import(/* webpackChunkName: "connection" */ "../views/connections/Connection.vue"),
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
        component: () => import(/* webpackChunkName: "connections" */ "../views/connections/Connections.vue"),
    },
    {
        path: "/connections/add",
        name: "AddConnections",
        beforeEnter: auth0.routeGuard,
        component: () => import(/* webpackChunkName: "addconnections" */ "../views/connections/AddConnections.vue"),
    },
];

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes,
});

export default router;
