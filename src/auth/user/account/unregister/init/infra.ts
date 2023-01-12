import { newUnregisterAuthUserAccountRemote } from "./unregister_remote"

import { UnregisterAuthUserAccountInfra } from "../action"

export function newUnregisterAuthUserAccountInfra(): UnregisterAuthUserAccountInfra {
    return {
        unregisterUserRemote: newUnregisterAuthUserAccountRemote(),
    }
}
