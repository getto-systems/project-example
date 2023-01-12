import { newChangeResetTokenDestinationConfig } from "./config"
import { newChangeResetTokenDestinationInfra } from "./infra"

import {
    initChangeResetTokenDestinationAction,
    ChangeResetTokenDestinationAction,
    ChangeResetTokenDestinationEntry,
} from "../action"
import { ModifyFieldHandler } from "../../../../../../../common/util/modify/action"

export function newChangeResetTokenDestinationAction(): Readonly<{
    action: ChangeResetTokenDestinationAction
    handler: ModifyFieldHandler<ChangeResetTokenDestinationEntry>
}> {
    return initChangeResetTokenDestinationAction({
        infra: newChangeResetTokenDestinationInfra(),
        config: newChangeResetTokenDestinationConfig(),
    })
}
