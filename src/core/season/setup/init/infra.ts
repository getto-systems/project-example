import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { newClock } from "../../../../z_lib/ui/clock/init"
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
