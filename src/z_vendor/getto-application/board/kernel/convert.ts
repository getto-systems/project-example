import { BoardValue } from "./data"

export function readBoardValue(input: HTMLInputElement | HTMLSelectElement): BoardValue {
    return markBoardValue(input.value)
}

export function markBoardValue(input: string): BoardValue {
    return input as BoardValue
}
