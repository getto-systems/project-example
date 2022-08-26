import { BaseResource } from "../../../base/resource"
import { SearchAuthUserAccountAction } from "../../../../../auth/user/account/search/action"
import { ToggleSidebarAction } from "../../../../../z_lib/ui/sidebar/action"
import { DetailAuthUserAccountActions } from "../../../../../auth/user/account/kernel/x_preact/detail"
import { SearchColumnsAction } from "../../../../../z_lib/ui/search/columns/action"
import { SearchAuthUserAccountTableStructure } from "../../../../../auth/user/account/search/x_preact/structure"

export type ManageUserAccountPageResource = BaseResource &
    DetailAuthUserAccountActions &
    Readonly<{
        sidebar: ToggleSidebarAction
        search: SearchAuthUserAccountAction
        columns: SearchColumnsAction
        structure: SearchAuthUserAccountTableStructure
    }>
