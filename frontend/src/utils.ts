export function decimalRound(num: number, places: number): number {
    const multiplier = 10 * places;
    return Math.round(num * multiplier) / multiplier;
}

export function compareVersions(a: string, b: string): number {
    const splitA = a.split(".");
    const splitB = b.split(".");
    for (let i = 0; i < Math.min(splitA.length, splitB.length); i++) {
        const numA = Number(splitA[i]);
        const numB = Number(splitB[i]);
        if (numA > numB) return 1;
        if (numB > numA) return -1;
    }
    return 0;
}
