import { BaseOutsideFeature, newBaseResource } from "../action_base/init"

import { DashboardView } from "./resource"
import { initDashboardView } from "./impl"

export function newDashboardView(feature: BaseOutsideFeature): DashboardView {
    return initDashboardView(newBaseResource(feature))
}
