import { RepositoryOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/repository/infra"
import { newLoadSeasonInfra } from "../load_season/impl/init"
import { initLoadSeasonCoreAction } from "./core/impl"
import { LoadSeasonResource } from "./resource"

type OutsideFeature = RepositoryOutsideFeature
export function newLoadSeasonResource(feature: OutsideFeature): LoadSeasonResource {
    return {
        season: initLoadSeasonCoreAction(newLoadSeasonInfra(feature)),
    }
}
