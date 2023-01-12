import { newChangePasswordConfig, newOverwritePasswordConfig } from "./config"

import {
    ChangePasswordAction,
    initChangePasswordAction,
    initOverwritePasswordAction,
    OverwritePasswordAction,
    OverwritePasswordEntry,
} from "../action"
import { ModifyFieldHandler } from "../../../../../common/util/modify/action"

import { newChangePasswordInfra, newOverwritePasswordInfra } from "./infra"

export function newChangePasswordAction(): ChangePasswordAction {
    return initChangePasswordAction({
        infra: newChangePasswordInfra(),
        config: newChangePasswordConfig(),
    })
}

export function newOverwritePasswordAction(): Readonly<{
    action: OverwritePasswordAction
    handler: ModifyFieldHandler<OverwritePasswordEntry>
}> {
    return initOverwritePasswordAction({
        infra: newOverwritePasswordInfra(),
        config: newOverwritePasswordConfig(),
    })
}
