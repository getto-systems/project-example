import { BoardValue } from "../kernel/data"

export function isSameMultipleBoardValue(
    a: readonly BoardValue[],
    b: readonly BoardValue[],
): boolean {
    return a.length === b.length && a.every((value, i) => value === b.at(i))
}
