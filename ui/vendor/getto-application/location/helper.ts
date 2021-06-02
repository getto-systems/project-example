import { LocationDetecter } from "./infra"

import { ConvertLocationResult } from "./data"

export function backgroundLocationDetecter<T>(
    result: ConvertLocationResult<T>,
): LocationDetecter<T> {
    return () => result
}

export type LocationSearchParam = [string, string]
export function encodeLocationSearchQuery(params: LocationSearchParam[]): string {
    return params
        .map((param) => {
            const [key, value] = param
            return `${encodeURIComponent(key)}=${encodeURIComponent(value)}`
        })
        .join("&")
}
