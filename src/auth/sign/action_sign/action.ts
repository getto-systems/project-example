import {
    ApplicationStateAction,
    ApplicationView,
} from "../../../../ui/vendor/getto-application/action/action"
import { CheckAuthTicketAction } from "../../ticket/check/action"
import { AuthenticatePasswordAction } from "../../user/password/authenticate/action"
import { RequestResetTokenAction } from "../../user/password/reset/request_token/action"
import { ResetPasswordAction } from "../../user/password/reset/reset/action"
import { SignLinkResource } from "../action_nav/resource"

export type SignAction = ApplicationStateAction<SignActionState>

export interface SignSubView {
    link(): SignLinkResource

    check(): ApplicationView<CheckAuthTicketAction>

    password_authenticate(): ApplicationView<AuthenticatePasswordAction>

    password_reset_requestToken(): ApplicationView<RequestResetTokenAction>
    password_reset(): ApplicationView<ResetPasswordAction>
}

export type SignActionState =
    | Readonly<{ type: "initial-view" }>
    | Static<"privacyPolicy">
    | View<"check-authTicket", ApplicationView<CheckAuthTicketAction>>
    | View<"password-authenticate", ApplicationView<AuthenticatePasswordAction>>
    | View<"password-reset-requestToken", ApplicationView<RequestResetTokenAction>>
    | View<"password-reset", ApplicationView<ResetPasswordAction>>
    | Readonly<{ type: "error"; err: string }>

type Static<T extends string> = Readonly<{ type: `static-${T}`; resource: SignLinkResource }>
type View<T, V> = Readonly<{ type: T; view: V }>

export const initialSignViewState: SignActionState = { type: "initial-view" }
