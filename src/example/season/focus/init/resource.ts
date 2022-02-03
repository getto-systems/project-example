import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { newFocusSeasonInfra } from "./infra"
import { newFocusSeasonConfig } from "./config"

import { FocusSeasonAction, initFocusSeasonAction } from "../action"
import { LoadSeasonAction } from "../../load/action"

type OutsideFeature = RepositoryOutsideFeature
export function newFocusSeasonResource(
    feature: OutsideFeature,
    season: LoadSeasonAction,
): Readonly<{ season: LoadSeasonAction; focusSeason: FocusSeasonAction }> {
    return {
        season,
        focusSeason: initFocusSeasonAction(
            {
                infra: newFocusSeasonInfra(feature),
                config: newFocusSeasonConfig(),
            },
            season.ignitionState,
        ),
    }
}
