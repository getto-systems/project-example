import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../z_details/_ui/remote/reason"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../../ui/vendor/getto-css/preact/design/form"
import { box } from "../../../../../../ui/vendor/getto-css/preact/design/box"

import { VNodeContent } from "../../../../../example/_ui/x_preact/design/common"
import { spinner } from "../../../../../example/_ui/x_preact/design/icon"

import { InputPasswordEntry } from "../../action_input/x_preact/input"

import { ChangePasswordResource, ChangePasswordResourceState } from "../resource"

import { ChangePasswordError } from "../../change/data"
import { notice_success } from "../../../../../../ui/vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../../../ui/vendor/getto-css/preact/design/alignment"

export function ChangePasswordEntry({ change }: ChangePasswordResource): VNode {
    return h(ChangePasswordComponent, {
        change,
        state: useApplicationAction(change),
        validate: useApplicationAction(change.validate),
    })
}

type Props = ChangePasswordResource & ChangePasswordResourceState
export function ChangePasswordComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, validate }: ChangePasswordResourceState): VNode {
        switch (state.type) {
            case "initial-change-password":
                return changePasswordBox({ state: validate })

            case "try-to-change-password":
                return changePasswordBox({ state: "connecting" })

            case "take-longtime-to-change-password":
                return changePasswordBox({ state: "take-longtime" })

            case "succeed-to-change-password":
                return changePasswordBox({ state: "success" })

            case "failed-to-change-password":
                return changePasswordBox({ state: validate, err: changePasswordError(state.err) })
        }
    }

    type State = "initial" | "valid" | "invalid" | "connecting" | "take-longtime" | "success"

    type Content = Readonly<{ state: State }> | Readonly<{ state: State; err: VNodeContent[] }>

    function changePasswordBox(content: Content): VNode {
        return form(
            box({
                title: "パスワード変更",
                body: [
                    h(InputPasswordEntry, {
                        field: props.change.currentPassword,
                        title: "現在のパスワード",
                        help: ["変更前のパスワードを入力します"],
                    }),
                    h(InputPasswordEntry, {
                        field: props.change.newPassword,
                        title: "新しいパスワード",
                        help: ["今後はこのパスワードになります"],
                    }),
                ],
                footer: [
                    buttons({
                        left: submitButton(),
                        right: clearButton(),
                    }),
                    ...message(),
                ],
            }),
        )

        function submitButton(): VNode {
            const label = "パスワード変更"

            switch (content.state) {
                case "initial":
                    return button_send({ state: "normal", label, onClick })

                case "valid":
                    return button_send({ state: "confirm", label, onClick })

                case "success":
                case "invalid":
                    return button_disabled({ label })

                case "connecting":
                case "take-longtime":
                    return button_send({
                        state: "connect",
                        label: html`パスワード変更中 ${spinner}`,
                    })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.change.submit()
            }
        }

        function clearButton(): VNode {
            const label = "入力内容をクリア"
            switch (content.state) {
                case "initial":
                    return button_disabled({ label })

                case "connecting":
                case "take-longtime":
                    return EMPTY_CONTENT

                case "invalid":
                case "valid":
                case "success":
                    return button_undo({ label, onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.change.clear()
            }
        }

        function message(): VNode[] {
            if ("err" in content) {
                return [fieldError(content.err)]
            }

            switch (content.state) {
                case "initial":
                case "valid":
                case "connecting":
                    return []

                case "take-longtime":
                    return [
                        fieldError([
                            html`${spinner} パスワード変更中です`,
                            html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                        ]),
                    ]

                case "success":
                    return [v_small(), notice_success(["パスワードを変更しました"])]

                case "invalid":
                    return [fieldError(["正しく入力されていません"])]
            }
        }
    }
}

function changePasswordError(err: ChangePasswordError): VNodeContent[] {
    switch (err.type) {
        case "validation-error":
            return ["正しく入力してください"]

        case "invalid-password":
            return ["現在のパスワードが違います"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}により認証に失敗しました`,
                ...reason.detail,
            ])
    }
}

const EMPTY_CONTENT = html``
