import { RemoteOutsideFeature } from "../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../z_lib/ui/repository/feature"
import { HistoryOutsideFeature, LocationOutsideFeature } from "../z_lib/ui/location/feature"

export type ForegroundOutsideFeature = CommonOutsideFeature &
    LocationOutsideFeature &
    HistoryOutsideFeature

export type CommonOutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature

export function newForegroundOutsideFeature(): ForegroundOutsideFeature {
    return {
        ...newCommonOutsideFeature(),
        currentLocation: location,
        currentHistory: history,
    }
}

export function newCommonOutsideFeature(): CommonOutsideFeature {
    return {
        webDB: indexedDB,
        webCrypto: crypto,
    }
}
