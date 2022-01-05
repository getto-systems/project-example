import { h, VNode } from "preact"
import { useErrorBoundary } from "preact/hooks"
import { html } from "htm/preact"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { ApplicationErrorComponent } from "../../../../avail/x_preact/application_error"
import { CheckAuthTicketEntry } from "../../../ticket/check/x_preact/check_ticket"
import { AuthenticatePasswordEntry } from "../../../user/password/authenticate/x_preact/authenticate_password"
import { RequestResetTokenEntry } from "../../../user/password/reset/action_request_token/x_preact/request_token"
import { ResetPasswordEntry } from "../../../user/password/reset/reset/x_preact/reset_password"

import { SignView, SignResource, SignResourceState } from "../resource"
import { PrivacyPolicyComponent } from "./privacy_policy"

export function SignEntry(view: SignView): VNode {
    const action = useApplicationView(view)
    const state = useApplicationAction(action)
    const [err] = useErrorBoundary((err) => {
        // 認証前なのでエラーはどうしようもない
        console.log(err)
    })

    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }
    return h(SignComponent, { state, sign: action })
}

type Props = SignResource & SignResourceState
export function SignComponent(props: Props): VNode {
    switch (props.state.type) {
        case "initial-view":
            return EMPTY_CONTENT

        case "static-privacyPolicy":
            return h(PrivacyPolicyComponent, props.state.resource)

        case "check-authTicket":
            return h(CheckAuthTicketEntry, props.state.view)

        case "password-authenticate":
            return h(AuthenticatePasswordEntry, props.state.view)

        case "password-reset-requestToken":
            return h(RequestResetTokenEntry, props.state.view)

        case "password-reset":
            return h(ResetPasswordEntry, props.state.view)

        case "error":
            return h(ApplicationErrorComponent, { err: props.state.err })
    }
}

const EMPTY_CONTENT = html``
