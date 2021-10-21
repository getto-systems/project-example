import { BoardValue, emptyBoardValue } from "./data"

export function readBoardValue(input: HTMLInputElement): BoardValue {
    return markBoardValue(input.value)
}
export function readSearchParams(query: URLSearchParams, name: string): BoardValue {
    const value = query.get(name)
    if (value === null) {
        return emptyBoardValue
    }
    return markBoardValue(value)
}

function markBoardValue(input: string): BoardValue {
    return input as BoardValue
}
