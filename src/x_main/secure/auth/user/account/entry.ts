import { h, render } from "preact"

import { ManageUserAccountPageEntry } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../../../core/base/init/resource"
import { newSearchAuthUserAccountAction } from "../../../../../auth/user/account/search/init/resource"
import { toManageUserAccountView } from "./common"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { ManageUserAccountPageResource } from "./resource"
import { initEditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { newOverridePasswordAction } from "../../../../../auth/user/password/change/init/resource"

render(h(ManageUserAccountPageEntry, props()), document.body)

function props(): ApplicationView<ManageUserAccountPageResource> {
    return toManageUserAccountView({ resource: newResource() })
}
function newResource() {
    const feature = newForegroundOutsideFeature()
    return {
        ...newBaseResource(feature),
        search: newSearchAuthUserAccountAction(feature),
        override: {
            editable: initEditableBoardAction(),
            override: newOverridePasswordAction(feature),
        }
    }
}
