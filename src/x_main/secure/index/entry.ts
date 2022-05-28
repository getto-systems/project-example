import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../x_outside_feature/common"
import { newBaseResource } from "../../../common/base/init/resource"

import { DashboardPage } from "./page"

import { BaseResource } from "../../../common/base/resource"

render(h(DashboardPage, props()), document.body)

function props(): BaseResource {
    const feature = newForegroundOutsideFeature()
    return newBaseResource(feature)
}
