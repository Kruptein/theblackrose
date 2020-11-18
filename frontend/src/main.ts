import { createApp } from "vue";
import App from "./App.vue";
import Auth from "./plugins/auth0";
import router from "./router";

async function init() {
    const AuthPlugin = await Auth.init(async (url: string) => {
        await router.isReady();
        await router.push(url);
    });
    createApp(App)
        .use(AuthPlugin)
        .use(router)
        .mount("#app");
}

init();
