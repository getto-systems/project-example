import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../../../common/base/init/resource"
import { newLogoutInfra } from "../../../../../auth/ticket/logout/init/infra"
import { initBaseView } from "../../../../../common/base/init"

import { BaseResource } from "../../../../../common/base/resource"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { initLogoutAction, LogoutAction } from "../../../../../auth/ticket/logout/action"

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
