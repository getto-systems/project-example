import { newClock } from "../../../../z_lib/ui/clock/init"
import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"
import { newSeasonRepository } from "../../kernel/init/season_repository"
import { SetupSeasonInfra } from "../action"

type OutsideFeature = RepositoryOutsideFeature
export function newSetupSeasonInfra(feature: OutsideFeature): SetupSeasonInfra {
    return {
        seasonRepository: newSeasonRepository(feature),
        clock: newClock(),
    }
}
