import { BaseResource } from "../../../../../core/base/resource"
import {
    ListAuthUserAccountAction,
    SearchAuthUserAccountAction,
} from "../../../../../auth/user/account/search/action"
import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { OverrideLoginIdAction } from "../../../../../auth/user/login_id/change/action"
import { OverridePasswordAction } from "../../../../../auth/user/password/change/action"
import { SearchSidebarAction } from "../../../../../z_lib/ui/search/sidebar/action"

export type ManageUserAccountPageResource = BaseResource &
    Readonly<{
        sidebar: SearchSidebarAction,
        search: SearchAuthUserAccountAction
        list: ListAuthUserAccountAction
        overrideLoginId: Readonly<{
            editable: EditableBoardAction
            override: OverrideLoginIdAction
        }>
        overridePassword: Readonly<{
            editable: EditableBoardAction
            override: OverridePasswordAction
        }>
    }>
