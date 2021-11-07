import { BoardValue, emptyBoardValue } from "./data"

export function readBoardValue(input: HTMLInputElement | HTMLSelectElement): BoardValue {
    return markBoardValue(input.value)
}

export type ReadSearchParamsProps =
    | Readonly<{ name: string }>
    | Readonly<{ name: string; default: string }>
export function readSearchParams(params: URLSearchParams, props: ReadSearchParamsProps): BoardValue {
    const value = params.get(props.name)
    if (value === null) {
        if ("default" in props) {
            return markBoardValue(props.default)
        }
        return emptyBoardValue
    }
    return markBoardValue(value)
}

function markBoardValue(input: string): BoardValue {
    return input as BoardValue
}
