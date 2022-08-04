import { h, render } from "preact"

import { ManageUserAccountPage } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../base/init/resource"
import { newRegisterAuthUserAccountAction } from "../../../../../auth/user/account/register/init/resource"
import { newSearchSidebarAction } from "../../../../../z_lib/ui/search/sidebar/init/resource"
import { newDetailAuthUserAccountActions } from "../init"

import { RegisterUserAccountPageResource } from "./resource"

render(h(ManageUserAccountPage, props()), document.body)

function props(): RegisterUserAccountPageResource {
    const feature = newForegroundOutsideFeature()
    const register = newRegisterAuthUserAccountAction(feature)
    return {
        ...newBaseResource(feature),
        ...newDetailAuthUserAccountActions(feature, register.list.focus),
        sidebar: newSearchSidebarAction(feature, "auth.user.register"),
        register,
    }
}
