import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newOverwritePasswordConfig } from "./config"

import {
    initOverwriteLoginIdAction,
    OverwriteLoginIdAction,
    OverwriteLoginIdEntry,
} from "../action"
import { ModifyFieldHandler } from "../../../../../z_lib/ui/modify/action"

import { newOverwritePasswordInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature
export function newOverwriteLoginIdAction(feature: OutsideFeature): Readonly<{
    action: OverwriteLoginIdAction
    handler: ModifyFieldHandler<OverwriteLoginIdEntry>
}> {
    return initOverwriteLoginIdAction({
        infra: newOverwritePasswordInfra(feature),
        config: newOverwritePasswordConfig(),
    })
}
