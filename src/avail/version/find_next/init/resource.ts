import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { newFindNextVersionConfig } from "./config"
import { newFindNextVersionShell } from "./shell"

import { FindNextVersionAction, initFindNextVersionAction } from "../action"

import { newFindNextVersionInfra } from "./infra"

export function newFindNextVersionAction(feature: LocationOutsideFeature): FindNextVersionAction {
    return initFindNextVersionAction({
        infra: newFindNextVersionInfra(),
        shell: newFindNextVersionShell(feature),
        config: newFindNextVersionConfig(),
    })
}
