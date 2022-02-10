import { initBaseView } from "../base/init"

import { DashboardView, DashboardResource } from "./resource"

export function initDashboardView(resource: DashboardResource): DashboardView {
    return initBaseView(resource, () => null)
}
