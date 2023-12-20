import { newModifyAuthUserAccountRemote } from "./modify_remote"

import { ModifyAuthUserAccountInfra } from "../action"

export function newModifyAuthUserAccountInfra(): ModifyAuthUserAccountInfra {
    return {
        modifyUserRemote: newModifyAuthUserAccountRemote(),
    }
}
