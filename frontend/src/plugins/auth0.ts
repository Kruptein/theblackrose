import createAuth0Client, { Auth0Client, IdToken, RedirectLoginOptions } from "@auth0/auth0-spa-js";
import { App, computed, reactive, watchEffect } from "vue";
import { NavigationGuardNext, RouteLocation } from "vue-router";

let client: Auth0Client;
const state = reactive({
    loading: true,
    isAuthenticated: false,
    user: {},
    popupOpen: false,
    error: null,
});

async function loginWithPopup(): Promise<void> {
    state.popupOpen = true;

    try {
        await client.loginWithPopup();
    } catch (e) {
        console.error(e);
    } finally {
        state.popupOpen = false;
    }

    state.user = (await client.getUser()) ?? {};
    state.isAuthenticated = true;
}

async function handleRedirectCallback(): Promise<void> {
    state.loading = true;

    try {
        await client.handleRedirectCallback();
        state.user = (await client.getUser()) ?? {};
        state.isAuthenticated = true;
    } catch (e) {
        state.error = e;
    } finally {
        state.loading = false;
    }
}

export const authPlugin = {
    isAuthenticated: computed(() => state.isAuthenticated),
    loading: computed(() => state.loading),
    user: computed(() => state.user),
    getIdTokenClaims: (): Promise<IdToken> => client.getIdTokenClaims(),
    getTokenSilently: (): Promise<any> => client.getTokenSilently(),
    getTokenWithPopup: (): Promise<string> => client.getTokenWithPopup(),
    handleRedirectCallback,
    loginWithRedirect: (o: RedirectLoginOptions | undefined): Promise<void> => client.loginWithRedirect(o),
    loginWithPopup,
    logout: (): void => client.logout(),
};

const routeGuard = (to: RouteLocation, _from: RouteLocation, next: NavigationGuardNext): void => {
    const { isAuthenticated, loading, loginWithRedirect } = authPlugin;

    const verify = (): void => {
        // If the user is authenticated, continue with the route
        if (isAuthenticated.value) {
            return next();
        }

        // Otherwise, log in
        loginWithRedirect({ appState: { targetUrl: to.fullPath } });
    };

    // If loading has already finished, check our auth state using `fn()`
    if (!loading.value) {
        return verify();
    }

    // Watch for the loading property to change before we check isAuthenticated
    watchEffect(() => {
        if (loading.value === false) {
            return verify();
        }
    });
};

async function init(onRedirectCallback: (url: string) => void): Promise<{ install(app: App): void }> {
    // const { onRedirectCallback, redirectUri = window.location.origin } = options;
    const redirectUri = window.location.origin;

    client = await createAuth0Client({
        domain: import.meta.env.VITE_AUTH0_DOMAIN,
        client_id: import.meta.env.VITE_AUTH0_CLIENT_KEY,
        audience: import.meta.env.VITE_AUTH0_AUDIENCE,
        redirect_uri: redirectUri,
    });

    try {
        // If the user is returning to the app after authentication
        if (window.location.search.includes("code=") && window.location.search.includes("state=")) {
            // handle the redirect and retrieve tokens
            const { appState } = await client.handleRedirectCallback();

            // Notify subscribers that the redirect callback has happened, passing the appState
            // (useful for retrieving any pre-authentication state)
            // onRedirectCallback(appState);
            onRedirectCallback(appState.targetUrl);
        }
    } catch (e) {
        state.error = e;
    } finally {
        // Initialize our internal authentication state
        state.isAuthenticated = await client.isAuthenticated();
        state.user = (await client.getUser()) ?? {};
        state.loading = false;
    }

    return {
        install: (app: App) => {
            app.provide("Auth", authPlugin);
        },
    };
}

export default {
    init,
    routeGuard,
};

export type AuthPlugin = typeof authPlugin;
