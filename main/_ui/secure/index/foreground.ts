import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../src/x_outside_feature/_ui/common"

import { newDashboardView } from "../../../../src/example/action_dashboard/init/resource"

import { DashboardEntry } from "../../../../src/example/action_dashboard/x_preact/dashboard"

render(h(DashboardEntry, newDashboardView(newForegroundOutsideFeature())), document.body)
