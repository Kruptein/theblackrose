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
        component: () => import(/* webpackChunkName: "connections" */ "../views/MatchFeed.vue"),
    },
    {
        path: "/connection/:name/feed",
        name: "ConnectionFeed",
        beforeEnter: auth0.routeGuard,
        component: () => import(/* webpackChunkName: "connections" */ "../views/ConnectionFeed.vue"),
    },
    {
        path: "/connections",
        name: "Connections",
        beforeEnter: auth0.routeGuard,
        component: () => import(/* webpackChunkName: "connections" */ "../views/Connections.vue"),
    },
    {
        path: "/connections/add",
        name: "AddConnections",
        beforeEnter: auth0.routeGuard,
        component: () => import(/* webpackChunkName: "addconnections" */ "../views/AddConnections.vue"),
    },
];

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes,
});

export default router;
