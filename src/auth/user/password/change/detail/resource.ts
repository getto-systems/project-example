import { newChangePasswordConfig, newOverwritePasswordConfig } from "./config"
import { newChangePasswordInfra, newOverwritePasswordInfra } from "./infra"

import { Atom } from "../../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../../common/util/load/data"
import {
    ChangePasswordAction,
    initChangePasswordAction,
    initOverwritePasswordAction,
    OverwritePasswordAction,
} from "../action"

import { AuthUserAccount } from "../../../account/kernel/data"

export function newChangePasswordAction(): ChangePasswordAction {
    return initChangePasswordAction({
        infra: newChangePasswordInfra(),
        config: newChangePasswordConfig(),
    })
}

export function newOverwritePasswordAction(
    data: Atom<LoadState<AuthUserAccount>>,
): OverwritePasswordAction {
    return initOverwritePasswordAction(data, {
        infra: newOverwritePasswordInfra(),
        config: newOverwritePasswordConfig(),
    })
}
