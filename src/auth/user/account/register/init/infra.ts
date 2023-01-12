import { newRegisterAuthUserAccountRemote } from "./register_remote"

import { RegisterAuthUserAccountInfra } from "../action"

export function newRegisterAuthUserAccountInfra(): RegisterAuthUserAccountInfra {
    return {
        registerUserRemote: newRegisterAuthUserAccountRemote(),
    }
}
