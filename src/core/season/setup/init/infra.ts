import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"

import { newClock } from "../../../../common/util/clock/init"
import { allSeasons } from "../../kernel/init/all_seasons"
import { newSeasonRepository } from "../../kernel/init/season_repository"

import { SetupSeasonInfra } from "../action"
import { currentSeason } from "../../kernel/init/current_season"

type OutsideFeature = RepositoryOutsideFeature
export function newSetupSeasonInfra(feature: OutsideFeature): SetupSeasonInfra {
    const defaultSeason = currentSeason(newClock())
    const availableSeasons = allSeasons(defaultSeason)
    return {
        availableSeasons,
        seasonRepository: newSeasonRepository(feature, availableSeasons),
        clock: newClock(),
    }
}
