import type { DeepReadonly } from "@vue/reactivity";
import { readonly } from "@vue/reactivity";
import { reactive } from "vue";

import type { Connection } from "../models/connections";
import type { Summoner } from "../models/match";

class ConnectionStore {
    private connectionInfo: Map<string, Connection & Partial<Summoner>> = reactive(new Map());

    addConnections(...connections: (Connection & Partial<Summoner>)[]): void {
        for (const connection of connections) {
            if (!this.connectionInfo.has(connection.name)) {
                this.connectionInfo.set(connection.name, connection);
            } else {
                const state = this.connectionInfo.get(connection.name)!;
                Object.assign(state, connection);
            }
        }
    }

    getConnections(): DeepReadonly<Map<string, Connection & Partial<Summoner>>> {
        return readonly(this.connectionInfo);
    }

    getConnection(name: string): DeepReadonly<Connection & Partial<Summoner>> | undefined {
        const info = this.connectionInfo.get(name);
        if (info === undefined) return undefined;
        return readonly(info);
    }
}

export const connectionStore = new ConnectionStore();
(window as any).connectionStore = connectionStore;
