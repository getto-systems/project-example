import { AUTH_CONFIG } from "../../../x_outside_feature/config"

import { newSearchAuthUserAccountRemote } from "./init/remote"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import {
    SearchAuthUserAccountFieldsDetecter,
    SearchAuthUserAccountInfra,
    UpdateSearchAuthUserAccountFieldsQuery,
} from "./infra"
import {
    HistoryOutsideFeature,
    LocationOutsideFeature,
} from "../../../../z_lib/ui/location/feature"
import { detectSearchAuthUserAccountFields, updateSearchAuthUserAccountFieldsQuery } from "./convert"

export function newSearchAuthUserAccountInfra(feature: RemoteOutsideFeature): SearchAuthUserAccountInfra {
    return {
        search: newSearchAuthUserAccountRemote(feature),
        config: {
            takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
        },
    }
}

export function newSearchAuthUserAccountFieldsDetecter(
    feature: LocationOutsideFeature,
): SearchAuthUserAccountFieldsDetecter {
    return (params) =>
        detectSearchAuthUserAccountFields(new URL(feature.currentLocation.toString()), params)
}
export function newUpdateSearchAuthUserAccountFieldsQuery(
    feature: LocationOutsideFeature & HistoryOutsideFeature,
): UpdateSearchAuthUserAccountFieldsQuery {
    return (fields) => {
        const url = updateSearchAuthUserAccountFieldsQuery(
            new URL(feature.currentLocation.toString()),
            fields,
        )
        feature.currentHistory.pushState(null, "", url)
    }
}
