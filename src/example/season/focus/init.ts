import { EXAMPLE_CONFIG } from "../../x_outside_feature/config"

import { newClock } from "../../../z_lib/ui/clock/init"
import { newSeasonRepository } from "../kernel/init/season_repository"

import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { FocusSeasonInfra } from "./infra"

type OutsideFeature = RepositoryOutsideFeature
export function newFocusSeasonInfra(feature: OutsideFeature): FocusSeasonInfra {
    return {
        season: newSeasonRepository(feature),
        clock: newClock(),
        config: {
            focusSeasonExpire: EXAMPLE_CONFIG.focusSeasonExpire,
        }
    }
}
