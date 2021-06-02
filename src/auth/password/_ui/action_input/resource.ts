import { InputPasswordAction, ValidatePasswordState } from "./core/action"

export type InputPasswordResource = Readonly<{
    field: InputPasswordAction
}>

export type InputPasswordResourceState = Readonly<{
    state: ValidatePasswordState
}>
