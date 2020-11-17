export function decimalRound(num: number, places: number): number {
    const multiplier = 10 * places;
    return Math.round(num * multiplier) / multiplier;
}

export function backendUrl(path: string): string {
    return `${process.env.VUE_APP_BACKEND_LOCATION}${path}`;
}
