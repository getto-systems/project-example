import { newModifyAuthUserAccountConfig } from "./config"

import {
    initModifyAuthUserAccountAction,
    ModifyAuthUserAccountAction,
    ModifyAuthUserAccountEntry,
} from "../action"
import { ModifyFieldHandler } from "../../../../../common/util/modify/action"

import { newModifyAuthUserAccountInfra } from "./infra"

export function newModifyAuthUserAccountAction(): Readonly<{
    action: ModifyAuthUserAccountAction
    handler: ModifyFieldHandler<ModifyAuthUserAccountEntry>
}> {
    return initModifyAuthUserAccountAction({
        infra: newModifyAuthUserAccountInfra(),
        config: newModifyAuthUserAccountConfig(),
    })
}
