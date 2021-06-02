import { initBaseView } from "../action_base/impl"

import { DashboardView, DashboardResource } from "./resource"

export function initDashboardView(resource: DashboardResource): DashboardView {
    return initBaseView(resource, () => null)
}
