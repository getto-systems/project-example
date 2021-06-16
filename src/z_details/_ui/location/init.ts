import { LocationOutsideFeature } from "./feature"

export function toURL(feature: LocationOutsideFeature): URL {
    return new URL(feature.currentLocation.toString())
}
