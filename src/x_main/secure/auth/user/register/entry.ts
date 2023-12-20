import { h, render } from "preact"

import { RegisterUserAccountPage } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../base/detail/resource"
import { newRegisterAuthUserAccountAction } from "../../../../../auth/user/account/register/detail/resource"
import { newToggleSidebarAction } from "../../../../../common/util/sidebar/detail/resource"
import { newDetailAuthUserAccountActions } from "../detail"
import { initRegisteredAuthUserAccountTableStructure } from "../../../../../auth/user/account/register/x_preact/structure"

import { RegisterUserAccountPageResource } from "./resource"

render(h(RegisterUserAccountPage, props()), document.body)

function props(): RegisterUserAccountPageResource {
    const key = "auth.user.register"
    const feature = newForegroundOutsideFeature()
    const [register, updater] = newRegisterAuthUserAccountAction()
    return {
        ...newBaseResource(feature),
        ...newDetailAuthUserAccountActions(register.focus.state, updater),
        sidebar: newToggleSidebarAction(feature, key),
        register,
        structure: initRegisteredAuthUserAccountTableStructure(register),
    }
}
