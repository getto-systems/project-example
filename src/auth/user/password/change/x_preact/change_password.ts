import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_success } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"
import { icon_spinner } from "../../../../../core/x_preact/design/icon"

import { InputPasswordEntry } from "../../input/x_preact/input"

import { ChangePasswordError } from "../data"
import { ChangePasswordAction, ChangePasswordState } from "../action"
import { ValidateBoardActionState } from "../../../../../z_vendor/getto-application/board/validate_board/action"

type EntryProps = Readonly<{
    change: ChangePasswordAction
}>
export function ChangePasswordEntry({ change }: EntryProps): VNode {
    return h(ChangePasswordComponent, {
        change,
        state: useApplicationAction(change),
        validate: useApplicationAction(change.validate),
    })
}

type Props = EntryProps &
    Readonly<{
        state: ChangePasswordState
        validate: ValidateBoardActionState
    }>
export function ChangePasswordComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, validate }: Props): VNode {
        switch (state.type) {
            case "initial-change-password":
                return buttonBox({ type: "initial" })

            case "input-password":
                return formBox({ type: validate })

            case "try-to-change-password":
                return formBox({ type: "connecting" })

            case "take-longtime-to-change-password":
                return formBox({ type: "take-longtime" })

            case "succeed-to-change-password":
                return buttonBox({ type: "success" })

            case "failed-to-change-password":
                return formBox({ type: validate, err: changePasswordError(state.err) })
        }
    }

    type ButtonContentType = "initial" | "success"
    type ButtonContent = Readonly<{ type: ButtonContentType }>
    function buttonBox(state: ButtonContent): VNode {
        return form(box(content()))

        type BoxContent =
            | Readonly<{ title: VNodeContent; body: VNodeContent }>
            | Readonly<{ title: VNodeContent; body: VNodeContent; footer: VNodeContent }>
        function content(): BoxContent {
            switch (state.type) {
                case "initial":
                    return {
                        title: title(),
                        body: openButton(),
                    }

                case "success":
                    return {
                        title: title(),
                        body: openButton(),
                        footer: notice_success(["パスワードを変更しました"]),
                    }
            }
        }
        function title() {
            return "パスワード変更"
        }
        function openButton(): VNode {
            return button_send({ state: "normal", label: "変更", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.change.open()
            }
        }
    }

    type FormContentType = "initial" | "valid" | "invalid" | "connecting" | "take-longtime"
    type FormContent = Readonly<{ type: FormContentType }> &
        Partial<{ err: readonly VNodeContent[] }>
    function formBox(state: FormContent): VNode {
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
                    buttons({
                        right: closeButton(),
                    }),
                ],
            }),
        )

        function submitButton(): VNode {
            const label = "パスワード変更"

            switch (state.type) {
                case "initial":
                    return button_send({ state: "normal", label, onClick })

                case "valid":
                    return button_send({ state: "confirm", label, onClick })

                case "invalid":
                    return button_disabled({ label })

                case "connecting":
                case "take-longtime":
                    return button_send({
                        state: "connect",
                        label: html`パスワード変更中 ${icon_spinner}`,
                    })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.change.submit()
            }
        }

        function clearButton(): VNode {
            const label = "入力内容をクリア"
            switch (state.type) {
                case "initial":
                    return button_disabled({ label })

                case "connecting":
                case "take-longtime":
                    return EMPTY_CONTENT

                case "invalid":
                case "valid":
                    return button_undo({ label, onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.change.clear()
            }
        }
        function closeButton(): VNode {
            return button_undo({ label: "閉じる", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.change.close()
            }
        }

        function message(): readonly VNode[] {
            if (state.err) {
                return [fieldError(state.err)]
            }

            switch (state.type) {
                case "initial":
                case "valid":
                case "connecting":
                    return []

                case "take-longtime":
                    return [
                        fieldError([
                            html`${icon_spinner} パスワード変更中です`,
                            html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                        ]),
                    ]

                case "invalid":
                    return [fieldError(["正しく入力されていません"])]
            }
        }
    }
}

function changePasswordError(err: ChangePasswordError): readonly VNodeContent[] {
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
