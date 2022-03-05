import { BaseResource } from "../../../../../core/base/resource"
import { SearchAuthUserAccountAction } from "../../../../../auth/user/account/search/action"
import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { OverridePasswordAction } from "../../../../../auth/user/password/change/action"

export type ManageUserAccountPageResource = BaseResource &
    Readonly<{
        search: SearchAuthUserAccountAction
        override: Readonly<{
            editable: EditableBoardAction
            override: OverridePasswordAction
        }>
    }>
