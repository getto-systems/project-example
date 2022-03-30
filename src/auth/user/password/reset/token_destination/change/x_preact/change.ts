import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../../../z_vendor/getto-css/preact/design/box"
import { notice_success } from "../../../../../../../z_vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../../../../z_vendor/getto-css/preact/design/alignment"

import { VNodeContent } from "../../../../../../../z_lib/ui/x_preact/common"
import { buttonLabel, icon_save, icon_spinner } from "../../../../../../../x_content/icon"

import { ResetTokenDestinationField } from "../../input/x_preact/destination"

import { remoteCommonErrorReason } from "../../../../../../../z_lib/ui/remote/x_error/reason"

import { EditableBoardAction } from "../../../../../../../z_vendor/getto-application/board/editable/action"
import { ChangeResetTokenDestinationAction } from "../action"

import { ChangeResetTokenDestinationError } from "../data"
import { LoginId } from "../../../../../login_id/input/data"
import { ResetTokenDestination } from "../../kernel/data"

type Props = Readonly<{
    user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>
    editable: EditableBoardAction
    change: ChangeResetTokenDestinationAction
    onSuccess: { (destination: ResetTokenDestination): void }
}>
export function ChangeResetTokenDestination(props: Props): VNode {
    const state = useApplicationAction(props.change)
    const editableState = useApplicationAction(props.editable)
    const validateState = useApplicationAction(props.change.validate)
    const observeState = useApplicationAction(props.change.observe)

    return form(
        box({
            title: "パスワードリセット",
            body: [
                h(ResetTokenDestinationField, {
                    user: props.user,
                    editable: props.editable,
                    field: props.change.destination,
                }),
            ],
            footer: editableState.isEditable ? editButtons() : staticButtons(),
        }),
    )

    function staticButtons(): VNodeContent {
        return [openButton(), ...message()]

        function openButton(): VNode {
            return button_send({ state: "normal", label: "変更", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.change.reset(props.user.resetTokenDestination)
                props.editable.open()
            }
        }

        function message(): VNode[] {
            if (state.type === "success") {
                return [v_small(), notice_success(["変更完了しました"])]
            }
            return []
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
                        return button_disabled({ label: LABEL_CHANGE.static })
                    }
                    return button_send({
                        state: observeState.hasChanged ? "confirm" : "normal",
                        label: LABEL_CHANGE.static,
                        onClick,
                    })

                case "try":
                case "take-longtime":
                    return button_send({ state: "connect", label: LABEL_CHANGE.connect })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.change.submit(props.user).then((state) => {
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
                props.change.reset(props.user.resetTokenDestination)
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

function modifyError(err: ChangeResetTokenDestinationError): readonly VNodeContent[] {
    switch (err.type) {
        case "validation-error":
            return ["正しく入力してください"]

        case "conflict":
            return ["他で変更がありました", "一旦リロードしてやり直してください"]

        case "not-found":
            return ["ユーザーが見つかりませんでした", "一旦リロードしてやり直してください"]

        case "invalid-destination-type":
            return ["有効/無効を選択してください"]

        case "invalid-email":
            return ["メールアドレスが正しくありません"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}により変更に失敗しました`,
                ...reason.detail,
            ])
    }
}

const LABEL_CHANGE = buttonLabel("変更", icon_save)
const LABEL_RESET = "変更前に戻す"

const EMPTY_CONTENT = html``
