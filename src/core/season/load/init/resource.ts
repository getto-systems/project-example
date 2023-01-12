import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"

import { newSeasonRepository } from "../../kernel/init/season_repository"
import { newClock } from "../../../../common/util/clock/init"

import { LoadSeasonAction, initLoadSeasonAction } from "../action"
import { allSeasons } from "../../kernel/init/all_seasons"
import { currentSeason } from "../../kernel/init/current_season"

type OutsideFeature = RepositoryOutsideFeature
export function newLoadSeasonResource(
    feature: OutsideFeature,
): Readonly<{ season: LoadSeasonAction }> {
    const defaultSeason = currentSeason(newClock())
    const availableSeasons = allSeasons(defaultSeason)
    return {
        season: initLoadSeasonAction({
            defaultSeason,
            availableSeasons,
            seasonRepository: newSeasonRepository(feature, availableSeasons),
            clock: newClock(),
        }),
    }
}
