import { newClock } from "../../../z_lib/ui/clock/init"
import { newSeasonRepository } from "./season_repository"

import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { LoadSeasonInfra } from "../action"

type OutsideFeature = RepositoryOutsideFeature
export function newLoadSeasonInfra(feature: OutsideFeature): LoadSeasonInfra {
    return {
        seasonRepository: newSeasonRepository(feature),
        clock: newClock(),
    }
}
