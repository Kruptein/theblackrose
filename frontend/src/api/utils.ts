import { authPlugin } from "../plugins/auth0";

export async function getHeader(): Promise<{ headers: { Authorization: string } }> {
    const token: string = await authPlugin.getTokenSilently();
    return { headers: { Authorization: `Bearer ${token}` } };
}
