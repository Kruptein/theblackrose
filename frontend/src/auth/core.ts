import type { User } from "@auth0/auth0-spa-js";
import createAuth0Client from "@auth0/auth0-spa-js";
import { reactive } from "vue";
import router from "../router";

export const authClient = await createAuth0Client({
    domain: import.meta.env.VITE_AUTH0_DOMAIN as string,
    client_id: import.meta.env.VITE_AUTH0_CLIENT_KEY as string,
    audience: import.meta.env.VITE_AUTH0_AUDIENCE as string,
    redirect_uri: window.location.origin,
});

interface AuthState {
    isAuthenticated: boolean;
    loading: boolean;
    user: User | undefined;
    popupOpen: boolean;
    error: any;
}

export const authState: AuthState = reactive({
    isAuthenticated: await authClient.isAuthenticated(),
    loading: true,
    user: undefined,
    popupOpen: false,
    error: null,
});

async function init() {
    try {
        // If the user is returning to the app after authentication
        if (window.location.search.includes("code=") && window.location.search.includes("state=")) {
            // handle the redirect and retrieve tokens
            const { appState } = await authClient.handleRedirectCallback();
            authState.isAuthenticated = await authClient.isAuthenticated();

            console.log("NAVIGATING TO", appState.targetUrl);
            // await router.isReady();
            const a = await router.push(appState.targetUrl);
            console.log(a);
        }
    } catch (e) {
        authState.error = e;
    } finally {
        authState.user = (await authClient.getUser()) ?? {};
        authState.loading = false;
    }
}

export default { init };
