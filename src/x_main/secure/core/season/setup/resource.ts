import { BaseResource } from "../../../../../core/action_base/resource"
import { SetupSeasonAction } from "../../../../../core/season/setup/action"
import { LoadSeasonAction } from "../../../../../core/season/load/action"

export type SetupSeasonPageResource = BaseResource &
    Readonly<{
        season: LoadSeasonAction
        setupSeason: SetupSeasonAction
    }>
