import { createApp } from "vue";
import App from "./App.vue";
import Auth from "./plugins/auth0";
import router from "./router";
import { patches } from "./state";
import { backendUrl } from "./utils";

async function init() {
    const AuthPlugin = await Auth.init(async (url: string) => {
        await router.isReady();
        await router.push(url);
    });

    const response = await fetch(backendUrl("/ddragon/patches.json"));
    patches.value = await response.json();

    createApp(App)
        .use(AuthPlugin)
        .use(router)
        .mount("#app");
}

init();
