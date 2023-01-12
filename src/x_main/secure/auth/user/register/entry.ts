import { h, render } from "preact"

import { RegisterUserAccountPage } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../base/init/resource"
import { newRegisterAuthUserAccountAction } from "../../../../../auth/user/account/register/init/resource"
import { newToggleSidebarAction } from "../../../../../common/util/sidebar/init/resource"
import { newDetailAuthUserAccountActions } from "../init"
import { initRegisteredAuthUserAccountTableStructure } from "../../../../../auth/user/account/register/x_preact/structure"

import { RegisterUserAccountPageResource } from "./resource"

render(h(RegisterUserAccountPage, props()), document.body)

function props(): RegisterUserAccountPageResource {
    const key = "auth.user.register"
    const feature = newForegroundOutsideFeature()
    const register = newRegisterAuthUserAccountAction()
    return {
        ...newBaseResource(feature),
        ...newDetailAuthUserAccountActions(feature, register.list.focus),
        sidebar: newToggleSidebarAction(feature, key),
        register,
        structure: initRegisteredAuthUserAccountTableStructure(register),
    }
}
