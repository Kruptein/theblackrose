import { NavigationGuardNext, RouteLocation } from "vue-router";
import { authClient, authState } from "./core";

export const auth0Guard = async (to: RouteLocation, _from: RouteLocation, next: NavigationGuardNext): Promise<void> => {
    // See if we're still authenticated
    authState.isAuthenticated = await authClient.isAuthenticated();

    // If the user is authenticated, continue with the route
    if (authState.isAuthenticated) {
        return next();
    }

    // Otherwise, log in
    next(false);
    authClient.loginWithRedirect({ appState: { targetUrl: to.fullPath } });
};
