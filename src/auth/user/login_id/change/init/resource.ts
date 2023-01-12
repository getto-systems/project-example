import { newOverwritePasswordConfig } from "./config"

import {
    initOverwriteLoginIdAction,
    OverwriteLoginIdAction,
    OverwriteLoginIdEntry,
} from "../action"
import { ModifyFieldHandler } from "../../../../../common/util/modify/action"

import { newOverwritePasswordInfra } from "./infra"

export function newOverwriteLoginIdAction(): Readonly<{
    action: OverwriteLoginIdAction
    handler: ModifyFieldHandler<OverwriteLoginIdEntry>
}> {
    return initOverwriteLoginIdAction({
        infra: newOverwritePasswordInfra(),
        config: newOverwritePasswordConfig(),
    })
}
