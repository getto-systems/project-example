import { h, VNode } from "preact"
import { html } from "htm/preact"

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
import { iconHtml, icon_save, icon_spinner } from "../../../../../core/x_preact/design/icon"

import { InputResetTokenDestinationEntry } from "../../input/x_preact/destination"
import { InputGrantedRolesComponent } from "../../input/x_preact/granted_roles"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { ModifyAuthUserAccountAction, ModifyAuthUserAccountState } from "../action"
import { ValidateBoardState } from "../../../../../z_vendor/getto-application/board/validate_board/action"
import {
    EditableBoardAction,
    EditableBoardState,
} from "../../../../../z_vendor/getto-application/board/editable/action"
import { ObserveBoardState } from "../../../../../z_vendor/getto-application/board/observe_board/action"

import { AuthUserAccountBasket } from "../../kernel/data"
import { ModifyAuthUserAccountError } from "../data"

type EntryProps = Readonly<{
    user: AuthUserAccountBasket
    editable: EditableBoardAction
    modify: ModifyAuthUserAccountAction
}>
export function ModifyAuthUserAccountEntry({ user, editable, modify }: EntryProps): VNode {
    return h(ModifyAuthUserAccountComponent, {
        user,
        editable,
        modify,
        state: useApplicationAction(modify),
        editableState: useApplicationAction(editable),
        validateState: useApplicationAction(modify.validate),
        observeState: useApplicationAction(modify.observe),
    })
}

type Props = EntryProps &
    Readonly<{
        state: ModifyAuthUserAccountState
        editableState: EditableBoardState
        validateState: ValidateBoardState
        observeState: ObserveBoardState
    }>
export function ModifyAuthUserAccountComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, editableState, validateState }: Props): VNode {
        if (editableState.isEditable) {
            switch (state.type) {
                case "initial-modify":
                case "succeed-to-modify":
                    return formBox({ type: validateState })

                case "try-to-modify":
                    return formBox({ type: "connecting" })

                case "take-longtime-to-modify":
                    return formBox({ type: "take-longtime" })

                case "failed-to-modify":
                    return formBox({ type: validateState, err: modifyError(state.err) })
            }
        } else {
            return buttonBox({
                type: state.type === "succeed-to-modify" ? "success" : "initial",
            })
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
            return {
                title: BOX_TITLE,
                body: openButton(),
                footer:
                    state.type === "success"
                        ? notice_success(["基本情報を変更しました"])
                        : undefined,
            }
        }
        function openButton(): VNode {
            return button_send({ state: "normal", label: "変更", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.modify.reset(props.user)
                props.editable.open()
            }
        }
    }

    type FormContentType = "initial" | "valid" | "invalid" | "connecting" | "take-longtime"
    type FormContent = Readonly<{ type: FormContentType }> &
        Partial<{ err: readonly VNodeContent[] }>
    function formBox(state: FormContent): VNode {
        return form(
            box({
                title: BOX_TITLE,
                body: [
                    h(InputGrantedRolesComponent, {
                        field: props.modify.grantedRoles,
                    }),
                    h(InputResetTokenDestinationEntry, {
                        field: props.modify.resetTokenDestination,
                    }),
                ],
                footer: [
                    buttons({
                        left: submitButton(),
                        right: resetButton(),
                    }),
                    ...message(),
                    buttons({
                        right: closeButton(),
                    }),
                ],
            }),
        )

        function submitButton(): VNode {
            switch (state.type) {
                case "initial":
                    if (props.observeState.hasChanged) {
                        return button_send({ state: "confirm", label: LABEL_STATIC, onClick })
                    } else {
                        return button_send({ state: "normal", label: LABEL_STATIC, onClick })
                    }

                case "valid":
                    if (props.observeState.hasChanged) {
                        return button_send({ state: "confirm", label: LABEL_STATIC, onClick })
                    } else {
                        return button_send({ state: "normal", label: LABEL_STATIC, onClick })
                    }

                case "invalid":
                    return button_disabled({ label: LABEL_STATIC })

                case "connecting":
                case "take-longtime":
                    return button_send({ state: "connect", label: LABEL_CONNECT })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.modify.submit(props.user).then((state) => {
                    switch (state.type) {
                        case "succeed-to-modify":
                            props.editable.close()
                    }
                })
            }
        }

        function resetButton(): VNode {
            switch (state.type) {
                case "initial":
                    if (props.observeState.hasChanged) {
                        return button_undo({ label: LABEL_RESET, onClick })
                    } else {
                        return button_disabled({ label: LABEL_RESET })
                    }

                case "connecting":
                case "take-longtime":
                    return EMPTY_CONTENT

                case "invalid":
                case "valid":
                    if (props.observeState.hasChanged) {
                        return button_undo({ label: LABEL_RESET, onClick })
                    } else {
                        return button_disabled({ label: LABEL_RESET })
                    }
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.modify.reset(props.user)
            }
        }
        function closeButton(): VNode {
            return button_undo({ label: "閉じる", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.editable.close()
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
                            html`${icon_spinner} 基本情報変更中です`,
                            html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                        ]),
                    ]

                case "invalid":
                    return [fieldError(["正しく入力されていません"])]
            }
        }
    }
}

function modifyError(err: ModifyAuthUserAccountError): readonly VNodeContent[] {
    switch (err.type) {
        case "validation-error":
            return ["正しく入力してください"]

        case "invalid-granted-role":
            return ["権限が正しくありません"]

        case "invalid-reset-token-destination-email":
            return ["メールアドレスが正しくありません"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}により基本情報変更に失敗しました`,
                ...reason.detail,
            ])
    }
}

const BOX_TITLE = "基本情報"

const LABEL_STATIC = html`変更 ${iconHtml(icon_save)}`
const LABEL_CONNECT = html`変更 ${iconHtml(icon_spinner)}`
const LABEL_RESET = "変更前に戻す"

const EMPTY_CONTENT = html``
