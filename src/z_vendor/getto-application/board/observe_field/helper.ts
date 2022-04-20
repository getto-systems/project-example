export function isSameMultipleBoardValue(
    a: readonly string[],
    b: readonly string[],
): boolean {
    return a.length === b.length && a.every((value, i) => value === b.at(i))
}
