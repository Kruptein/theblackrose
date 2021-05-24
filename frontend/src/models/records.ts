export type Record = { id: number; recordType: number; value: number; name: string; queueId: number; gameId: number };

export enum RecordType {
    TotalMinionsKilled,
    Kills,
    Deaths,
    Assists,
    CsPerMinute,
    KDA,
}
