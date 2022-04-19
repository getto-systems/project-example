import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldError, form } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"
import { icon_spinner } from "../../../../../x_content/icon"
import { iconHtml } from "../../../../../core/x_preact/design/icon"

import { InputAuthRoles } from "../../input/x_preact/roles"
import { EditButton } from "../../../../../core/x_preact/button/edit_button"
import { ResetButton } from "../../../../../core/x_preact/button/reset_button"
import { CloseButton } from "../../../../../core/x_preact/button/close_button"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { ModifyAuthUserAccountAction } from "../action"

import { ModifyAuthUserAccountError, ModifyAuthUserAccountFields } from "../data"
import { LoginId } from "../../../login_id/kernel/data"
import { AuthRole } from "../../../kernel/data"
import { ChangeButton } from "../../../../../core/x_preact/button/change_button"

type Props = Readonly<{
    user: Readonly<{ loginId: LoginId; grantedRoles: readonly AuthRole[] }>
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
                h(InputAuthRoles, {
                    user: props.user,
                    editable: props.editable,
                    field: props.modify.grantedRoles,
                }),
            ],
            footer: editableState.isEditable
                ? [
                      buttons({
                          left: submitButton(),
                          right: resetButton(),
                      }),
                      ...validationMessage(),
                      ...message(),
                      buttons({
                          right: closeButton(),
                      }),
                  ]
                : editButton(),
        }),
    )

    function editButton(): VNode {
        return h(EditButton, { onClick, isSuccess: state.type === "success" })

        function onClick(e: Event) {
            e.preventDefault()
            props.modify.reset(props.user)
            props.editable.open()
        }
    }

    function submitButton(): VNode {
        return h(ChangeButton, {
            isConnecting: state.type === "try" || state.type === "take-longtime",
            validateState,
            observeState,
            onClick,
        })

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
        return h(ResetButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.modify.reset(props.user)
        }
    }

    function closeButton(): VNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.editable.close()
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
                        html`${iconHtml(icon_spinner)} 変更に時間がかかっています`,
                        html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                    ]),
                ]

            case "failed":
                return [fieldError(modifyError(state.err))]
        }
    }
}

function modifyError(err: ModifyAuthUserAccountError): readonly VNodeContent[] {
    switch (err.type) {
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
