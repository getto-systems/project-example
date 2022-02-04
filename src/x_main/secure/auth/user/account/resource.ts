import { BaseResource } from "../../../../../core/action_base/resource"
import { SearchAuthUserAccountAction } from "../../../../../auth/user/account/search/action"

export type ManageUserAccountPageResource = BaseResource &
    Readonly<{
        search: SearchAuthUserAccountAction
    }>
