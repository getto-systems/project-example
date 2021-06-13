import { newFindNextVersionInfra, newFindNextVersionLocationDetecter } from "../find_next/init"

import { initFindNextVersionView } from "./impl"
import { initFindNextVersionCoreAction, initFindNextVersionCoreMaterial } from "./core/impl"

import { LocationOutsideFeature } from "../../../z_details/_ui/location/feature"

import { FindNextVersionView } from "./resource"

export function newFindNextVersionView(feature: LocationOutsideFeature): FindNextVersionView {
    return initFindNextVersionView({
        findNext: initFindNextVersionCoreAction(
            initFindNextVersionCoreMaterial(
                newFindNextVersionInfra(),
                newFindNextVersionLocationDetecter(feature),
            ),
        ),
    })
}
