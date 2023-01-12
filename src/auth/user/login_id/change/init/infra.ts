import { newOverwriteLoginIdRemote } from "./overwrite_remote"

import { OverwriteLoginIdInfra } from "../action"

export function newOverwritePasswordInfra(): OverwriteLoginIdInfra {
    return {
        overwriteLoginIdRemote: newOverwriteLoginIdRemote(),
    }
}
