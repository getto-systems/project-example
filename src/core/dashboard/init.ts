import { ApplicationView } from "../../z_vendor/getto-application/action/action"
import { initBaseView } from "../base/init"
import { DashboardResource } from "./resource"

export function initDashboardView(resource: DashboardResource): ApplicationView<DashboardResource> {
    return initBaseView(resource, () => null)
}
