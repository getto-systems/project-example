import { BaseTypes } from "../action_base/resource"

type DashboardTypes = BaseTypes<EmptyResource>
type EmptyResource = {
    // no additional resources
}
export type DashboardView = DashboardTypes["view"]
export type DashboardResource = DashboardTypes["resource"]
