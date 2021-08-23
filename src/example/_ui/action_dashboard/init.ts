import { initBaseView } from "../action_base/init"

import { DashboardView, DashboardResource } from "./resource"

export function initDashboardView(resource: DashboardResource): DashboardView {
    return initBaseView(resource, () => null)
}
