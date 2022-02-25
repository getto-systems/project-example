import {
    BoardValue,
    zeroBoardValue,
} from "../../../../z_vendor/getto-application/board/kernel/data"
import { readSearchParams } from "../kernel/convert"

const SEARCH_OFFSET = "search-offset"

export function readSearchOffset(params: URLSearchParams): BoardValue {
    const result = readSearchParams(params, SEARCH_OFFSET)
    if (!result.found) {
        return zeroBoardValue
    }
    return result.value
}

export function updateSearchOffset(currentURL: URL, offset: BoardValue): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    params.set(SEARCH_OFFSET, offset)
    return url
}
