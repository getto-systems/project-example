import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
    ApplicationView,
} from "../../../z_vendor/getto-application/action/action"
import { CheckAuthTicketAction } from "../../ticket/check/action"
import { AuthenticatePasswordAction } from "../../user/password/authenticate/action"
import { RequestResetTokenAction } from "../../user/password/reset/request_token/action"
import { ResetPasswordAction } from "../../user/password/reset/reset/action"

import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { SignViewType } from "../router/data"
import { SignViewTypeDetecter } from "../router/infra"

export type SignAction = StatefulApplicationAction<SignActionState>

export interface SignViewFactory {
    check(): ApplicationView<CheckAuthTicketAction>

    password_authenticate(): ApplicationView<AuthenticatePasswordAction>

    password_reset_requestToken(): ApplicationView<RequestResetTokenAction>
    password_reset(): ApplicationView<ResetPasswordAction>
}

export type SignActionState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "static-privacyPolicy" }>
    | View<"authTicket-check", ApplicationView<CheckAuthTicketAction>>
    | View<"password-authenticate", ApplicationView<AuthenticatePasswordAction>>
    | View<"password-reset-requestToken", ApplicationView<RequestResetTokenAction>>
    | View<"password-reset", ApplicationView<ResetPasswordAction>>

type View<T, V> = Readonly<{ type: T; view: V }>

const initialState: SignActionState = { type: "initial" }

export type SignActionShell = Readonly<{
    detectViewType: SignViewTypeDetecter
}>

export function initSignAction(shell: SignActionShell, factory: SignViewFactory): SignAction {
    return new Action(shell, factory)
}

class Action extends AbstractStatefulApplicationAction<SignActionState> implements SignAction {
    readonly initialState = initialState

    factory: SignViewFactory

    constructor(shell: SignActionShell, factory: SignViewFactory) {
        super({
            ignite: async () => {
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
                return this.post({ type: "authTicket-check", view })
            },
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
                return { type }

            case "password-reset-requestToken":
                return { type, view: this.factory.password_reset_requestToken() }
            case "password-reset":
                return { type, view: this.factory.password_reset() }
        }
    }
}
