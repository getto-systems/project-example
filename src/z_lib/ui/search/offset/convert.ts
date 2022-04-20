import { readSearchParams } from "../kernel/convert"

const SEARCH_OFFSET = "search-offset"

export function readSearchOffset(params: URLSearchParams): string {
    const result = readSearchParams(params, SEARCH_OFFSET)
    if (!result.found) {
        return "0"
    }
    return result.value
}

export function updateSearchOffset(currentURL: URL, offset: string): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    params.set(SEARCH_OFFSET, offset)
    return url
}
