import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newUnregisterAuthUserAccountConfig } from "./config"

import {
    initUnregisterAuthUserAccountAction,
    UnregisterAuthUserAccountAction,
    UnregisterAuthUserAccountEntry,
} from "../action"

import { newUnregisterAuthUserAccountInfra } from "./infra"
import { ModifyFieldHandler } from "../../../../../z_lib/ui/modify/action"

type OutsideFeature = RemoteOutsideFeature
export function newUnregisterAuthUserAccountAction(feature: OutsideFeature): Readonly<{
    action: UnregisterAuthUserAccountAction
    handler: ModifyFieldHandler<UnregisterAuthUserAccountEntry>
}> {
    return initUnregisterAuthUserAccountAction({
        infra: newUnregisterAuthUserAccountInfra(feature),
        config: newUnregisterAuthUserAccountConfig(),
    })
}
