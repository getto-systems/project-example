import { SignLink } from "../../../../_ui/common/nav/action_nav/resource"
import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"
import { InputLoginIDAction } from "../../../../login_id/_ui/action_input/action"
import { ValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { RequestResetTokenMethod } from "../request_token/method"

import { RequestResetTokenEvent } from "../request_token/event"

import { RequestResetTokenFields } from "../request_token/data"

export interface RequestResetTokenAction extends ApplicationStateAction<RequestResetTokenState> {
    readonly link: SignLink

    readonly loginID: InputLoginIDAction
    readonly validate: ValidateRequestTokenAction

    clear(): void
    submit(): Promise<RequestResetTokenState>
}

export const requestResetTokenFields = ["loginID"] as const
export type ValidateRequestTokenAction = ValidateBoardAction<
    typeof requestResetTokenFields[number],
    RequestResetTokenFields
>

export type RequestResetTokenMaterial = Readonly<{
    requestToken: RequestResetTokenMethod
}>

export type RequestResetTokenState =
    | Readonly<{ type: "initial-request-token" }>
    | RequestResetTokenEvent

export const initialRequestResetTokenState: RequestResetTokenState = {
    type: "initial-request-token",
}
