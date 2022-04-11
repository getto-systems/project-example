import { BaseOutsideFeature, newBaseResource } from "../../base/init/resource"
import { initDashboardView } from "../init"

import { DashboardResource } from "../resource"
import { ApplicationView } from "../../../z_vendor/getto-application/action/action"

export function newDashboardView(feature: BaseOutsideFeature): ApplicationView<DashboardResource> {
    return initDashboardView(newBaseResource(feature))
}
