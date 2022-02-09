import { h, render } from "preact"

import { SetupSeasonPageEntry } from "./page"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"

import { newBaseResource } from "../../../../../core/action_base/init/resource"
import { newSetupSeasonResource } from "../../../../../core/season/setup/init/resource"
import { initBaseView } from "../../../../../core/action_base/init"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { SetupSeasonPageResource } from "./resource"

render(h(SetupSeasonPageEntry, props()), document.body)

function props(): ApplicationView<SetupSeasonPageResource> {
    const resource = newResource()
    return initBaseView(resource, () => {
        resource.setupSeason.terminate()
    })
}
function newResource(): SetupSeasonPageResource {
    const feature = newForegroundOutsideFeature()
    const baseResource = newBaseResource(feature)
    return { ...baseResource, ...newSetupSeasonResource(feature, baseResource.season) }
}