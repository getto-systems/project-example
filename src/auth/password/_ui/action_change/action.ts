import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { InputPasswordAction } from "../action_input/action"
import { ValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { ChangePasswordMethod } from "../change/method"

import { ChangePasswordEvent } from "../change/event"

export interface ChangePasswordAction extends ApplicationStateAction<ChangePasswordState> {
    readonly currentPassword: InputPasswordAction
    readonly newPassword: InputPasswordAction
    readonly validate: ValidateBoardAction

    clear(): ChangePasswordState
    submit(): Promise<ChangePasswordState>
}

export const changePasswordFieldNames = ["currentPassword", "newPassword"] as const
export type ChangePasswordFieldName = typeof changePasswordFieldNames[number]

export type ChangePasswordMaterial = Readonly<{
    change: ChangePasswordMethod
}>

export type ChangePasswordState =
    | Readonly<{ type: "initial-change-password" }>
    | ChangePasswordEvent

export const initialChangePasswordState: ChangePasswordState = {
    type: "initial-change-password",
}
