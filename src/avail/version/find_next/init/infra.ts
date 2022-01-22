import { FindNextVersionInfra } from "../action"

import { newCheckDeployExistsRemote } from "./check_remote"

export function newFindNextVersionInfra(): FindNextVersionInfra {
    return {
        check: newCheckDeployExistsRemote(),
    }
}
