import { RepositoryOutsideFeature } from "../common/util/repository/feature"
import { HistoryOutsideFeature, LocationOutsideFeature } from "../common/util/location/feature"

export type ForegroundOutsideFeature = CommonOutsideFeature &
    LocationOutsideFeature &
    HistoryOutsideFeature

export type CommonOutsideFeature = RepositoryOutsideFeature

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
    }
}
