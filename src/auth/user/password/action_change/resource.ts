import { ValidateBoardActionState } from "../../../../../ui/vendor/getto-application/board/action_validate_board/action"
import { ChangePasswordAction, ChangePasswordState } from "./action"

export type ChangePasswordResource = Readonly<{
    change: ChangePasswordAction
}>

export type ChangePasswordResourceState = Readonly<{
    state: ChangePasswordState
    validate: ValidateBoardActionState
}>
