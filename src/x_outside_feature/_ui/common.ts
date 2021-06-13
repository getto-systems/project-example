import { RemoteOutsideFeature } from "../../../ui/vendor/getto-application/infra/remote/feature"
import { RepositoryOutsideFeature } from "../../../ui/vendor/getto-application/infra/repository/feature"
import { LocationOutsideFeature } from "../../../ui/vendor/getto-application/location/feature"

export type ForegroundOutsideFeature = CommonOutsideFeature & LocationOutsideFeature

export type CommonOutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature

export function foregroundOutsideFeature(): ForegroundOutsideFeature {
    return {
        ...commonOutsideFeature(),
        currentLocation: location,
    }
}

export function commonOutsideFeature(): CommonOutsideFeature {
    return {
        webDB: indexedDB,
        webCrypto: crypto,
    }
}
