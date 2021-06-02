import { LocationDetecter, LocationDetectMethod, LocationOutsideFeature } from "./infra"

export function newLocationDetecter<T>(
    feature: LocationOutsideFeature,
    method: LocationDetectMethod<T>,
): LocationDetecter<T> {
    const { currentLocation } = feature
    return () => method(new URL(currentLocation.toString()))
}
