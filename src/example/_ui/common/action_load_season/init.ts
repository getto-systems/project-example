import { newLoadSeasonInfra } from "../load_season/init"
import { initLoadSeasonCoreAction } from "./core/impl"

import { RepositoryOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/repository/feature"

import { LoadSeasonResource } from "./resource"

type OutsideFeature = RepositoryOutsideFeature
export function newLoadSeasonResource(feature: OutsideFeature): LoadSeasonResource {
    return {
        season: initLoadSeasonCoreAction(newLoadSeasonInfra(feature)),
    }
}
