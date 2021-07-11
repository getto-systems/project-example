import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../src/x_outside_feature/_ui/common"

import { newDashboardView } from "../../../../src/example/_ui/action_dashboard/init"

import { DashboardEntry } from "../../../../src/example/_ui/action_dashboard/x_preact/dashboard"

render(h(DashboardEntry, newDashboardView(foregroundOutsideFeature())), document.body)
