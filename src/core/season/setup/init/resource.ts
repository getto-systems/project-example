import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { newSetupSeasonInfra } from "./infra"
import { newSetupSeasonConfig } from "./config"

import { SetupSeasonAction, initSetupSeasonAction } from "../action"
import { LoadSeasonAction } from "../../load/action"

type OutsideFeature = RepositoryOutsideFeature
export function newSetupSeasonResource(
    feature: OutsideFeature,
    season: LoadSeasonAction,
): Readonly<{ season: LoadSeasonAction; setup: SetupSeasonAction }> {
    return {
        season,
        setup: initSetupSeasonAction(
            {
                infra: newSetupSeasonInfra(feature),
                config: newSetupSeasonConfig(),
            },
            season,
        ),
    }
}
