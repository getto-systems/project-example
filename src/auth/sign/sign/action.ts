import {
    ApplicationStateAction,
    ApplicationView,
} from "../../../../ui/vendor/getto-application/action/action"
import { ApplicationAbstractStateAction } from "../../../../ui/vendor/getto-application/action/init"
import { CheckAuthTicketAction } from "../../ticket/check/action"
import { AuthenticatePasswordAction } from "../../user/password/authenticate/action"
import { RequestResetTokenAction } from "../../user/password/reset/request_token/action"
import { ResetPasswordAction } from "../../user/password/reset/reset/action"
import { SignLink } from "../nav/resource"

import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { SignViewType } from "../router/data"
import { SignViewTypeDetecter } from "../router/infra"

export type SignAction = ApplicationStateAction<SignActionState>

export interface SignViewFactory {
    link(): Readonly<{ link: SignLink }>

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

type Static<T extends string> = Readonly<{
    type: `static-${T}`
    resource: Readonly<{ link: SignLink }>
}>
type View<T, V> = Readonly<{ type: T; view: V }>

export const initialSignViewState: SignActionState = { type: "initial-view" }

export type SignActionShell = Readonly<{
    detectViewType: SignViewTypeDetecter
}>

export function initSignAction(shell: SignActionShell, factory: SignViewFactory): SignAction {
    return new Action(shell, factory)
}

class Action extends ApplicationAbstractStateAction<SignActionState> implements SignAction {
    readonly initialState = initialSignViewState

    factory: SignViewFactory

    constructor(shell: SignActionShell, factory: SignViewFactory) {
        super(async () => {
            const view = this.factory.check()
            const viewType = shell.detectViewType()

            view.resource.subscriber.subscribe((state) => {
                switch (state.type) {
                    case "required-to-login":
                        this.post(this.mapViewType(viewType))
                        return
                }
            })

            if (viewType.valid) {
                switch (viewType.value) {
                    case "password-reset":
                        return this.post(this.mapViewType(viewType))
                }
            }
            return this.post({ type: "check-authTicket", view: view })
        })
        this.factory = factory
    }

    mapViewType(result: ConvertLocationResult<SignViewType>): SignActionState {
        if (!result.valid) {
            // 特に指定が無ければパスワードログイン
            return {
                type: "password-authenticate",
                view: this.factory.password_authenticate(),
            }
        }

        const type = result.value
        switch (type) {
            case "static-privacyPolicy":
                return { type, resource: this.factory.link() }

            case "password-reset-requestToken":
                return { type, view: this.factory.password_reset_requestToken() }
            case "password-reset":
                return { type, view: this.factory.password_reset() }
        }
    }
}