import { Connection } from "../models/connections";
import { MatchFeedElement, MatchFeedFilter } from "../models/matchfeed";

import { backendUrl, getAuthHeader } from "./utils";

export async function fetchConnections(): Promise<string[]> {
    const headers = await getAuthHeader();
    const response = await fetch(backendUrl("/api/connections/"), headers);

    const connectionData: Connection[] = JSON.parse(await response.json());
    return connectionData.map((c: Connection) => c.name);
}

export async function fetchMatchFeed(filter?: MatchFeedFilter): Promise<MatchFeedElement[]> {
    const headers = await getAuthHeader();
    let matchUrl = "/api/matches/";
    const queries: string[] = [];
    if (filter?.names) {
        queries.push(`names=${filter.names}`);
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
    if (queries.length > 0) {
        matchUrl += `?${queries.join("&")}`;
    }
    const response = await fetch(backendUrl(matchUrl), headers);
    return JSON.parse(await response.json());
}
