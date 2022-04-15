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

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"
import { iconHtml } from "../../../../../core/x_preact/design/icon"
import { icon_save, icon_spinner } from "../../../../../x_content/icon"

import { InputGrantedRoles } from "../../input/x_preact/granted_roles"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { ModifyAuthUserAccountAction } from "../action"

import { ModifyAuthUserAccountError, ModifyAuthUserAccountFields } from "../data"
import { LoginId } from "../../../login_id/kernel/data"
import { GrantedAuthRole } from "../../../kernel/data"
import { SuccessButton } from "../../../../../core/x_preact/design/button"

type Props = Readonly<{
    user: Readonly<{ loginId: LoginId; grantedRoles: readonly GrantedAuthRole[] }>
    editable: EditableBoardAction
    modify: ModifyAuthUserAccountAction
    onSuccess: { (fields: ModifyAuthUserAccountFields): void }
}>
export function ModifyAuthUserAccount(props: Props): VNode {
    const state = useApplicationAction(props.modify)
    const editableState = useApplicationAction(props.editable)
    const validateState = useApplicationAction(props.modify.validate)
    const observeState = useApplicationAction(props.modify.observe)

    return form(
        box({
            title: "基本情報",
            body: [
                h(InputGrantedRoles, {
                    user: props.user,
                    editable: props.editable,
                    field: props.modify.grantedRoles,
                }),
            ],
            footer: editableState.isEditable ? editButtons() : openButton(),
        }),
    )

    function openButton(): VNode {
        return h(SuccessButton, { label: "変更", onClick, isSuccess: state.type === "success" })

        function onClick(e: Event) {
            e.preventDefault()
            props.modify.reset(props.user)
            props.editable.open()
        }
    }

    function editButtons(): VNodeContent {
        return [
            buttons({
                left: submitButton(),
                right: resetButton(),
            }),
            ...message(),
            buttons({
                right: closeButton(),
            }),
        ]

        function submitButton(): VNode {
            switch (state.type) {
                case "initial":
                case "failed":
                case "success":
                    if (validateState === "invalid") {
                        return button_disabled({ label: LABEL_STATIC })
                    }
                    return button_send({
                        state: observeState.hasChanged ? "confirm" : "normal",
                        label: LABEL_STATIC,
                        onClick,
                    })

                case "try":
                case "take-longtime":
                    return button_send({ state: "connect", label: LABEL_CONNECT })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.modify.submit(props.user).then((state) => {
                    if (state.type === "success") {
                        props.editable.close()
                        props.onSuccess(state.data)
                    }
                })
            }
        }

        function resetButton(): VNode {
            switch (state.type) {
                case "initial":
                case "success":
                case "failed":
                    if (observeState.hasChanged) {
                        return button_undo({ label: LABEL_RESET, onClick })
                    } else {
                        return button_disabled({ label: LABEL_RESET })
                    }

                case "try":
                case "take-longtime":
                    return EMPTY_CONTENT
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
            switch (state.type) {
                case "initial":
                case "success":
                    if (validateState === "invalid") {
                        return [fieldError(["正しく入力されていません"])]
                    }
                    return []

                case "try":
                    return []

                case "take-longtime":
                    return [
                        fieldError([
                            html`${icon_spinner} 基本情報変更中です`,
                            html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                        ]),
                    ]

                case "failed":
                    return [fieldError(modifyError(state.err))]
            }
        }
    }
}

function modifyError(err: ModifyAuthUserAccountError): readonly VNodeContent[] {
    switch (err.type) {
        case "validation-error":
            return ["正しく入力してください"]

        case "conflict":
            return ["他で変更がありました", "一旦リロードしてやり直してください"]

        case "not-found":
            return ["ユーザーが見つかりませんでした", "一旦リロードしてやり直してください"]

        case "invalid":
            return ["データが正しくありません", "一旦リロードしてやり直してください"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}により変更に失敗しました`,
                ...reason.detail,
            ])
    }
}

const LABEL_STATIC = html`変更 ${iconHtml(icon_save)}`
const LABEL_CONNECT = html`変更 ${iconHtml(icon_spinner)}`
const LABEL_RESET = "変更前に戻す"

const EMPTY_CONTENT = html``
