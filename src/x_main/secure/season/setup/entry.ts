import { h, render } from "preact"

import { SetupSeasonPage } from "./page"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"

import { newBaseResource } from "../../base/detail/resource"
import { newSetupSeasonResource } from "../../../../core/season/setup/detail/resource"

import { SetupSeasonPageResource } from "./resource"

render(h(SetupSeasonPage, props()), document.body)

function props(): SetupSeasonPageResource {
    const feature = newForegroundOutsideFeature()
    const baseResource = newBaseResource(feature)
    return {
        ...baseResource,
        ...newSetupSeasonResource(feature, baseResource.season),
    }
}
