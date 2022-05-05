import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../x_outside_feature/common"
import { newBaseResource } from "../../../core/base/init/resource"
import { initBaseView } from "../../../core/base/init"

import { DashboardPage } from "./page"

import { ApplicationView } from "../../../z_vendor/getto-application/action/action"
import { BaseResource } from "../../../core/base/resource"

render(h(DashboardPage, props()), document.body)

function props(): ApplicationView<BaseResource> {
    return initBaseView(newResource(), () => null)
}
function newResource() {
    const feature = newForegroundOutsideFeature()
    return newBaseResource(feature)
}
