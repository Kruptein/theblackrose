import { auth0 } from "../auth";
import type { MatchFeedFilter } from "../models/matchfeed";

export function backendUrl(path: string): string {
    return `${import.meta.env.VITE_BACKEND_LOCATION}${path}`;
}

(window as any).bu = async (url: string) => {
    const headers = await getAuthHeader();
    const response = await fetch(backendUrl(url), headers);
    return await response.json();
};

export async function getAuthHeader(): Promise<{ headers: { Authorization: string } }> {
    const token = await auth0.getAccessTokenSilently();
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
    return await response.json();
}
