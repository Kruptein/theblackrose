import { authPlugin } from "../plugins/auth0";

export function backendUrl(path: string): string {
    return `${process.env.VUE_APP_BACKEND_LOCATION}${path}`;
}

export async function getAuthHeader(): Promise<{ headers: { Authorization: string } }> {
    const token: string = await authPlugin.getTokenSilently();
    return { headers: { Authorization: `Bearer ${token}` } };
}
