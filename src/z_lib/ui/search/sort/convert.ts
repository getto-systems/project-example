import { BoardValue } from "../../../../z_vendor/getto-application/board/kernel/data"
import { readSearchParams } from "../kernel/convert"
import { ReadSearchSortKeyResult, SearchSort } from "./data"

const SEARCH_SORT_KEY = "search-sort-key"
const SEARCH_SORT_ORDER = "search-sort-order"

export interface ReadSearchSortKey<K> {
    (value: BoardValue): ReadSearchSortKeyResult<K>
}

export function readSearchSort<K>(
    params: URLSearchParams,
    defaultSortKey: K,
    readSortKey: ReadSearchSortKey<K>,
): SearchSort<K> {
    const rawKey = readSearchParams(params, SEARCH_SORT_KEY)
    if (!rawKey.found) {
        return { key: defaultSortKey, order: "normal" }
    }

    const key = readSortKey(rawKey.value)
    if (!key.found) {
        return { key: defaultSortKey, order: "normal" }
    }

    const order = readSearchParams(params, SEARCH_SORT_ORDER)
    if (!order.found) {
        return { key: key.key, order: "normal" }
    }

    switch (order.value) {
        case "normal":
            return { key: key.key, order: "normal" }

        case "reverse":
            return { key: key.key, order: "reverse" }

        default:
            return { key: key.key, order: "normal" }
    }
}

export function updateSearchSort<K extends string>(currentURL: URL, sort: SearchSort<K>): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    params.set(SEARCH_SORT_KEY, sort.key)
    params.set(SEARCH_SORT_ORDER, sort.order)
    return url
}
