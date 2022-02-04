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
import { spinner } from "../../../../../../core/x_preact/design/icon"
import { signNav } from "../../../../../sign/nav/x_preact/nav"

import { InputLoginIDEntry } from "../../../../login_id/input/x_preact/input"

import { RequestResetTokenError } from "../data"
import { ApplicationView } from "../../../../../../z_vendor/getto-application/action/action"
import { RequestResetTokenAction, RequestResetTokenState } from "../action"
import { ValidateBoardActionState } from "../../../../../../z_vendor/getto-application/board/validate_board/action"

export function RequestResetTokenEntry(view: ApplicationView<RequestResetTokenAction>): VNode {
    const action = useApplicationView(view)
    return h(RequestResetTokenComponent, {
        requestToken: action,
        state: useApplicationAction(action),
        validate: useApplicationAction(action.validate),
    })
}

const title = "パスワードリセット"

type Props = Readonly<{
    requestToken: RequestResetTokenAction
    state: RequestResetTokenState
    validate: ValidateBoardActionState
}>
export function RequestResetTokenComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state }: Props): VNode {
        switch (state.type) {
            case "initial-request-token":
                return requestTokenForm({ state: "start" })

            case "failed-to-request-token":
                return requestTokenForm({
                    state: "start",
                    err: requestTokenError(state.err),
                })

            case "try-to-request-token":
                return requestTokenForm({ state: "connecting" })

            case "take-longtime-to-request-token":
                return takeLongtimeMessage()

            case "succeed-to-request-token":
                return successMessage()
        }
    }

    type FormState = "start" | "connecting"

    type FormContent = Readonly<{ state: FormState }> & Partial<{ err: readonly VNodeContent[] }>

    function requestTokenForm(content: FormContent): VNode {
        return form(
            loginBox(siteInfo, {
                title,
                body: [
                    h(InputLoginIDEntry, {
                        field: props.requestToken.loginID,
                        help: ["このログインIDに設定された送信先にリセットトークンを送信します"],
                    }),
                    buttons({ left: button(), right: clearButton() }),
                ],
                footer: [footerLinks(), error()],
            }),
        )

        function clearButton() {
            const label = "入力内容をクリア"
            switch (props.validate) {
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

        function button() {
            switch (content.state) {
                case "start":
                    return startSessionButton()

                case "connecting":
                    return connectingButton()
            }

            function startSessionButton() {
                const label = "トークン送信"

                switch (props.validate) {
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
                    label: html`トークンを送信しています ${spinner}`,
                })
            }
        }

        function error() {
            if (content.err) {
                return fieldError(content.err)
            }
            return ""
        }
    }
    function takeLongtimeMessage() {
        return loginBox(siteInfo, {
            title,
            body: [
                html`<p>${spinner} トークンの送信に時間がかかっています</p>`,
                html`<p>
                    30秒以上かかる場合は何かがおかしいので、
                    <br />
                    お手数ですが管理者に連絡お願いします
                </p>`,
            ],
            footer: footerLinks(),
        })
    }
    function successMessage() {
        return loginBox(siteInfo, {
            title,
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

    function footerLinks() {
        return buttons({ left: privacyPolicyLink(), right: loginLink() })
    }
    function privacyPolicyLink() {
        return signNav(props.requestToken.link.getNav_static_privacyPolicy())
    }
    function loginLink(): VNode {
        return signNav(props.requestToken.link.getNav_password_authenticate())
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
