import { ValidateBoardActionState } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"
import { RequestResetTokenProfileAction, RequestResetTokenProfileState } from "./action"

export type RequestResetTokenProfileResource = Readonly<{
    requestToken: RequestResetTokenProfileAction
}>

export type RequestResetTokenResourceProfileState = Readonly<{
    state: RequestResetTokenProfileState
    validate: ValidateBoardActionState
}>
