import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"
import { newBaseResource } from "../../../../../common/base/init/resource"
import { newLogoutInfra } from "../../../../../auth/ticket/logout/init/infra"

import { BaseResource } from "../../../../../common/base/resource"

import { initLogoutAction, LogoutAction } from "../../../../../auth/ticket/logout/action"

export type LogoutPageResource = BaseResource & Readonly<{ logout: LogoutAction }>

export function newLogoutPageResource(): LogoutPageResource {
    const feature = newForegroundOutsideFeature()
    return {
        ...newBaseResource(feature),
        logout: initLogoutAction(newLogoutInfra(feature)),
    }
}
