import { BaseResource } from "../../../../../../src/example/action_base/resource"
import { SearchAuthUserAccountAction } from "../../../../../../src/auth/user/account/search/action"

export type ManageUserAccountPageResource = BaseResource &
    Readonly<{
        search: SearchAuthUserAccountAction
    }>
