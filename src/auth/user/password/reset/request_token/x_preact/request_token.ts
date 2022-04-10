import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../../z_lib/ui/remote/x_error/reason"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../../z_vendor/getto-css/preact/design/form"
import { loginBox } from "../../../../../../z_vendor/getto-css/preact/layout/login"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"
import { siteInfo } from "../../../../../../x_content/site"
import { icon_spinner } from "../../../../../../x_content/icon"
import { signNav } from "../../../../../sign/nav/x_preact/nav"

import { InputLoginId } from "../../../../login_id/input/x_preact/input"

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

    const content = {
        title: "パスワードリセット",
    }

    switch (state.type) {
        case "initial":
        case "try":
        case "failed":
            return form(
                loginBox(siteInfo, {
                    ...content,
                    body: [
                        h(InputLoginId, {
                            field: props.requestToken.loginId,
                            help: [
                                "このログインIDに設定された送信先にリセットトークンを送信します",
                            ],
                        }),
                        buttons({
                            left: state.type === "try" ? connectingButton() : sendButton(),
                            right: clearButton(),
                        }),
                    ],
                    footer: [
                        footerLinks(),
                        state.type === "failed" ? fieldError(requestTokenError(state.err)) : "",
                    ],
                }),
            )

        case "take-longtime":
            return loginBox(siteInfo, {
                ...content,
                body: [
                    html`<p>${icon_spinner} トークンの送信に時間がかかっています</p>`,
                    html`<p>
                        30秒以上かかる場合は何かがおかしいので、
                        <br />
                        お手数ですが管理者に連絡お願いします
                    </p>`,
                ],
                footer: footerLinks(),
            })

        case "success":
            return loginBox(siteInfo, {
                ...content,
                body: [
                    html`<p>トークンの送信が完了しました</p>`,
                    html`<p>
                        メールからパスワードのリセットができます<br />
                        メールを確認してください
                    </p>`,
                ],
                footer: footerLinks(),
            })
    }

    function clearButton() {
        const label = "入力内容をクリア"
        switch (validateState) {
            case "initial":
                return button_disabled({ label })

            case "invalid":
            case "valid":
                return button_undo({ label, onClick })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.clear()
        }
    }

    function sendButton() {
        const label = "トークン送信"

        switch (validateState) {
            case "initial":
                return button_send({ state: "normal", label, onClick })

            case "valid":
                return button_send({ state: "confirm", label, onClick })

            case "invalid":
                return button_disabled({ label })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.submit()
        }
    }
    function connectingButton(): VNode {
        return button_send({
            state: "connect",
            label: html`トークンを送信しています ${icon_spinner}`,
        })
    }

    function footerLinks() {
        return buttons({ left: privacyPolicyLink(), right: loginLink() })
    }
    function privacyPolicyLink() {
        return signNav(props.link.getNav_static_privacyPolicy())
    }
    function loginLink(): VNode {
        return signNav(props.link.getNav_password_authenticate())
    }
}

function requestTokenError(err: RequestResetTokenError): readonly VNodeContent[] {
    switch (err.type) {
        case "validation-error":
            return ["正しく入力してください"]

        case "invalid-reset":
            return ["ログインIDが登録されていないか、トークンの送信先が登録されていません"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}によりトークンの送信に失敗しました`,
                ...reason.detail,
            ])
    }
}
