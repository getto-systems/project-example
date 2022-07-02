import { BaseResource } from "../../../base/resource"
import { SearchAuthUserAccountAction } from "../../../../../auth/user/account/search/action"
import { SearchSidebarAction } from "../../../../../z_lib/ui/search/sidebar/action"
import { DetailAuthUserAccountActions } from "../../../../../auth/user/account/kernel/x_preact/detail"

export type ManageUserAccountPageResource = BaseResource &
    DetailAuthUserAccountActions &
    Readonly<{
        sidebar: SearchSidebarAction
        search: SearchAuthUserAccountAction
    }>
