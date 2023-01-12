import { newRegisterAuthUserAccountConfig } from "./config"

import { initRegisterAuthUserAccountAction, RegisterAuthUserAccountAction } from "../action"

import { newRegisterAuthUserAccountInfra } from "./infra"

export function newRegisterAuthUserAccountAction(): RegisterAuthUserAccountAction {
    return initRegisterAuthUserAccountAction({
        infra: newRegisterAuthUserAccountInfra(),
        config: newRegisterAuthUserAccountConfig(),
    })
}
