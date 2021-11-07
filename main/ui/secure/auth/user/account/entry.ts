import { h, render } from "preact"

import { ManageUserAccountPageEntry } from "./page"

import { newForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/common"

import { newBaseResource } from "../../../../../../src/example/action_base/init/resource"
import { newManageUserAccountResource } from "../../../../../../src/auth/user/account/action_manage/init/resource"
import { toManageUserAccountView } from "./common"

import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"
import { ManageUserAccountPageResource } from "./resource"

render(h(ManageUserAccountPageEntry, props()), document.body)

function props(): ApplicationView<ManageUserAccountPageResource> {
    return toManageUserAccountView({ resource: newResource() })
}
function newResource() {
    const feature = newForegroundOutsideFeature()
    return {
        ...newBaseResource(feature),
        ...newManageUserAccountResource(feature),
    }
}
