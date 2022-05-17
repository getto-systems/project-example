import { BaseResource } from "../../../../../common/base/resource"
import { RegisterAuthUserAccountAction } from "../../../../../auth/user/account/register/action"
import { SearchSidebarAction } from "../../../../../z_lib/ui/search/sidebar/action"
import { DetailAuthUserAccountActions } from "../../../../../auth/user/account/kernel/x_preact/detail"

export type RegisterUserAccountPageResource = BaseResource &
    DetailAuthUserAccountActions &
    Readonly<{
        sidebar: SearchSidebarAction
        register: RegisterAuthUserAccountAction
    }>
