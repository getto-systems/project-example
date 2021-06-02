import { newFindNextVersionInfra, newFindNextVersionLocationDetecter } from "../find_next/impl/init"

import { initFindNextVersionView } from "./impl"
import { initFindNextVersionCoreAction, initFindNextVersionCoreMaterial } from "./core/impl"

import { FindNextVersionView } from "./resource"
import { LocationOutsideFeature } from "../../../../ui/vendor/getto-application/location/infra"

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
