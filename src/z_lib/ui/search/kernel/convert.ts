import { MultipleValueFilter, ReadSearchResult, SingleValueFilter } from "./data"

export function readSearchParams(params: URLSearchParams, name: string): ReadSearchResult {
    const value = params.get(name)
    if (value === null) {
        return { found: false }
    }
    return { found: true, value }
}

export function readSingleValueFilter(params: URLSearchParams, name: string): SingleValueFilter {
    const value = params.get(name)
    if (value === null) {
        return { filter: false }
    }
    return { filter: true, value }
}
export function readMultipleValueFilter(
    params: URLSearchParams,
    name: string,
): MultipleValueFilter {
    return params.getAll(name)
}

export function updateSingleValueFilter(
    currentURL: URL,
    key: string,
    filter: SingleValueFilter,
): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    if (!filter.filter) {
        params.delete(key)
    } else {
        params.set(key, filter.value)
    }
    return url
}

export function updateMultipleValueFilter(
    currentURL: URL,
    key: string,
    filter: MultipleValueFilter,
): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    params.delete(key)
    filter.forEach((value) => {
        params.append(key, value)
    })
    return url
}
