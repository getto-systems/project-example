import { BaseResource } from "../../../../../core/base/resource"
import { RegisterAuthUserAccountAction } from "../../../../../auth/user/account/register/action"
import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { OverrideLoginIdAction } from "../../../../../auth/user/login_id/change/action"
import { OverridePasswordAction } from "../../../../../auth/user/password/change/action"
import { SearchSidebarAction } from "../../../../../z_lib/ui/search/sidebar/action"
import { ModifyAuthUserAccountAction } from "../../../../../auth/user/account/modify/action"
import { ChangeResetTokenDestinationAction } from "../../../../../auth/user/password/reset/token_destination/change/action"

export type RegisterUserAccountPageResource = BaseResource &
    Readonly<{
        sidebar: SearchSidebarAction
        register: RegisterAuthUserAccountAction
        modify: Readonly<{
            editable: EditableBoardAction
            modify: ModifyAuthUserAccountAction
        }>
        changeResetTokenDestination: Readonly<{
            editable: EditableBoardAction
            change: ChangeResetTokenDestinationAction
        }>
        overrideLoginId: Readonly<{
            editable: EditableBoardAction
            override: OverrideLoginIdAction
        }>
        overridePassword: Readonly<{
            editable: EditableBoardAction
            override: OverridePasswordAction
        }>
    }>