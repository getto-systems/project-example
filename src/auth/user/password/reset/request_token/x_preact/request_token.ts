import { h } from "preact"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../../../common/x_preact/node"

import { remoteCommonErrorReason } from "../../../../../../common/util/remote/x_error/reason"

import { useAtom } from "../../../../../../z_vendor/getto-atom/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../../z_vendor/getto-css/preact/design/form"
import { loginBox } from "../../../../../../z_vendor/getto-css/preact/layout/login"

import { siteInfo } from "../../../../../../x_content/site"
import { icon_change } from "../../../../../../x_content/icon"
import { signNav } from "../../../../../sign/nav/x_preact/nav"
import {
    takeLongtimeField,
    ValidateBoardMessage,
} from "../../../../../../common/x_preact/design/form"

import { AuthUserLoginIdField } from "../../../../login_id/input/field/x_preact/input"
import { ClearChangesButton } from "../../../../../../common/x_preact/button/clear_changes_button"
import { SendButton } from "../../../../../../common/x_preact/button/send_button"

import { RequestResetTokenAction } from "../action"
import { SignLink } from "../../../../../sign/nav/action"

import { RequestResetTokenError } from "../data"

type Props = Readonly<{
    link: SignLink
    requestToken: RequestResetTokenAction
}>
export function RequestResetToken(props: Props): PreactNode {
    const requestTokenState = useAtom(props.requestToken.state)

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
                      h(AuthUserLoginIdField, {
                          field: props.requestToken.loginId,
                          help: ["登録されたメールアドレスにリセットトークンを送信します"],
                      }),
                      buttons({ left: h(Submit, {}), right: h(Reset, {}) }),
                      h(ValidateBoardMessage, { state: props.requestToken.validate }),
                      h(Message, {}),
                  ],
              }),
    })

    function Submit(_props: unknown): PreactNode {
        return h(SendButton, {
            label: "トークン送信",
            icon: icon_change,
            connect: props.requestToken.connect,
            validate: props.requestToken.validate,
            observe: props.requestToken.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.submit()
        }
    }

    function Reset(_props: unknown): PreactNode {
        return h(ClearChangesButton, {
            observe: props.requestToken.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.reset()
        }
    }

    function Message(_props: unknown): PreactNode {
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

    function footerLinks(): PreactNode {
        return buttons({ left: privacyPolicyLink(), right: loginLink() })
    }
    function privacyPolicyLink(): PreactNode {
        return signNav(props.link.getNav_static_privacyPolicy())
    }
    function loginLink(): PreactNode {
        return signNav(props.link.getNav_password_authenticate())
    }
}

function requestTokenError(err: RequestResetTokenError): readonly PreactContent[] {
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
