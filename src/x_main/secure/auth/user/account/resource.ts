import { BaseResource } from "../../../../../core/base/resource"
import { SearchAuthUserAccountAction } from "../../../../../auth/user/account/search/action"

export type ManageUserAccountPageResource = BaseResource &
    Readonly<{
        search: SearchAuthUserAccountAction
    }>
