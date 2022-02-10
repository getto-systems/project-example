import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../x_outside_feature/common"

import { newDashboardView } from "../../../core/dashboard/init/resource"

import { DashboardEntry } from "../../../core/dashboard/x_preact/dashboard"

render(h(DashboardEntry, newDashboardView(newForegroundOutsideFeature())), document.body)
