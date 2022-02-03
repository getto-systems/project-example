import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { newSeasonRepository } from "../../kernel/init/season_repository"
import { newClock } from "../../../../z_lib/ui/clock/init"

import { LoadSeasonAction, initLoadSeasonAction } from "../action"

type OutsideFeature = RepositoryOutsideFeature
export function newLoadSeasonResource(
    feature: OutsideFeature,
): Readonly<{ season: LoadSeasonAction }> {
    return {
        season: initLoadSeasonAction({
            season: newSeasonRepository(feature),
            clock: newClock(),
        }),
    }
}
