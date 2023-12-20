import { newRegisterAuthUserAccountConfig } from "./config"
import { newRegisterAuthUserAccountInfra } from "./infra"

import { LoadableListAtomUpdater } from "../../../../../common/util/list/action"
import { initRegisterAuthUserAccountAction, RegisterAuthUserAccountAction } from "../action"

import { AuthUserAccount } from "../../kernel/data"

export function newRegisterAuthUserAccountAction(): [
    RegisterAuthUserAccountAction,
    LoadableListAtomUpdater<AuthUserAccount>,
] {
    return initRegisterAuthUserAccountAction({
        infra: newRegisterAuthUserAccountInfra(),
        config: newRegisterAuthUserAccountConfig(),
    })
}
