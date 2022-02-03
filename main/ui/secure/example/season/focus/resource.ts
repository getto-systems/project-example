import { BaseResource } from "../../../../../../src/example/action_base/resource"
import { FocusSeasonAction } from "../../../../../../src/example/season/focus/action"
import { LoadSeasonAction } from "../../../../../../src/example/season/load/action"

export type FocusSeasonPageResource = BaseResource &
    Readonly<{
        season: LoadSeasonAction
        focusSeason: FocusSeasonAction
    }>
