import { SignLink } from "../../../../sign/action_nav/resource"
import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"
import { InputLoginIDAction } from "../../../login_id/input/action_input/action"
import { ValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { RequestResetTokenMethod } from "../request_token/method"

import { RequestResetTokenEvent } from "../request_token/event"

export interface RequestResetTokenAction extends ApplicationStateAction<RequestResetTokenState> {
    readonly link: SignLink

    readonly loginID: InputLoginIDAction
    readonly validate: ValidateBoardAction

    clear(): void
    submit(): Promise<RequestResetTokenState>
}

export const requestResetTokenFieldNames = ["loginID"] as const
export type RequestResetTokenFieldName = typeof requestResetTokenFieldNames[number]

export type RequestResetTokenMaterial = Readonly<{
    requestToken: RequestResetTokenMethod
}>

export type RequestResetTokenState =
    | Readonly<{ type: "initial-request-token" }>
    | RequestResetTokenEvent

export const initialRequestResetTokenState: RequestResetTokenState = {
    type: "initial-request-token",
}