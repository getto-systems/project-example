import {
    HistoryOutsideFeature,
    LocationOutsideFeature,
} from "../../../../../z_lib/ui/location/feature"

import {
    clearFocusAuthUserAccountQuery,
    detectFocusAuthUserAccount,
    detectSearchAuthUserAccountFilter,
    updateFocusAuthUserAccountQuery,
    updateSearchAuthUserAccountFilterQuery,
} from "../convert"

import { SearchAuthUserAccountShell } from "../action"

type OutsideFeature = LocationOutsideFeature & HistoryOutsideFeature
export function newSearchAuthUserAccountShell(feature: OutsideFeature): SearchAuthUserAccountShell {
    return {
        detectFilter: () =>
            detectSearchAuthUserAccountFilter(new URL(feature.currentLocation.toString())),

        updateQuery: (fields) =>
            updateURL(feature, (url) => updateSearchAuthUserAccountFilterQuery(url, fields)),
        detectFocus: () => detectFocusAuthUserAccount(new URL(feature.currentLocation.toString())),
        updateFocus: {
            focus: (user) =>
                updateURL(feature, (url) => updateFocusAuthUserAccountQuery(url, user)),
            clear: () => updateURL(feature, (url) => clearFocusAuthUserAccountQuery(url)),
        },
    }
}

function updateURL(feature: OutsideFeature, updater: { (url: URL): URL }): void {
    feature.currentHistory.pushState(null, "", updater(new URL(feature.currentLocation.toString())))
}
