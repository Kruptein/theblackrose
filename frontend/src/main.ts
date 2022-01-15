import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { createApp } from "vue";

import { backendUrl } from "./api/utils";
import App from "./App.vue";
import Auth from "./auth/core";
import { loadChampionData } from "./ddragon";
import { loadFontAwesome } from "./fa";
import router from "./router";
import { patches } from "./state";

loadFontAwesome();

async function init(): Promise<void> {
    await Auth.init();

    const response = await fetch(backendUrl("/ddragon/patches.json"));
    patches.value = await response.json();
    await loadChampionData();

    createApp(App).use(router).component("font-awesome-icon", FontAwesomeIcon).mount("#app");
}

init();
