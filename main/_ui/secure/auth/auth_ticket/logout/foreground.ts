import { h, render } from "preact"

import { newForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/_ui/common"
import { newBaseResource } from "../../../../../../src/example/_ui/action_base/init/resource"
import { newLogoutResource } from "../../../../../../src/auth/auth_ticket/_ui/action_logout/init/resource"
import { initBaseView } from "../../../../../../src/example/_ui/action_base/init"

import { LogoutPageEntry } from "./entry"

import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"
import { LogoutPageResource } from "./resource"

render(h(LogoutPageEntry, initView()), document.body)

function initView(): ApplicationView<LogoutPageResource> {
    const feature = newForegroundOutsideFeature()
    const resource = {
        ...newBaseResource(feature),
        ...newLogoutResource(feature),
    }
    return initBaseView(resource, () => {
        resource.logout.terminate()
    })
}
