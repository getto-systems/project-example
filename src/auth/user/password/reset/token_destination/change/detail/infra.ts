import { newModifyAuthUserAccountRemote } from "./change_remote"

import { ChangeResetTokenDestinationInfra } from "../action"

export function newChangeResetTokenDestinationInfra(): ChangeResetTokenDestinationInfra {
    return {
        changeDestinationRemote: newModifyAuthUserAccountRemote(),
    }
}
