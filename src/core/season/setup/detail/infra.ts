import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"

import { newClock } from "../../../../common/util/clock/detail"
import { newSeasonRepository } from "../../kernel/detail/season_repository"

import { SetupSeasonInfra } from "../action"

type OutsideFeature = RepositoryOutsideFeature
export function newSetupSeasonInfra(feature: OutsideFeature): SetupSeasonInfra {
    const clock = newClock()
    const [seasonRepository, availableSeasons] = newSeasonRepository(feature, clock)
    return {
        availableSeasons,
        seasonRepository,
        clock,
    }
}
