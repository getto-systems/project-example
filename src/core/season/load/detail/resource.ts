import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"

import { newSeasonRepository } from "../../kernel/detail/season_repository"
import { newClock } from "../../../../common/util/clock/detail"

import { LoadSeasonAction, initLoadSeasonAction } from "../action"

type OutsideFeature = RepositoryOutsideFeature
export function newLoadSeasonResource(
    feature: OutsideFeature,
): Readonly<{ season: LoadSeasonAction }> {
    const clock = newClock()
    const [seasonRepository, _availableSeasons] = newSeasonRepository(feature, clock)
    return {
        season: initLoadSeasonAction({
            seasonRepository,
            clock,
        }),
    }
}
