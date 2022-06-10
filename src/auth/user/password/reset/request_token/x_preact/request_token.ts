import { h, VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { remoteCommonErrorReason } from "../../../../../../z_lib/ui/remote/x_error/reason"

import { useApplicationAction } from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../../z_vendor/getto-css/preact/design/form"
import { loginBox } from "../../../../../../z_vendor/getto-css/preact/layout/login"

import { siteInfo } from "../../../../../../x_content/site"
import { icon_change } from "../../../../../../x_content/icon"
import { signNav } from "../../../../../sign/nav/x_preact/nav"
import { takeLongtimeField, validationMessage } from "../../../../../../common/x_preact/design/form"

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
    const state = useApplicationAction(props.requestToken)
    const validateState = useApplicationAction(props.requestToken.validate)
    const observeState = useApplicationAction(props.requestToken.observe)

    return loginBox(siteInfo, {
        title: "パスワードリセット",
        ...(state.type === "success"
            ? {
                  body: [
                      html`<p>トークンの送信が完了しました</p>`,
                      html`<p>
                          メールからパスワードのリセットができます<br />
                          メールを確認してください
                      </p>`,
                  ],
                  footer: footerLinks(),
              }
            : {
                  form: true,
                  body: [
                      h(LoginIdField, {
                          field: props.requestToken.loginId,
                          help: ["このログインIDに設定された送信先にリセットトークンを送信します"],
                      }),
                      buttons({
                          left: sendButton(),
                          right: clearButton(),
                      }),
                  ],
                  footer: [footerLinks(), ...validationMessage(validateState), ...message()],
              }),
    })

    function clearButton(): VNode {
        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.clear()
        }
    }

    function sendButton(): VNode {
        return h(SendButton, {
            label: "トークン送信",
            icon: icon_change,
            isConnecting: state.type === "try",
            validateState,
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.submit(onSuccess)

            function onSuccess() {
                // noop
            }
        }
    }

    function message(): readonly VNode[] {
        switch (state.type) {
            case "initial":
            case "success":
                return []

            case "try":
                if (state.hasTakenLongtime) {
                    return [takeLongtimeField("トークンの送信")]
                }
                return []

            case "failed":
                return [fieldHelp_error(requestTokenError(state.err))]
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
