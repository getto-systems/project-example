import { auth_config } from "../../../x_outside_feature/config"

import { newSearchUserAccountRemote } from "./init/remote"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import {
    SearchUserAccountFieldsDetecter,
    SearchUserAccountInfra,
    UpdateSearchUserAccountFieldsQuery,
} from "./infra"
import {
    HistoryOutsideFeature,
    LocationOutsideFeature,
} from "../../../../z_lib/ui/location/feature"
import { detectSearchUserAccountFields, updateSearchUserAccountFieldsQuery } from "./convert"

export function newSearchUserAccountInfra(feature: RemoteOutsideFeature): SearchUserAccountInfra {
    return {
        search: newSearchUserAccountRemote(feature),
        config: {
            takeLongtimeThreshold: auth_config.takeLongtimeThreshold,
        },
    }
}

export function newSearchUserAccountFieldsDetecter(
    feature: LocationOutsideFeature,
): SearchUserAccountFieldsDetecter {
    return (params) =>
        detectSearchUserAccountFields(new URL(feature.currentLocation.toString()), params)
}
export function newUpdateSearchUserAccountFieldsQuery(
    feature: LocationOutsideFeature & HistoryOutsideFeature,
): UpdateSearchUserAccountFieldsQuery {
    return (fields) => {
        const url = updateSearchUserAccountFieldsQuery(
            new URL(feature.currentLocation.toString()),
            fields,
        )
        feature.currentHistory.pushState(null, "", url)
    }
}
