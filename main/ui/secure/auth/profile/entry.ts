import { h, render } from "preact"

import { ProfilePageEntry } from "./page"

import { newForegroundOutsideFeature } from "../../../../../src/x_outside_feature/common"

import { newBaseResource } from "../../../../../src/example/action_base/init/resource"
import { newChangePasswordResource } from "../../../../../src/auth/user/password/action_change/init/resource"
import { newRequestResetTokenProfileResource } from "../../../../../src/auth/user/password/reset/action_request_token_profile/init/resource"
import { initBaseView } from "../../../../../src/example/action_base/init"

import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"
import { ProfilePageResource } from "./resource"

render(h(ProfilePageEntry, props()), document.body)

function props(): ApplicationView<ProfilePageResource> {
    const resource = newResource()
    return initBaseView(resource, () => {
        resource.change.terminate()
    })
}
function newResource() {
    const feature = newForegroundOutsideFeature()
    return {
        ...newBaseResource(feature),
        ...newChangePasswordResource(feature),
        ...newRequestResetTokenProfileResource(feature),
    }
}
