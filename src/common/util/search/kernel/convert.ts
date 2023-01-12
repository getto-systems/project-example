import { SelectResult } from "../../validate/data"
import { MultipleValueFilter, ReadSearchResult, SearchPageResponse, TextFilter } from "./data"

export function parseSearchPage(
    data:
        | Readonly<{ offset?: number | null; limit?: number | null; count?: number | null }>
        | null
        | undefined,
): SearchPageResponse {
    if (data === null || data === undefined) {
        return {
            offset: 0,
            limit: 0,
            count: 0,
        }
    }
    return {
        offset: data.offset || 0,
        limit: data.limit || 0,
        count: data.count || 0,
    }
}

export function readSearchParams(params: URLSearchParams, name: string): ReadSearchResult {
    const value = params.get(name)
    if (value === null) {
        return { found: false }
    }
    return { found: true, value }
}

export function readSearchTextFilter(params: URLSearchParams, name: string): TextFilter {
    const value = params.get(name)
    if (value === null) {
        return { filter: false }
    }
    return { filter: true, value }
}

export function readSearchSelectResult<T>(
    params: URLSearchParams,
    name: string,
    mapper: (value: string) => T,
): SelectResult<T> {
    const value = params.get(name)
    if (value === null) {
        return { isSelected: false }
    }
    return { isSelected: true, value: mapper(value) }
}

export function readSearchMultipleValueFilter(
    params: URLSearchParams,
    name: string,
): MultipleValueFilter {
    return params.getAll(name)
}

export function updateSearchTextFilter(currentURL: URL, key: string, filter: TextFilter): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    if (!filter.filter) {
        params.delete(key)
    } else {
        params.set(key, filter.value)
    }
    return url
}

export function updateSearchSelectResult<T>(
    currentURL: URL,
    key: string,
    filter: SelectResult<T>,
): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    if (!filter.isSelected) {
        params.delete(key)
    } else {
        params.set(key, `${filter.value}`)
    }
    return url
}

export function updateSearchMultipleValueFilter(
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
