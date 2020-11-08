import { createApp } from "vue";
import App from "./App.vue";
import Auth from "./plugins/auth0";
import router from "./router";

async function init() {
    const AuthPlugin = await Auth.init();
    createApp(App)
        .use(AuthPlugin)
        .use(router)
        .mount("#app");
}

init();
