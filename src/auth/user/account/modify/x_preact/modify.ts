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
import { InputGrantedRolesEntry } from "../../input/x_preact/granted_roles"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { ModifyAuthUserAccountAction } from "../action"
import { DetailAuthUserAccountAction } from "../../search/action"

import { AuthUserAccountBasket } from "../../kernel/data"
import { ModifyAuthUserAccountError } from "../data"
import { v_small } from "../../../../../z_vendor/getto-css/preact/design/alignment"

type Props = Readonly<{
    user: AuthUserAccountBasket
    editable: EditableBoardAction
    detail: DetailAuthUserAccountAction
    modify: ModifyAuthUserAccountAction
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
                h(InputGrantedRolesEntry, {
                    user: props.user,
                    editable: props.editable,
                    field: props.modify.grantedRoles,
                }),
                h(InputResetTokenDestinationEntry, {
                    user: props.user,
                    editable: props.editable,
                    field: props.modify.resetTokenDestination,
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
                props.modify.reset(props.user)
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
                        props.detail.update(state.data)
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

        case "invalid-granted-role":
            return ["権限が正しくありません", "一旦リロードしてやり直してください"]

        case "invalid-reset-token-destination-email":
            return ["メールアドレスが正しくありません"]

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
