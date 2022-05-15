import { createAuth0 } from "@auth0/auth0-vue";

export const auth0 = createAuth0({
    domain: import.meta.env.VITE_AUTH0_DOMAIN,
    client_id: import.meta.env.VITE_AUTH0_CLIENT_KEY,
    audience: import.meta.env.VITE_AUTH0_AUDIENCE,
    redirect_uri: window.location.origin,
    cacheLocation: "localstorage",
});
