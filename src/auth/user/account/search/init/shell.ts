import {
    HistoryOutsideFeature,
    LocationOutsideFeature,
} from "../../../../../z_lib/ui/location/feature"

import {
    detectSearchAuthUserAccountFilter,
    updateSearchAuthUserAccountFilterQuery,
} from "../convert"

import { SearchAuthUserAccountShell } from "../action"

type OutsideFeature = LocationOutsideFeature & HistoryOutsideFeature
export function newSearchAuthUserAccountShell(feature: OutsideFeature): SearchAuthUserAccountShell {
    return {
        detectFields: () =>
            detectSearchAuthUserAccountFilter(new URL(feature.currentLocation.toString())),

        updateQuery: (fields) => {
            const url = updateSearchAuthUserAccountFilterQuery(
                new URL(feature.currentLocation.toString()),
                fields,
            )
            feature.currentHistory.pushState(null, "", url)
        },
    }
}
