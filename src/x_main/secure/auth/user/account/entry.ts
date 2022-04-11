import { h, render } from "preact"

import { ManageUserAccountPage } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../../../core/base/init/resource"
import { newSearchAuthUserAccountAction } from "../../../../../auth/user/account/search/init/resource"
import { toManageUserAccountView } from "./common"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { ManageUserAccountPageResource } from "./resource"
import { initEditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { newOverridePasswordAction } from "../../../../../auth/user/password/change/init/resource"
import { newSearchSidebarAction } from "../../../../../z_lib/ui/search/sidebar/init/resource"
import { newOverrideLoginIdAction } from "../../../../../auth/user/login_id/change/init/resource"
import { newModifyAuthUserAccountAction } from "../../../../../auth/user/account/modify/init/resource"
import { newChangeResetTokenDestinationAction } from "../../../../../auth/user/password/reset/token_destination/change/init/resource"

render(h(ManageUserAccountPage, props()), document.body)

function props(): ApplicationView<ManageUserAccountPageResource> {
    return toManageUserAccountView({ resource: newResource() })
}
function newResource() {
    const feature = newForegroundOutsideFeature()
    const search = newSearchAuthUserAccountAction(feature)
    return {
        ...newBaseResource(feature),
        sidebar: newSearchSidebarAction(feature, "auth.user.account"),
        search,
        list: search,
        detail: search.detail,
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
    }
}
