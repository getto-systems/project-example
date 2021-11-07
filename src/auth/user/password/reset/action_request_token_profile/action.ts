import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"
import { InputLoginIDAction } from "../../../login_id/input/action_input/action"
import { ValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { RequestResetTokenMethod } from "../request_token/method"

import { RequestResetTokenEvent } from "../request_token/event"

export interface RequestResetTokenProfileAction
    extends ApplicationStateAction<RequestResetTokenProfileState> {
    readonly loginID: InputLoginIDAction
    readonly validate: ValidateBoardAction

    open(): RequestResetTokenProfileState
    clear(): RequestResetTokenProfileState
    submit(): Promise<RequestResetTokenProfileState>
    close(): RequestResetTokenProfileState
}

export const requestResetTokenProfileFieldNames = ["loginID"] as const
export type RequestResetTokenProfileFieldName = typeof requestResetTokenProfileFieldNames[number]

export type RequestResetTokenProfileMaterial = Readonly<{
    requestToken: RequestResetTokenMethod
}>

export type RequestResetTokenProfileState =
    | Readonly<{ type: "initial-request-token" }>
    | Readonly<{ type: "input-login-id" }>
    | RequestResetTokenEvent

export const initialRequestResetTokenProfileState: RequestResetTokenProfileState = {
    type: "initial-request-token",
}
