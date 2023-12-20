import { BaseResource } from "../../../base/resource"
import { SearchAuthUserAccountAction } from "../../../../../auth/user/account/search/action"
import { ToggleSidebarAction } from "../../../../../common/util/sidebar/action"
import { DetailAuthUserAccountActions } from "../../../../../auth/user/account/kernel/x_preact/detail"
import { SearchColumnsBoard } from "../../../../../common/util/search/columns/action"
import { SearchAuthUserAccountTableStructure } from "../../../../../auth/user/account/search/x_preact/structure"

export type ManageUserAccountPageResource = BaseResource &
    DetailAuthUserAccountActions &
    Readonly<{
        sidebar: ToggleSidebarAction
        search: SearchAuthUserAccountAction
        columns: SearchColumnsBoard
        structure: SearchAuthUserAccountTableStructure
    }>
