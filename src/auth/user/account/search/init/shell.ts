import {
    HistoryOutsideFeature,
    LocationOutsideFeature,
} from "../../../../../z_lib/ui/location/feature"

import {
    detectSearchAuthUserAccountFields,
    updateSearchAuthUserAccountFieldsQuery,
} from "../convert"

import { SearchAuthUserAccountShell } from "../action"

type OutsideFeature = LocationOutsideFeature & HistoryOutsideFeature
export function newSearchAuthUserAccountShell(feature: OutsideFeature): SearchAuthUserAccountShell {
    return {
        detectFields: (params) =>
            detectSearchAuthUserAccountFields(new URL(feature.currentLocation.toString()), params),

        updateQuery: (fields) => {
            const url = updateSearchAuthUserAccountFieldsQuery(
                new URL(feature.currentLocation.toString()),
                fields,
            )
            feature.currentHistory.pushState(null, "", url)
        },
    }
}
