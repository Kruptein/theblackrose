export function decimalRound(num: number, places: number): number {
    const multiplier = 10 * places;
    return Math.round(num * multiplier) / multiplier;
}
