import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    fieldHelp_error,
    form,
} from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, validationMessage } from "../../../../../common/x_preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { StaticLoginIdField } from "../../../login_id/input/x_preact/static"
import { AuthUserMemoField } from "../../input/memo/x_preact/input"
import { GrantedRolesField } from "../../input/granted_roles/x_preact/input"
import { EditButton } from "../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../common/x_preact/button/edit_success_button"
import { ResetButton } from "../../../../../common/x_preact/button/reset_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"
import { ChangeButton } from "../../../../../common/x_preact/button/change_button"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { ModifyAuthUserAccountAction } from "../action"

import { ModifyAuthUserAccountError, ModifyAuthUserAccountFields } from "../data"
import { LoginId } from "../../../login_id/kernel/data"
import { AuthRole } from "../../../kernel/data"
import { AuthUserMemo } from "../../kernel/data"

type Props = Readonly<{
    user: Readonly<{ loginId: LoginId; grantedRoles: readonly AuthRole[]; memo: AuthUserMemo }>
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
                h(StaticLoginIdField, { user: props.user }),
                h(AuthUserMemoField, {
                    edit: {
                        data: props.user,
                        editable: props.editable,
                    },
                    field: props.modify.memo,
                }),
                h(GrantedRolesField, {
                    edit: {
                        data: props.user,
                        editable: props.editable,
                    },
                    field: props.modify.grantedRoles,
                }),
            ],
            footer: editableState.isEditable
                ? [
                      buttons({
                          left: submitButton(),
                          right: resetButton(),
                      }),
                      ...validationMessage(validateState),
                      ...message(),
                      buttons({
                          right: closeButton(),
                      }),
                  ]
                : editButton(),
        }),
    )

    function editButton(): VNode {
        if (state.type === "success") {
            return h(EditSuccessButton, { onClick })
        } else {
            return h(EditButton, { onClick })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.modify.reset(props.user)
            props.editable.open()
        }
    }

    function submitButton(): VNode {
        return h(ChangeButton, {
            isConnecting: state.type === "try",
            validateState,
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.modify.submit(props.user, onSuccess)

            function onSuccess(data: ModifyAuthUserAccountFields) {
                props.editable.close()
                props.onSuccess(data)
            }
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

    function message(): readonly VNode[] {
        switch (state.type) {
            case "initial":
            case "success":
                return []

            case "try":
                if (state.hasTakenLongtime) {
                    return [takeLongtimeField("変更")]
                }
                return []

            case "failed":
                return [fieldHelp_error(modifyError(state.err))]
        }
    }
}

function modifyError(err: ModifyAuthUserAccountError): readonly VNodeContent[] {
    switch (err.type) {
        case "conflict":
            return ["他で変更がありました", "一旦リロードしてやり直してください"]

        case "not-found":
            return ["データが見つかりませんでした", "一旦リロードしてやり直してください"]

        case "invalid":
            return ["データが正しくありません", "一旦リロードしてやり直してください"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}により変更に失敗しました`,
                ...reason.detail,
            ])
    }
}
