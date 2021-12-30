import { newForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/common"
import { newBaseResource } from "../../../../../../src/example/action_base/init/resource"
import { newLogoutInfra } from "../../../../../../src/auth/ticket/logout/init/infra"
import { initBaseView } from "../../../../../../src/example/action_base/init"

import { BaseResource } from "../../../../../../src/example/action_base/resource"

import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"
import { initLogoutAction, LogoutAction } from "../../../../../../src/auth/ticket/logout/action"

export type LogoutPageResource = BaseResource & Readonly<{ logout: LogoutAction }>

export function newLogoutPageView(): ApplicationView<LogoutPageResource> {
    const resource = newResource()
    return initBaseView(resource, () => {
        resource.logout.terminate()
    })
}
function newResource() {
    const feature = newForegroundOutsideFeature()
    return {
        ...newBaseResource(feature),
        logout: initLogoutAction(newLogoutInfra(feature)),
    }
}
