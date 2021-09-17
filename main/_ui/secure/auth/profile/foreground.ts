import { h, render } from "preact"

import { ProfilePageEntry } from "./page"

import { newForegroundOutsideFeature } from "../../../../../src/x_outside_feature/_ui/common"

import { newBaseResource } from "../../../../../src/example/_ui/action_base/init/resource"
import { newChangePasswordResource } from "../../../../../src/auth/password/_ui/action_change/init/resource"
import { initBaseView } from "../../../../../src/example/_ui/action_base/init"

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
    }
}
