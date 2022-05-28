import { h, render } from "preact"

import { ManageUserAccountPage } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../../../common/base/init/resource"
import { newRegisterAuthUserAccountAction } from "../../../../../auth/user/account/register/init/resource"
import { initEditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { newOverwritePasswordAction } from "../../../../../auth/user/password/change/init/resource"
import { newSearchSidebarAction } from "../../../../../z_lib/ui/search/sidebar/init/resource"
import { newOverwriteLoginIdAction } from "../../../../../auth/user/login_id/change/init/resource"
import { newModifyAuthUserAccountAction } from "../../../../../auth/user/account/modify/init/resource"
import { newChangeResetTokenDestinationAction } from "../../../../../auth/user/password/reset/token_destination/change/init/resource"
import { newUnregisterAuthUserAccountAction } from "../../../../../auth/user/account/unregister/init/resource"

import { RegisterUserAccountPageResource } from "./resource"

render(h(ManageUserAccountPage, props()), document.body)

function props(): RegisterUserAccountPageResource {
    const feature = newForegroundOutsideFeature()
    const register = newRegisterAuthUserAccountAction(feature)
    return {
        ...newBaseResource(feature),
        sidebar: newSearchSidebarAction(feature, "auth.user.register"),
        register,
        modify: {
            editable: initEditableBoardAction(),
            modify: newModifyAuthUserAccountAction(feature),
        },
        changeResetTokenDestination: {
            editable: initEditableBoardAction(),
            change: newChangeResetTokenDestinationAction(feature),
        },
        overwriteLoginId: {
            editable: initEditableBoardAction(),
            overwrite: newOverwriteLoginIdAction(feature),
        },
        overwritePassword: {
            editable: initEditableBoardAction(),
            overwrite: newOverwritePasswordAction(feature),
        },
        unregister: {
            editable: initEditableBoardAction(),
            unregister: newUnregisterAuthUserAccountAction(feature),
        },
    }
}
