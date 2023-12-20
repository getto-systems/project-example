import {
    HistoryOutsideFeature,
    LocationOutsideFeature,
} from "../../../../../common/util/location/feature"

import {
    clearFocusAuthUserAccountQuery,
    detectFocusAuthUserAccount,
    detectSearchAuthUserAccountFilter,
    updateFocusAuthUserAccountQuery,
    updateSearchAuthUserAccountFilterQuery,
} from "./query"

import { SearchAuthUserAccountShell } from "../action"

import { DetectFocusListKeyResult } from "../../../../../common/util/list/data"

type OutsideFeature = LocationOutsideFeature & HistoryOutsideFeature
export function newSearchAuthUserAccountShell(feature: OutsideFeature): SearchAuthUserAccountShell {
    return {
        detectFilter: () =>
            detectSearchAuthUserAccountFilter(new URL(feature.currentLocation.toString())),

        updateQuery: (fields) =>
            updateURL(feature, (url) => updateSearchAuthUserAccountFilterQuery(url, fields)),
        focus: {
            detect: () => detectFocusAuthUserAccount(new URL(feature.currentLocation.toString())),
            update: (data: DetectFocusListKeyResult) => {
                if (data.found) {
                    updateURL(feature, (url) => updateFocusAuthUserAccountQuery(url, data.key))
                } else {
                    updateURL(feature, (url) => clearFocusAuthUserAccountQuery(url))
                }
            },
        },
    }
}

function updateURL(feature: OutsideFeature, updater: { (url: URL): URL }): void {
    feature.currentHistory.pushState(null, "", updater(new URL(feature.currentLocation.toString())))
}
