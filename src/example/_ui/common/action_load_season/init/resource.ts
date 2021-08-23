import { newLoadSeasonInfra } from "../../load_season/init"
import { initLoadSeasonAction } from "../init"

import { RepositoryOutsideFeature } from "../../../../../z_details/_ui/repository/feature"

import { LoadSeasonResource } from "../resource"

type OutsideFeature = RepositoryOutsideFeature
export function newLoadSeasonResource(feature: OutsideFeature): LoadSeasonResource {
    return {
        season: initLoadSeasonAction(newLoadSeasonInfra(feature)),
    }
}
