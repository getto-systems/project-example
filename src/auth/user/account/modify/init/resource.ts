import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newModifyAuthUserAccountConfig } from "./config"

import {
    initModifyAuthUserAccountAction,
    ModifyAuthUserAccountAction,
    ModifyAuthUserAccountEntry,
} from "../action"
import { ModifyFieldHandler } from "../../../../../z_lib/ui/modify/action"

import { newModifyAuthUserAccountInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature
export function newModifyAuthUserAccountAction(feature: OutsideFeature): Readonly<{
    action: ModifyAuthUserAccountAction
    handler: ModifyFieldHandler<ModifyAuthUserAccountEntry>
}> {
    return initModifyAuthUserAccountAction({
        infra: newModifyAuthUserAccountInfra(feature),
        config: newModifyAuthUserAccountConfig(),
    })
}
