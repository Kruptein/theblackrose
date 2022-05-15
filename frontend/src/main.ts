import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { createApp } from "vue";

import { backendUrl } from "./api/utils";
import App from "./App.vue";
import { auth0 } from "./auth";
import { loadChampionData } from "./ddragon";
import { loadFontAwesome } from "./fa";
import { initRouter } from "./router";
import { patches } from "./state";

loadFontAwesome();

async function init(): Promise<void> {
    const response = await fetch(backendUrl("/ddragon/patches.json"));
    patches.value = await response.json();
    await loadChampionData();

    const app = createApp(App);
    app.use(auth0).use(initRouter(app)).component("font-awesome-icon", FontAwesomeIcon).mount("#app");
}

init();
