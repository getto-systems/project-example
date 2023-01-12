import { newUnregisterAuthUserAccountConfig } from "./config"

import {
    initUnregisterAuthUserAccountAction,
    UnregisterAuthUserAccountAction,
    UnregisterAuthUserAccountEntry,
} from "../action"

import { newUnregisterAuthUserAccountInfra } from "./infra"
import { ModifyFieldHandler } from "../../../../../common/util/modify/action"

export function newUnregisterAuthUserAccountAction(): Readonly<{
    action: UnregisterAuthUserAccountAction
    handler: ModifyFieldHandler<UnregisterAuthUserAccountEntry>
}> {
    return initUnregisterAuthUserAccountAction({
        infra: newUnregisterAuthUserAccountInfra(),
        config: newUnregisterAuthUserAccountConfig(),
    })
}
