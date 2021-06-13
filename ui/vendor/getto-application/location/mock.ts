import { LocationDetecter, LocationDetectMethod } from "./infra"

export function mockDetecter<T>(
    currentURL: URL,
    method: LocationDetectMethod<T>,
): LocationDetecter<T> {
    return () => method(currentURL)
}
