import { toApplicationView } from "../../../../../ui/vendor/getto-application/action/helper"

import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { newFindNextVersionConfig } from "./config"
import { newFindNextVersionShell } from "./shell"

import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"
import { FindNextVersionAction, initFindNextVersionAction } from "../action"

import { newFindNextVersionInfra } from "./infra"

export function newFindNextVersionView(
    feature: LocationOutsideFeature,
): ApplicationView<FindNextVersionAction> {
    return toApplicationView(
        initFindNextVersionAction({
            infra: newFindNextVersionInfra(),
            shell: newFindNextVersionShell(feature),
            config: newFindNextVersionConfig(),
        }),
    )
}
