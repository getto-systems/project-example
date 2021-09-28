import { h, render } from "preact"

import { LogoutPageEntry } from "./page"

import { newForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/_ui/common"
import { newBaseResource } from "../../../../../../src/example/action_base/init/resource"
import { newLogoutResource } from "../../../../../../src/auth/ticket/action_logout/init/resource"
import { initBaseView } from "../../../../../../src/example/action_base/init"

import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"
import { LogoutPageResource } from "./resource"

render(h(LogoutPageEntry, props()), document.body)

function props(): ApplicationView<LogoutPageResource> {
    const resource = newResource()
    return initBaseView(resource, () => {
        resource.logout.terminate()
    })
}
function newResource() {
    const feature = newForegroundOutsideFeature()
    return {
        ...newBaseResource(feature),
        ...newLogoutResource(feature),
    }
}
