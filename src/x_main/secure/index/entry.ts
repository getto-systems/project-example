import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../x_outside_feature/common"

import { newDashboardView } from "../../../example/action_dashboard/init/resource"

import { DashboardEntry } from "../../../example/action_dashboard/x_preact/dashboard"

render(h(DashboardEntry, newDashboardView(newForegroundOutsideFeature())), document.body)
