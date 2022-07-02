import { h, render } from "preact"

import { ManageUserAccountPage } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../base/init/resource"
import { newSearchAuthUserAccountAction } from "../../../../../auth/user/account/search/init/resource"

import { ManageUserAccountPageResource } from "./resource"
import { initEditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { newOverwritePasswordAction } from "../../../../../auth/user/password/change/init/resource"
import { newSearchSidebarAction } from "../../../../../z_lib/ui/search/sidebar/init/resource"
import { newOverwriteLoginIdAction } from "../../../../../auth/user/login_id/change/init/resource"
import { newModifyAuthUserAccountAction } from "../../../../../auth/user/account/modify/init/resource"
import { newChangeResetTokenDestinationAction } from "../../../../../auth/user/password/reset/token_destination/change/init/resource"
import { newUnregisterAuthUserAccountAction } from "../../../../../auth/user/account/unregister/init/resource"

render(h(ManageUserAccountPage, props()), document.body)

function props(): ManageUserAccountPageResource {
    const feature = newForegroundOutsideFeature()
    const search = newSearchAuthUserAccountAction(feature)
    return {
        ...newBaseResource(feature),
        sidebar: newSearchSidebarAction(feature, "auth.user.account"),
        search,
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
