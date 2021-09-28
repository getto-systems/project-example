import { ApplicationStateAction } from "../../../../ui/vendor/getto-application/action/action"

import { CheckAuthTicketView } from "../../ticket/action_check/resource"
import { AuthenticatePasswordView } from "../../user/password/action_authenticate/resource"
import { RequestResetTokenView } from "../../user/password/reset/action_request_token/resource"
import { ResetPasswordView } from "../../user/password/reset/action_reset/resource"
import { SignLinkResource } from "../action_nav/resource"

export interface SignAction extends ApplicationStateAction<SignActionState> {
    error(err: string): Promise<SignActionState>
}

export interface SignSubView {
    link(): SignLinkResource

    check(): CheckAuthTicketView

    password_authenticate(): AuthenticatePasswordView

    password_reset_requestToken(): RequestResetTokenView
    password_reset(): ResetPasswordView
}

export type SignActionState =
    | Readonly<{ type: "initial-view" }>
    | Static<"privacyPolicy">
    | View<"check-authTicket", CheckAuthTicketView>
    | View<"password-authenticate", AuthenticatePasswordView>
    | View<"password-reset-requestToken", RequestResetTokenView>
    | View<"password-reset", ResetPasswordView>
    | Readonly<{ type: "error"; err: string }>

type Static<T extends string> = Readonly<{ type: `static-${T}`; resource: SignLinkResource }>
type View<T, V> = Readonly<{ type: T; view: V }>

export const initialSignViewState: SignActionState = { type: "initial-view" }
