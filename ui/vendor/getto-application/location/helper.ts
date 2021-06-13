export type LocationSearchParam = [string, string]
export function encodeLocationSearchQuery(params: LocationSearchParam[]): string {
    return params
        .map((param) => {
            const [key, value] = param
            return `${encodeURIComponent(key)}=${encodeURIComponent(value)}`
        })
        .join("&")
}
