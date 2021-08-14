import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { createApp } from "vue";

import { backendUrl } from "./api/utils";
import App from "./App.vue";
import { loadChampionData } from "./ddragon";
import { loadFontAwesome } from "./fa";
import Auth from "./plugins/auth0";
import router from "./router";
import { patches } from "./state";

loadFontAwesome();

async function init(): Promise<void> {
    const AuthPlugin = await Auth.init(async (url: string) => {
        await router.isReady();
        await router.push(url);
    });

    const response = await fetch(backendUrl("/ddragon/patches.json"));
    patches.value = await response.json();
    await loadChampionData();

    createApp(App).use(AuthPlugin).use(router).component("font-awesome-icon", FontAwesomeIcon).mount("#app");
}

init();
