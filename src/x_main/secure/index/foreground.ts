import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../x_outside_feature/_ui/common"

import { newDashboardView } from "../../../example/_ui/action_dashboard/init"

import { DashboardEntry } from "../../../example/_ui/action_dashboard/x_preact/dashboard"

render(h(DashboardEntry, newDashboardView(foregroundOutsideFeature())), document.body)
