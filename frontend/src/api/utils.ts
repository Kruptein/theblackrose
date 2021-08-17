import type { MatchFeedFilter } from "../models/matchfeed";
import { authPlugin } from "../plugins/auth0";

export function backendUrl(path: string): string {
    return `${import.meta.env.VITE_BACKEND_LOCATION}${path}`;
}

export async function getAuthHeader(): Promise<{ headers: { Authorization: string } }> {
    const token: string = await authPlugin.getTokenSilently();
    return { headers: { Authorization: `Bearer ${token}` } };
}

export async function fetchWithQuery<T>(apiPath: string, filter?: MatchFeedFilter): Promise<T> {
    const headers = await getAuthHeader();
    let matchUrl = apiPath;
    const queries: string[] = [];
    if ((filter?.names?.length ?? 0) > 0) {
        queries.push(`names=${filter!.names}`);
    }
    if (filter?.after !== undefined) {
        queries.push(`after=${filter.after}`);
    }
    if (filter?.before !== undefined) {
        queries.push(`before=${filter.before}`);
    }
    if (filter?.length !== undefined) {
        queries.push(`length=${filter.length}`);
    }
    if ((filter?.queues?.length ?? 0) > 0) {
        queries.push(`queues=${filter!.queues}`);
    }
    if (queries.length > 0) {
        matchUrl += `?${queries.join("&")}`;
    }
    const response = await fetch(backendUrl(matchUrl), headers);
    return JSON.parse(await response.json());
}
