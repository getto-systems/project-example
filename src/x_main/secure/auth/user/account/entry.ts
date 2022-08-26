import { h, render } from "preact"

import { ManageUserAccountPage } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../base/init/resource"
import { newSearchAuthUserAccountAction } from "../../../../../auth/user/account/search/init/resource"
import { newToggleSidebarAction } from "../../../../../z_lib/ui/sidebar/init/resource"
import { newDetailAuthUserAccountActions } from "../init"
import { newSearchColumnsAction } from "../../../../../z_lib/ui/search/columns/init/resource"
import { initSearchAuthUserAccountTableStructure } from "../../../../../auth/user/account/search/x_preact/structure"

import { ManageUserAccountPageResource } from "./resource"

render(h(ManageUserAccountPage, props()), document.body)

function props(): ManageUserAccountPageResource {
    const key = "auth.user.account"
    const feature = newForegroundOutsideFeature()
    const search = newSearchAuthUserAccountAction(feature)
    const structure = initSearchAuthUserAccountTableStructure(search)
    return {
        ...newBaseResource(feature),
        ...newDetailAuthUserAccountActions(feature, search.list.focus),
        sidebar: newToggleSidebarAction(feature, key),
        search,
        structure,
        columns: newSearchColumnsAction(feature, key, structure.initialVisibleCells()),
    }
}
