import { InputPasswordAction, ValidatePasswordState } from "./action"

export type InputPasswordResource = Readonly<{
    field: InputPasswordAction
}>

export type InputPasswordResourceState = Readonly<{
    state: ValidatePasswordState
}>
