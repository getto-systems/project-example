import { h, VNode } from "preact"
import { html } from "htm/preact"

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
import { AuthUserMemoField, AuthUserGrantedRolesField } from "../../input/field/x_preact/input"
import { EditButton } from "../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../common/x_preact/button/edit_success_button"
import { ResetButton } from "../../../../../common/x_preact/button/reset_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"
import { ChangeButton } from "../../../../../common/x_preact/button/change_button"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { ModifyAuthUserAccountAction } from "../action"

import { ModifyAuthUserAccountError } from "../data"

type Props = Readonly<{
    modify: ModifyAuthUserAccountAction
}>
export function ModifyAuthUserAccount(props: Props): VNode {
    const modifyState = useApplicationAction(props.modify)
    const editableState = useApplicationAction(props.modify.editable)
    const validateState = useApplicationAction(props.modify.validate)
    const observeState = useApplicationAction(props.modify.observe)

    const element = props.modify.data()
    if (!element.isLoad) {
        return html``
    }

    const edit = { data: element.data, editable: props.modify.editable }

    return form(
        box({
            title: "基本情報",
            body: [
                h(StaticLoginIdField, { data: edit.data }),
                h(AuthUserMemoField, { edit, field: props.modify.memo }),
                h(AuthUserGrantedRolesField, { edit, field: props.modify.grantedRoles }),
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
        if (modifyState.type === "success") {
            return h(EditSuccessButton, { onClick })
        } else {
            return h(EditButton, { onClick })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.modify.editable.open()
        }
    }

    function submitButton(): VNode {
        return h(ChangeButton, {
            isConnecting: modifyState.type === "try",
            validateState,
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.modify.submit()
        }
    }

    function resetButton(): VNode {
        return h(ResetButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.modify.reset()
        }
    }

    function closeButton(): VNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.modify.editable.close()
        }
    }

    function message(): readonly VNode[] {
        switch (modifyState.type) {
            case "initial":
            case "success":
                return []

            case "try":
                if (modifyState.hasTakenLongtime) {
                    return [takeLongtimeField("変更")]
                }
                return []

            case "failed":
                return [fieldHelp_error(modifyError(modifyState.err))]
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
