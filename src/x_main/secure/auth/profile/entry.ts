import { h, render } from "preact"

import { ProfilePageEntry } from "./page"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"

import { newBaseResource } from "../../../../core/action_base/init/resource"
import { newChangePasswordResource } from "../../../../auth/user/password/change/init/resource"
import { newRequestResetTokenProfileResource } from "../../../../auth/user/password/reset/request_token/init/resource"
import { toProfileView } from "./common"

import { ApplicationView } from "../../../../z_vendor/getto-application/action/action"
import { ProfilePageResource } from "./resource"

render(h(ProfilePageEntry, props()), document.body)

function props(): ApplicationView<ProfilePageResource> {
    return toProfileView({ resource: newResource() })
}
function newResource() {
    const feature = newForegroundOutsideFeature()
    return {
        ...newBaseResource(feature),
        ...newChangePasswordResource(feature),
        ...newRequestResetTokenProfileResource(feature),
    }
}
