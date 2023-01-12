import { h, VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../../common/x_preact/vnode"

import { remoteCommonErrorReason } from "../../../../../../common/util/remote/x_error/reason"

import { useApplicationState } from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../../z_vendor/getto-css/preact/design/form"
import { loginBox } from "../../../../../../z_vendor/getto-css/preact/layout/login"

import { siteInfo } from "../../../../../../x_content/site"
import { icon_change } from "../../../../../../x_content/icon"
import { signNav } from "../../../../../sign/nav/x_preact/nav"
import { takeLongtimeField, ValidationMessage } from "../../../../../../common/x_preact/design/form"

import { LoginIdField } from "../../../../login_id/input/x_preact/field"
import { ClearChangesButton } from "../../../../../../common/x_preact/button/clear_changes_button"
import { SendButton } from "../../../../../../common/x_preact/button/send_button"

import { RequestResetTokenAction } from "../action"
import { SignLink } from "../../../../../sign/nav/action"

import { RequestResetTokenError } from "../data"

type Props = Readonly<{
    link: SignLink
    requestToken: RequestResetTokenAction
}>
export function RequestResetToken(props: Props): VNode {
    const requestTokenState = useApplicationState(props.requestToken.state)

    return loginBox(siteInfo, {
        title: "パスワードリセット",
        footer: footerLinks(),
        ...(requestTokenState.type === "success"
            ? {
                  body: [
                      html`<p>トークンの送信が完了しました</p>`,
                      html`<p>
                          メールからパスワードのリセットができます<br />
                          メールを確認してください
                      </p>`,
                  ],
              }
            : {
                  form: true,
                  body: [
                      h(LoginIdField, {
                          field: props.requestToken.loginId,
                          help: ["登録されたメールアドレスにリセットトークンを送信します"],
                      }),
                      buttons({ left: h(Submit, {}), right: h(Clear, {}) }),
                      h(ValidationMessage, props.requestToken.validate),
                      h(Message, {}),
                  ],
              }),
    })

    function Submit(_props: unknown): VNode {
        const validateState = useApplicationState(props.requestToken.validate.state)
        const observeState = useApplicationState(props.requestToken.observe.state)

        return h(SendButton, {
            label: "トークン送信",
            icon: icon_change,
            isConnecting: requestTokenState.type === "try",
            validateState,
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.submit()
        }
    }

    function Clear(_props: unknown): VNode {
        const observeState = useApplicationState(props.requestToken.observe.state)

        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.clear()
        }
    }

    function Message(_props: unknown): VNode {
        switch (requestTokenState.type) {
            case "initial":
            case "success":
                return html``

            case "try":
                if (requestTokenState.hasTakenLongtime) {
                    return takeLongtimeField("トークンの送信")
                }
                return html``

            case "failed":
                return fieldHelp_error(requestTokenError(requestTokenState.err))
        }
    }

    function footerLinks(): VNode {
        return buttons({ left: privacyPolicyLink(), right: loginLink() })
    }
    function privacyPolicyLink(): VNode {
        return signNav(props.link.getNav_static_privacyPolicy())
    }
    function loginLink(): VNode {
        return signNav(props.link.getNav_password_authenticate())
    }
}

function requestTokenError(err: RequestResetTokenError): readonly VNodeContent[] {
    switch (err.type) {
        case "invalid-reset":
            return ["ログインIDが登録されていないか、トークンの送信先が登録されていません"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}によりトークンの送信に失敗しました`,
                ...reason.detail,
            ])
    }
}
