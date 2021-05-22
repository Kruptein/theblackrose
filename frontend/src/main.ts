import { createApp } from "vue";

import { backendUrl } from "./api/utils";
import App from "./App.vue";
import Auth from "./plugins/auth0";
import router from "./router";
import { patches } from "./state";

async function init(): Promise<void> {
    const AuthPlugin = await Auth.init(async (url: string) => {
        await router.isReady();
        await router.push(url);
    });

    const response = await fetch(backendUrl("/ddragon/patches.json"));
    patches.value = await response.json();

    createApp(App).use(AuthPlugin).use(router).mount("#app");
}

init();
