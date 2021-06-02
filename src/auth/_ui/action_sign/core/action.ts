import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { CheckAuthTicketView } from "../../../auth_ticket/_ui/action_check/resource"
import { AuthenticatePasswordView } from "../../../password/_ui/action_authenticate/resource"
import { CheckResetTokenSendingStatusView } from "../../../password/reset/ui/action_check_status/resource"
import { RequestResetTokenView } from "../../../password/reset/ui/action_request_token/resource"
import { ResetPasswordView } from "../../../password/reset/ui/action_reset/resource"
import { SignLinkResource } from "../../common/nav/action_nav/resource"

export interface SignAction extends ApplicationStateAction<SignActionState> {
    error(err: string): Promise<SignActionState>
}

export interface SignSubView {
    link(): SignLinkResource

    check(): CheckAuthTicketView

    password_authenticate(): AuthenticatePasswordView

    password_reset_requestToken(): RequestResetTokenView
    password_reset_checkStatus(): CheckResetTokenSendingStatusView
    password_reset(): ResetPasswordView
}

export type SignActionState =
    | Readonly<{ type: "initial-view" }>
    | Static<"privacyPolicy">
    | View<"check-authTicket", CheckAuthTicketView>
    | View<"password-authenticate", AuthenticatePasswordView>
    | View<"password-reset-requestToken", RequestResetTokenView>
    | View<"password-reset-checkStatus", CheckResetTokenSendingStatusView>
    | View<"password-reset", ResetPasswordView>
    | Readonly<{ type: "error"; err: string }>

type Static<T extends string> = Readonly<{ type: `static-${T}`; resource: SignLinkResource }>
type View<T, V> = Readonly<{ type: T; view: V }>

export const initialSignViewState: SignActionState = { type: "initial-view" }
