import { env } from "../../../../y_environment/ui/env"
import { FindNextVersionInfra } from "../action"

import { newCheckDeployExistsRemote } from "./check_remote"

export function newFindNextVersionInfra(): FindNextVersionInfra {
    return {
        version: env.version,
        versionSuffix: env.versionSuffix,
        check: newCheckDeployExistsRemote(),
    }
}
