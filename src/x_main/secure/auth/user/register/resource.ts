import { BaseResource } from "../../../base/resource"
import { RegisterAuthUserAccountAction } from "../../../../../auth/user/account/register/action"
import { ToggleSidebarAction } from "../../../../../z_lib/ui/sidebar/action"
import { DetailAuthUserAccountActions } from "../../../../../auth/user/account/kernel/x_preact/detail"
import { RegisteredAuthUserAccountTableStructure } from "../../../../../auth/user/account/register/x_preact/structure"

export type RegisterUserAccountPageResource = BaseResource &
    DetailAuthUserAccountActions &
    Readonly<{
        sidebar: ToggleSidebarAction
        register: RegisterAuthUserAccountAction
        structure: RegisteredAuthUserAccountTableStructure
    }>
