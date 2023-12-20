import { h } from "preact"
import { html } from "htm/preact"
import { PreactNode } from "../../../../common/x_preact/vnode"

import { useErrorBoundary } from "preact/hooks"
import { useAtom } from "../../../../z_vendor/getto-atom/x_preact/hooks"

import { ApplicationError } from "../../../../avail/x_preact/application_error"
import { CheckAuthTicket } from "../../../ticket/authenticate/x_preact/check_ticket"
import { AuthenticatePassword } from "../../../user/password/authenticate/x_preact/authenticate_password"
import { RequestResetToken } from "../../../user/password/reset/request_token/x_preact/request_token"
import { ResetPassword } from "../../../user/password/reset/reset/x_preact/reset_password"
import { PrivacyPolicy } from "./privacy_policy"

import { SignAction } from "../action"
import { SignLink } from "../../nav/action"

type Props = Readonly<{
    link: SignLink
    sign: SignAction
}>
export function Sign(props: Props): PreactNode {
    const state = useAtom(props.sign.state)
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
            return h(CheckAuthTicket, { check: state.action })

        case "password-authenticate":
            return h(AuthenticatePassword, { link: props.link, authenticate: state.action })

        case "password-reset-requestToken":
            return h(RequestResetToken, { link: props.link, requestToken: state.action })

        case "password-reset":
            return h(ResetPassword, { link: props.link, reset: state.action })
    }
}
