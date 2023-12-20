import { newChangePasswordRemote } from "./change_remote"
import { newOverwritePasswordRemote } from "./overwrite_remote"

import { ChangePasswordInfra, OverwritePasswordInfra } from "../action"

export function newChangePasswordInfra(): ChangePasswordInfra {
    return {
        changePasswordRemote: newChangePasswordRemote(),
    }
}

export function newOverwritePasswordInfra(): OverwritePasswordInfra {
    return {
        overwritePasswordRemote: newOverwritePasswordRemote(),
    }
}
