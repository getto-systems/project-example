import { h, VNode } from "preact"
import { useErrorBoundary } from "preact/hooks"
import { html } from "htm/preact"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { ApplicationError } from "../../../../avail/x_preact/application_error"
import { CheckAuthTicket } from "../../../ticket/check/x_preact/check_ticket"
import { AuthenticatePassword } from "../../../user/password/authenticate/x_preact/authenticate_password"
import { RequestResetToken } from "../../../user/password/reset/request_token/x_preact/request_token"
import { ResetPassword } from "../../../user/password/reset/reset/x_preact/reset_password"
import { PrivacyPolicy } from "./privacy_policy"

import { ApplicationView } from "../../../../z_vendor/getto-application/action/action"
import { SignAction } from "../action"
import { SignLink } from "../../nav/action"

type Props = Readonly<{
    link: SignLink
    sign: ApplicationView<SignAction>
}>
export function Sign(viewProps: Props): VNode {
    const props = {
        link: viewProps.link,
        sign: useApplicationView(viewProps.sign),
    }
    const state = useApplicationAction(props.sign)
    const [err] = useErrorBoundary((err) => {
        // 認証前なのでエラーはどうしようもない
        console.log(err)
    })

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    switch (state.type) {
        case "initial":
            return html``

        case "static-privacyPolicy":
            return h(PrivacyPolicy, { link: props.link })

        case "authTicket-check":
            return h(CheckAuthTicket, state.view)

        case "password-authenticate":
            return h(AuthenticatePassword, { link: props.link, authenticate: state.view })

        case "password-reset-requestToken":
            return h(RequestResetToken, { link: props.link, requestToken: state.view })

        case "password-reset":
            return h(ResetPassword, { link: props.link, reset: state.view })
    }
}
