import { newClock } from "../../../../../ui/vendor/getto-application/infra/clock/init"
import { newSeasonRepositoryPod } from "./infra/repository/season"

import { RepositoryOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/repository/feature"

import { LoadSeasonInfra } from "./infra"

type OutsideFeature = RepositoryOutsideFeature
export function newLoadSeasonInfra(feature: OutsideFeature): LoadSeasonInfra {
    return {
        season: newSeasonRepositoryPod(feature),
        clock: newClock(),
    }
}
