import { h, render } from "preact"

import { ManageUserAccountPage } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../base/init/resource"
import { newSearchAuthUserAccountAction } from "../../../../../auth/user/account/search/init/resource"
import { newSearchSidebarAction } from "../../../../../z_lib/ui/search/sidebar/init/resource"
import { newDetailAuthUserAccountActions } from "../init"

import { ManageUserAccountPageResource } from "./resource"

render(h(ManageUserAccountPage, props()), document.body)

function props(): ManageUserAccountPageResource {
    const feature = newForegroundOutsideFeature()
    const search = newSearchAuthUserAccountAction(feature)
    return {
        ...newBaseResource(feature),
        ...newDetailAuthUserAccountActions(feature, search.list.focus),
        sidebar: newSearchSidebarAction(feature, "auth.user.account"),
        search,
    }
}
