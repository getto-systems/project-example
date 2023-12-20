import { readSearchParams } from "../kernel/convert"
import { ReadSearchSortKeyResult, SearchSort } from "./data"

const SEARCH_SORT_KEY = "search-sort-key"
const SEARCH_SORT_ORDER = "search-sort-order"

export interface ReadSearchSortKey<K> {
    (value: string): ReadSearchSortKeyResult<K>
}

export function readSearchSort<K>(
    params: URLSearchParams,
    defaultSortKey: K,
    readSortKey: ReadSearchSortKey<K>,
): SearchSort<K> {
    const key = readSearchParams(params, SEARCH_SORT_KEY)
    const order = readSearchParams(params, SEARCH_SORT_ORDER)
    if (!key.found || !order.found) {
        return { key: defaultSortKey, order: "normal" }
    }
    return parseSearchSort({ key: key.value, order: order.value }, defaultSortKey, readSortKey)
}
export function parseSearchSort<K>(
    data: Readonly<{ key?: string | null; order?: string | null }> | null | undefined,
    defaultSortKey: K,
    readSortKey: ReadSearchSortKey<K>,
): SearchSort<K> {
    const key = readSortKey(data?.key || "")
    if (!key.found) {
        return { key: defaultSortKey, order: "normal" }
    }

    switch (data?.order) {
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
