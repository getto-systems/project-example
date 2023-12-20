import { h, render } from "preact"

import { ManageUserAccountPage } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../base/detail/resource"
import { newSearchAuthUserAccountAction } from "../../../../../auth/user/account/search/detail/resource"
import { newToggleSidebarAction } from "../../../../../common/util/sidebar/detail/resource"
import { newDetailAuthUserAccountActions } from "../detail"
import { newSearchColumnsBoard } from "../../../../../common/util/search/columns/detail/resource"
import { initSearchAuthUserAccountTableStructure } from "../../../../../auth/user/account/search/x_preact/structure"

import { ManageUserAccountPageResource } from "./resource"

render(h(ManageUserAccountPage, props()), document.body)

function props(): ManageUserAccountPageResource {
    const key = "auth.user.account"
    const feature = newForegroundOutsideFeature()
    const [search, updater] = newSearchAuthUserAccountAction(feature)
    const structure = initSearchAuthUserAccountTableStructure(search)
    return {
        ...newBaseResource(feature),
        ...newDetailAuthUserAccountActions(search.focus.state, updater),
        sidebar: newToggleSidebarAction(feature, key),
        search,
        structure,
        columns: newSearchColumnsBoard(feature, key, structure.allCells()),
    }
}
