import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
} from "../../../../../../../ui/vendor/getto-css/preact/design/form"
import { loginBox } from "../../../../../../../ui/vendor/getto-css/preact/layout/login"

import { VNodeContent } from "../../../../../../example/_ui/x_preact/design/common"
import { siteInfo } from "../../../../../../example/site"
import { spinner } from "../../../../../../example/_ui/x_preact/design/icon"
import { appendScript } from "../../../../../_ui/common/x_preact/script"
import { signNav } from "../../../../../_ui/common/nav/x_preact/nav"

import { ApplicationErrorComponent } from "../../../../../../avail/_ui/x_preact/application_error"
import { InputLoginIDEntry } from "../../../../../login_id/_ui/action_input/x_preact/input"
import { InputPasswordEntry } from "../../../../_ui/action_input/x_preact/input"

import { ResetPasswordView, ResetPasswordResource, ResetPasswordResourceState } from "../resource"

import { ResetPasswordError } from "../../reset/data"
import { remoteCommonError } from "../../../../../../z_details/_ui/remote/helper"

export function ResetPasswordEntry(view: ResetPasswordView): VNode {
    const resource = useApplicationView(view)
    return h(ResetPasswordComponent, {
        ...resource,
        state: {
            core: useApplicationAction(resource.reset.core),
            form: useApplicationAction(resource.reset.form.validate),
        },
    })
}

type Props = ResetPasswordResource & ResetPasswordResourceState
export function ResetPasswordComponent(props: Props): VNode {
    useLayoutEffect(() => {
        // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
        switch (props.state.core.type) {
            case "try-to-load":
                if (!props.state.core.scriptPath.valid) {
                    props.reset.core.loadError({
                        type: "infra-error",
                        err: `スクリプトのロードに失敗しました: ${props.state.core.type}`,
                    })
                    break
                }
                appendScript(props.state.core.scriptPath.value, (script) => {
                    script.onerror = () => {
                        props.reset.core.loadError({
                            type: "infra-error",
                            err: `スクリプトのロードに失敗しました: ${props.state.core.type}`,
                        })
                    }
                })
                break
        }
    }, [props.reset.core, props.state.core])

    switch (props.state.core.type) {
        case "initial-reset":
            return resetForm({ state: "reset" })

        case "failed-to-reset":
            return resetForm({ state: "reset", error: resetError(props.state.core.err) })

        case "try-to-reset":
            return resetForm({ state: "connecting" })

        case "take-longtime-to-reset":
            return takeLongtimeMessage()

        case "try-to-load":
            // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
            return EMPTY_CONTENT

        case "succeed-to-renew":
        case "authn-not-expired":
        case "required-to-login":
        case "failed-to-renew":
            // これらはスクリプトがロードされた後に発行される
            // したがって、un-mount されているのでここには来ない
            return EMPTY_CONTENT

        case "repository-error":
        case "load-error":
            return h(ApplicationErrorComponent, { err: props.state.core.err.err })
    }

    type ResetFormState = "reset" | "connecting"

    type ResetFormContent = ResetFormContent_base | (ResetFormContent_base & ResetFormContent_error)
    type ResetFormContent_base = Readonly<{ state: ResetFormState }>
    type ResetFormContent_error = Readonly<{ error: VNodeContent[] }>

    function resetTitle() {
        return "パスワードリセット"
    }

    function resetForm(content: ResetFormContent): VNode {
        return form(
            loginBox(siteInfo, {
                title: resetTitle(),
                body: [
                    h(InputLoginIDEntry, {
                        field: props.reset.form.loginID,
                        help: ["最初に入力したログインIDを入力してください"],
                    }),
                    h(InputPasswordEntry, {
                        field: props.reset.form.password,
                        help: ["新しいパスワードを入力してください"],
                    }),
                    buttons({ left: button(), right: clearButton() }),
                ],
                footer: [footerLinks(), error()],
            }),
        )

        function clearButton() {
            const label = "入力内容をクリア"
            switch (props.state.form) {
                case "initial":
                    return button_disabled({ label })

                case "invalid":
                case "valid":
                    return button_undo({ label, onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.reset.form.clear()
            }
        }

        function button() {
            switch (content.state) {
                case "reset":
                    return resetButton()

                case "connecting":
                    return connectingButton()
            }

            function resetButton() {
                const label = "パスワードリセット"

                switch (props.state.form) {
                    case "initial":
                        return button_send({ state: "normal", label, onClick })

                    case "valid":
                        return button_send({ state: "confirm", label, onClick })

                    case "invalid":
                        return button_disabled({ label })
                }

                function onClick(e: Event) {
                    e.preventDefault()
                    props.reset.core.submit(props.reset.form.validate.get())
                }
            }
            function connectingButton(): VNode {
                return button_send({
                    state: "connect",
                    label: html`パスワードをリセットしています ${spinner}`,
                })
            }
        }

        function error() {
            if ("error" in content) {
                return fieldError(content.error)
            }
            return ""
        }
    }
    function takeLongtimeMessage() {
        return loginBox(siteInfo, {
            title: resetTitle(),
            body: [
                html`<p>${spinner} リセットに時間がかかっています</p>`,
                html`<p>
                    30秒以上かかる場合は何かがおかしいので、
                    <br />
                    お手数ですが管理者に連絡お願いします
                </p>`,
            ],
            footer: footerLinks(),
        })
    }

    function footerLinks() {
        return buttons({ left: privacyPolicyLink(), right: sendLink() })
    }
    function privacyPolicyLink() {
        return signNav(props.link.getNav_static_privacyPolicy())
    }
    function sendLink() {
        return signNav(props.link.getNav_password_reset_requestToken_retry())
    }
}

function resetError(err: ResetPasswordError): VNodeContent[] {
    switch (err.type) {
        case "validation-error":
            return ["正しく入力してください"]

        case "empty-reset-token":
            return ["リセットトークンが見つかりませんでした"]

        case "invalid-reset":
            return ["ログインIDが最初に入力したものと違うか、有効期限が切れています"]

        case "already-reset":
            return [
                "すでにリセット済みです",
                "もう一度リセットする場合はトークンの送信からやり直してください",
            ]

        default:
            return remoteCommonError(err, (reason) => [
                `${reason.message}によりリセットに失敗しました`,
                ...reason.detail,
            ])
    }
}

function form(content: VNodeContent) {
    return html`<form>${content}</form>`
}

const EMPTY_CONTENT = html``
