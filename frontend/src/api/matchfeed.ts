import { MatchFeedElement, MatchFeedFilter } from "../models/matchfeed";
import { backendUrl } from "../utils";
import { getHeader } from "./utils";

export async function fetchConnections(): Promise<string[]> {
    const headers = await getHeader();
    const response = await fetch(backendUrl("/api/connections/"), headers);

    const connectionData: [string, number][] = JSON.parse(await response.json());
    return connectionData.map((c: [string, number]) => c[0]);
}

export async function fetchMatchFeed(filter?: MatchFeedFilter): Promise<MatchFeedElement[]> {
    const headers = await getHeader();
    let matchUrl = "/api/matches/";
    const queries: string[] = [];
    if (filter?.names) {
        queries.push(`names=${filter.names}`);
    }
    if (filter?.start) {
        queries.push(`start=${filter.start}`);
    }
    if (queries.length > 0) {
        matchUrl += `?${queries.join("&")}`;
    }
    const response = await fetch(backendUrl(matchUrl), headers);
    return JSON.parse(await response.json());
}
