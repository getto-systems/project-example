import { BaseOutsideFeature, newBaseResource } from "../../base/init/resource"

import { DashboardView } from "../resource"
import { initDashboardView } from "../init"

export function newDashboardView(feature: BaseOutsideFeature): DashboardView {
    return initDashboardView(newBaseResource(feature))
}
