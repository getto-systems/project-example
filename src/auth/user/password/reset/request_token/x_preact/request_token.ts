import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../../z_lib/ui/remote/x_error/reason"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldError } from "../../../../../../z_vendor/getto-css/preact/design/form"
import { loginBox } from "../../../../../../z_vendor/getto-css/preact/layout/login"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"
import { siteInfo } from "../../../../../../x_content/site"
import { icon_change, icon_spinner } from "../../../../../../x_content/icon"
import { signNav } from "../../../../../sign/nav/x_preact/nav"
import { iconHtml } from "../../../../../../core/x_preact/design/icon"

import { InputLoginId } from "../../../../login_id/input/x_preact/input"
import { ClearChangesButton } from "../../../../../../core/x_preact/button/clear_changes_button"
import { SendButton } from "../../../../../../core/x_preact/button/send_button"

import { ApplicationView } from "../../../../../../z_vendor/getto-application/action/action"
import { RequestResetTokenAction } from "../action"
import { SignLink } from "../../../../../sign/nav/action"

import { RequestResetTokenError } from "../data"

type Props = Readonly<{
    link: SignLink
    requestToken: ApplicationView<RequestResetTokenAction>
}>
export function RequestResetToken(viewProps: Props): VNode {
    const props = {
        link: viewProps.link,
        requestToken: useApplicationView(viewProps.requestToken),
    }
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
                      h(InputLoginId, {
                          field: props.requestToken.loginId,
                          help: ["このログインIDに設定された送信先にリセットトークンを送信します"],
                      }),
                      buttons({
                          left: sendButton(),
                          right: clearButton(),
                      }),
                  ],
                  footer: [footerLinks(), ...validationMessage(), ...message()],
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
            isConnecting: state.type === "try" || state.type === "take-longtime",
            validateState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.submit()
        }
    }

    function validationMessage(): readonly VNode[] {
        switch (validateState) {
            case "initial":
            case "valid":
                return []

            case "invalid":
                return [fieldError(["正しく入力されていません"])]
        }
    }
    function message(): readonly VNode[] {
        switch (state.type) {
            case "initial":
            case "success":
            case "try":
                return []

            case "take-longtime":
                return [
                    fieldError([
                        html`${iconHtml(icon_spinner)} トークンの送信に時間がかかっています`,
                        html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                    ]),
                ]

            case "failed":
                return [fieldError(requestTokenError(state.err))]
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
