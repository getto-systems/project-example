import { readSearchParams } from "../../../../../ui/vendor/getto-application/board/kernel/convert"
import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"

const SEARCH_OFFSET = "search-offset"

export function readSearchOffset(params: URLSearchParams): BoardValue {
    return readSearchParams(params, { name: SEARCH_OFFSET, default: "0" })
}
export function updateSearchOffsetQuery(
    currentURL: URL,
    offset: BoardValue,
): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    params.set(SEARCH_OFFSET, offset)
    return url
}
