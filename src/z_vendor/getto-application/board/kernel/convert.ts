import { BoardValue } from "./data"

export function readBoardValue(input: HTMLInputElement | HTMLSelectElement): BoardValue {
    return toBoardValue(input.value)
}

export function toBoardValue(input: string): BoardValue {
    return input as BoardValue
}
