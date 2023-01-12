import { newRequestResetTokenConfig } from "./config"
import { newRequestResetTokenInfra } from "./infra"

import { initRequestResetTokenAction, RequestResetTokenAction } from "../action"

export function newRequestResetTokenAction(): RequestResetTokenAction {
    return initRequestResetTokenAction({
        infra: newRequestResetTokenInfra(),
        config: newRequestResetTokenConfig(),
    })
}
