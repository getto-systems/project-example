import { BaseResource } from "../../../../../../src/example/action_base/resource"
import { SetupSeasonAction } from "../../../../../../src/example/season/setup/action"
import { LoadSeasonAction } from "../../../../../../src/example/season/load/action"

export type SetupSeasonPageResource = BaseResource &
    Readonly<{
        season: LoadSeasonAction
        setupSeason: SetupSeasonAction
    }>
