import { initFocusSeasonAction } from "../init"
import { newFocusSeasonInfra } from "../../focus/init"

import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { FocusSeasonResource } from "../resource"
import { LoadSeasonAction } from "../../action_load/action"

type OutsideFeature = RepositoryOutsideFeature
export function newFocusSeasonResource(
    feature: OutsideFeature,
    season: LoadSeasonAction,
): FocusSeasonResource {
    return {
        season,
        focusSeason: initFocusSeasonAction(
            { focusSeason: newFocusSeasonInfra(feature) },
            season.ignitionState,
        ),
    }
}
