import { RemoteOutsideFeature } from "../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../z_details/_ui/repository/feature"
import { LocationOutsideFeature } from "../../z_details/_ui/location/feature"

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
