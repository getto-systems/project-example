import { readSearchParams } from "../../../../ui/vendor/getto-application/board/kernel/convert"
import {
    BoardValue,
    emptyBoardValue,
} from "../../../../ui/vendor/getto-application/board/kernel/data"
import { normalSearchSortOrder, SearchSort } from "./sort/data"

const SEARCH_OFFSET = "search-offset"
const SEARCH_SORT_KEY = "search-sort-key"
const SEARCH_SORT_ORDER = "search-sort-order"

export function readSearchOffset(params: URLSearchParams): BoardValue {
    return readSearchParams(params, { name: SEARCH_OFFSET, default: "0" })
}
export function readSearchSort(params: URLSearchParams, defaultSort: SearchSort): SearchSort {
    const key = readSearchParams(params, { name: SEARCH_SORT_KEY })
    if (key === emptyBoardValue) {
        return defaultSort
    }

    const order = readSearchParams(params, { name: SEARCH_SORT_ORDER })
    if (order === normalSearchSortOrder) {
        return { key, order: "normal" }
    } else {
        return { key, order: "reverse" }
    }
}
export function updateSearchQuery(currentURL: URL, offset: BoardValue, sort: SearchSort): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    params.set(SEARCH_OFFSET, offset)
    params.set(SEARCH_SORT_KEY, sort.key)
    params.set(SEARCH_SORT_ORDER, sort.order)
    return url
}
