import {
    initApplicationState,
    StatefulApplicationAction,
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
    check(): CheckAuthTicketAction

    password_authenticate(): AuthenticatePasswordAction

    password_reset_requestToken(): RequestResetTokenAction
    password_reset(): ResetPasswordAction
}

export type SignActionState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "static-privacyPolicy" }>
    | View<"authTicket-check", CheckAuthTicketAction>
    | View<"password-authenticate", AuthenticatePasswordAction>
    | View<"password-reset-requestToken", RequestResetTokenAction>
    | View<"password-reset", ResetPasswordAction>

type View<T, A> = Readonly<{ type: T; action: A }>

const initialState: SignActionState = { type: "initial" }

export type SignActionShell = Readonly<{
    detectViewType: SignViewTypeDetecter
}>

export function initSignAction(shell: SignActionShell, factory: SignViewFactory): SignAction {
    const { state, post } = initApplicationState({ initialState, ignite })
    return { state }

    async function ignite(): Promise<SignActionState> {
        const checkAction = factory.check()
        const viewType = shell.detectViewType()

        checkAction.state.subscribe((state) => {
            switch (state.type) {
                case "required-to-login":
                    post(mapViewType(viewType))
                    return
            }
        })

        if (viewType.valid) {
            switch (viewType.value) {
                case "password-reset":
                    return post(mapViewType(viewType))
            }
        }
        return post({ type: "authTicket-check", action: checkAction })
    }
    function mapViewType(result: ConvertLocationResult<SignViewType>): SignActionState {
        if (!result.valid) {
            // 特に指定が無ければパスワードログイン
            return {
                type: "password-authenticate",
                action: factory.password_authenticate(),
            }
        }

        const type = result.value
        switch (type) {
            case "static-privacyPolicy":
                return { type }

            case "password-reset-requestToken":
                return { type, action: factory.password_reset_requestToken() }
            case "password-reset":
                return { type, action: factory.password_reset() }
        }
    }
}
