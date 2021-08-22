import { ApplicationView } from "../../../../ui/vendor/getto-application/action/action"
import { SignAction, SignActionState } from "./action"

export type SignView = ApplicationView<SignResource>

export type SignResource = Readonly<{
    sign: SignAction
}>
export type SignResourceState = Readonly<{
    state: SignActionState
}>
