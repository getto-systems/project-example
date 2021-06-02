import { ApplicationStateAction } from "../../../../../../../ui/vendor/getto-application/action/action"

import { RequestResetTokenMethod } from "../../request_token/method"

import { RequestResetTokenEvent } from "../../request_token/event"

import { ConvertBoardResult } from "../../../../../../../ui/vendor/getto-application/board/kernel/data"
import { RequestResetTokenFields } from "../../request_token/data"

export interface RequestResetTokenCoreAction
    extends ApplicationStateAction<RequestResetTokenCoreState> {
    submit(fields: ConvertBoardResult<RequestResetTokenFields>): Promise<RequestResetTokenCoreState>
}

export type RequestResetTokenCoreMaterial = Readonly<{
    requestToken: RequestResetTokenMethod
}>

export type RequestResetTokenCoreState =
    | Readonly<{ type: "initial-request-token" }>
    | RequestResetTokenEvent

export const initialRequestResetTokenCoreState: RequestResetTokenCoreState = {
    type: "initial-request-token",
}
