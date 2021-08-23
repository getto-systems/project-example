import { toApplicationView } from "../../../../../../ui/vendor/getto-application/action/helper"

import { initFindNextVersionAction, initFindNextVersionMaterial } from "../init"
import { newFindNextVersionInfra, newFindNextVersionLocationDetecter } from "../../find_next/init"

import { LocationOutsideFeature } from "../../../../../z_details/_ui/location/feature"

import { FindNextVersionView } from "../resource"

export function newFindNextVersionView(feature: LocationOutsideFeature): FindNextVersionView {
    return toApplicationView(
        initFindNextVersionAction(
            initFindNextVersionMaterial(
                newFindNextVersionInfra(),
                newFindNextVersionLocationDetecter(feature),
            ),
        ),
    )
}
