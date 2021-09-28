import { newClock } from "../../z_details/_ui/clock/init"
import { newSeasonRepository } from "./init/repository/season"

import { RepositoryOutsideFeature } from "../../z_details/_ui/repository/feature"

import { LoadSeasonInfra } from "./infra"

type OutsideFeature = RepositoryOutsideFeature
export function newLoadSeasonInfra(feature: OutsideFeature): LoadSeasonInfra {
    return {
        season: newSeasonRepository(feature),
        clock: newClock(),
    }
}
