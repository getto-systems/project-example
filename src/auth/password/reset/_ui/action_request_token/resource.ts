import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"

import { ValidateBoardActionState } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"
import { RequestResetTokenAction, RequestResetTokenState } from "./action"

export type RequestResetTokenView = ApplicationView<RequestResetTokenAction>

export type RequestResetTokenResource = Readonly<{
    requestToken: RequestResetTokenAction
}>

export type RequestResetTokenResourceState = Readonly<{
    state: RequestResetTokenState
    validate: ValidateBoardActionState
}>
