import { h, render } from "preact"

import { FocusSeasonPageEntry } from "./page"

import { newForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/common"

import { newBaseResource } from "../../../../../../src/example/action_base/init/resource"
import { newFocusSeasonResource } from "../../../../../../src/example/season/focus/init/resource"
import { initBaseView } from "../../../../../../src/example/action_base/init"

import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"
import { FocusSeasonPageResource } from "./resource"

render(h(FocusSeasonPageEntry, props()), document.body)

function props(): ApplicationView<FocusSeasonPageResource> {
    const resource = newResource()
    return initBaseView(resource, () => {
        resource.focusSeason.terminate()
    })
}
function newResource(): FocusSeasonPageResource {
    const feature = newForegroundOutsideFeature()
    const baseResource = newBaseResource(feature)
    return { ...baseResource, ...newFocusSeasonResource(feature, baseResource.season) }
}
