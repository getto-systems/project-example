import { h, render } from "preact"

import { ManageUserAccountPageEntry } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"

import { newBaseResource } from "../../../../../example/action_base/init/resource"
import { newManageUserAccountResource } from "../../../../../auth/user/account/manage/init/resource"
import { toManageUserAccountView } from "./common"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
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
