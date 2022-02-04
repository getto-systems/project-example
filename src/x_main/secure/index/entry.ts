import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../x_outside_feature/common"

import { newDashboardView } from "../../../core/action_dashboard/init/resource"

import { DashboardEntry } from "../../../core/action_dashboard/x_preact/dashboard"

render(h(DashboardEntry, newDashboardView(newForegroundOutsideFeature())), document.body)
