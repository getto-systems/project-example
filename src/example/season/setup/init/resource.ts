import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { newSetupSeasonInfra } from "./infra"
import { newSetupSeasonConfig } from "./config"

import { SetupSeasonAction, initSetupSeasonAction } from "../action"
import { LoadSeasonAction } from "../../load/action"

type OutsideFeature = RepositoryOutsideFeature
export function newSetupSeasonResource(
    feature: OutsideFeature,
    season: LoadSeasonAction,
): Readonly<{ season: LoadSeasonAction; setupSeason: SetupSeasonAction }> {
    return {
        season,
        setupSeason: initSetupSeasonAction(
            {
                infra: newSetupSeasonInfra(feature),
                config: newSetupSeasonConfig(),
            },
            season,
        ),
    }
}
