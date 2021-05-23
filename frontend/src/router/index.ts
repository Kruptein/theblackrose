import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";

import auth0 from "../plugins/auth0";
import Home from "../views/Home.vue";

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
        component: () => import(/* webpackChunkName: "matchfeed" */ "../views/MatchFeed.vue"),
    },
    {
        path: "/records",
        name: "Records",
        beforeEnter: auth0.routeGuard,
        component: () => import(/* webpackChunkName: "records" */ "../views/Records.vue"),
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
        component: () => import(/* webpackChunkName: "connectionfeed" */ "../views/connections/ConnectionFeed.vue"),
    },
    {
        path: "/connection/:name/records",
        name: "ConnectionRecords",
        beforeEnter: auth0.routeGuard,
        component: () =>
            import(/* webpackChunkName: "connectionrecords" */ "../views/connections/ConnectionRecords.vue"),
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
