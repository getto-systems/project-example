import { LocationDetecter, LocationDetectMethod } from "./infra"

export function mockLocationDetecter<T>(
    currentURL: URL,
    method: LocationDetectMethod<T>,
): LocationDetecter<T> {
    return () => method(currentURL)
}
