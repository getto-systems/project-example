import { newClock } from "../../../../z_lib/ui/clock/init"
import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"
import { newSeasonRepository } from "../../kernel/init/season_repository"
import { FocusSeasonInfra } from "../action"

type OutsideFeature = RepositoryOutsideFeature
export function newFocusSeasonInfra(feature: OutsideFeature): FocusSeasonInfra {
    return {
        seasonRepository: newSeasonRepository(feature),
        clock: newClock(),
    }
}
