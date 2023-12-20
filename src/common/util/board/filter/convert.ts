import { mapSelectFilterBoardFilter } from "./action"

import { MultipleFilterBoardFilter, SelectFilterBoardFilter, SingleFilterBoardValue } from "./data"

export function readSingleFilterBoardValue(
    params: URLSearchParams,
    name: string,
): SingleFilterBoardValue {
    const value = params.get(name)
    if (value === null) {
        return []
    }
    return [value]
}

export function readSelectFilterBoardFilter<T>(
    params: URLSearchParams,
    name: string,
    convert: (value: string) => T,
): SelectFilterBoardFilter<T> {
    return mapSelectFilterBoardFilter(readSingleFilterBoardValue(params, name), convert)
}

export function readMultipleFilterBoardFilter<T>(
    params: URLSearchParams,
    name: string,
    convert: (value: string) => T,
): MultipleFilterBoardFilter<T> {
    return params.getAll(name).map(convert)
}

export function updateSingleFilterBoardValue(
    currentURL: URL,
    key: string,
    filter: SingleFilterBoardValue,
): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    if (filter.length === 0) {
        params.delete(key)
    } else {
        params.set(key, filter[0])
    }
    return url
}

export function updateSelectFilterBoardFilter<F>(
    currentURL: URL,
    key: string,
    filter: SelectFilterBoardFilter<F>,
    convert: (filter: F) => string,
): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    if (filter.length === 0) {
        params.delete(key)
    } else {
        params.set(key, convert(filter[0]))
    }
    return url
}

export function updateMultipleFilterBoardFilter<F>(
    currentURL: URL,
    key: string,
    filter: MultipleFilterBoardFilter<F>,
    convert: (filter: F) => string,
): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    params.delete(key)
    filter.forEach((value) => {
        params.append(key, convert(value))
    })
    return url
}
