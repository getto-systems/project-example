import { newLoadSeasonInfra } from "./infra"

import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { initLoadSeasonAction, LoadSeasonAction } from "../action"

type OutsideFeature = RepositoryOutsideFeature
export function newLoadSeasonResource(
    feature: OutsideFeature,
): Readonly<{ season: LoadSeasonAction }> {
    return {
        season: initLoadSeasonAction(newLoadSeasonInfra(feature)),
    }
}
