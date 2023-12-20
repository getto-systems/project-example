import { newRequestResetTokenRemote } from "./request_token_remote"

import { RequestResetTokenInfra } from "../action"

export function newRequestResetTokenInfra(): RequestResetTokenInfra {
    return {
        requestTokenRemote: newRequestResetTokenRemote(),
    }
}
