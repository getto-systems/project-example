import { BaseOutsideFeature, newBaseResource } from "../../action_base/init/resource"

import { DashboardView } from "../resource"
import { initDashboardView } from "../init"

export function newDashboardView(feature: BaseOutsideFeature): DashboardView {
    return initDashboardView(newBaseResource(feature))
}
