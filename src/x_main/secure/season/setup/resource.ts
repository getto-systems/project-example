import { BaseResource } from "../../../../common/base/resource"
import { SetupSeasonAction } from "../../../../core/season/setup/action"
import { LoadSeasonAction } from "../../../../core/season/load/action"
import { EditableBoardAction } from "../../../../z_vendor/getto-application/board/editable/action"

export type SetupSeasonPageResource = BaseResource &
    Readonly<{
        season: LoadSeasonAction
        setup: Readonly<{
            season: SetupSeasonAction
            editable: EditableBoardAction
        }>
    }>
