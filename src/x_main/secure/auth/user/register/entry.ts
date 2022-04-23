import { h, render } from "preact"

import { ManageUserAccountPage } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../../../core/base/init/resource"
import { newRegisterAuthUserAccountAction } from "../../../../../auth/user/account/register/init/resource"
import { initEditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { newOverridePasswordAction } from "../../../../../auth/user/password/change/init/resource"
import { newSearchSidebarAction } from "../../../../../z_lib/ui/search/sidebar/init/resource"
import { newOverrideLoginIdAction } from "../../../../../auth/user/login_id/change/init/resource"
import { newModifyAuthUserAccountAction } from "../../../../../auth/user/account/modify/init/resource"
import { newChangeResetTokenDestinationAction } from "../../../../../auth/user/password/reset/token_destination/change/init/resource"
import { newUnregisterAuthUserAccountAction } from "../../../../../auth/user/account/unregister/init/resource"
import { toRegisterUserAccountView } from "./common"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { RegisterUserAccountPageResource } from "./resource"

render(h(ManageUserAccountPage, props()), document.body)

function props(): ApplicationView<RegisterUserAccountPageResource> {
    return toRegisterUserAccountView({ resource: newResource() })
}
function newResource() {
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
        overrideLoginId: {
            editable: initEditableBoardAction(),
            override: newOverrideLoginIdAction(feature),
        },
        overridePassword: {
            editable: initEditableBoardAction(),
            override: newOverridePasswordAction(feature),
        },
        unregister: {
            editable: initEditableBoardAction(),
            unregister: newUnregisterAuthUserAccountAction(feature),
        },
    }
}
