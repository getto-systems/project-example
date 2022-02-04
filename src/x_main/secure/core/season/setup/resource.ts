import { BaseResource } from "../../../../../example/action_base/resource"
import { SetupSeasonAction } from "../../../../../example/season/setup/action"
import { LoadSeasonAction } from "../../../../../example/season/load/action"

export type SetupSeasonPageResource = BaseResource &
    Readonly<{
        season: LoadSeasonAction
        setupSeason: SetupSeasonAction
    }>
