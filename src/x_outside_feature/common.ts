import { RemoteOutsideFeature } from "../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../z_lib/ui/repository/feature"
import { LocationOutsideFeature } from "../z_lib/ui/location/feature"

export type ForegroundOutsideFeature = CommonOutsideFeature & LocationOutsideFeature

export type CommonOutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature

export function newForegroundOutsideFeature(): ForegroundOutsideFeature {
    return {
        ...newCommonOutsideFeature(),
        currentLocation: location,
    }
}

export function newCommonOutsideFeature(): CommonOutsideFeature {
    return {
        webDB: indexedDB,
        webCrypto: crypto,
    }
}
