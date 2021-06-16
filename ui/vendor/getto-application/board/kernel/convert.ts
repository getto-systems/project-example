import { BoardValue } from "./data"

export function readBoardValue(input: HTMLInputElement): BoardValue {
    return markBoardValue(input.value)
}

function markBoardValue(input: string): BoardValue {
    return input as BoardValue
}
