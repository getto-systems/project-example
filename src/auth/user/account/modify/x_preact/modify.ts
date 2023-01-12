import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, ValidationMessage } from "../../../../../common/x_preact/design/form"

import { VNodeContent } from "../../../../../common/x_preact/vnode"

import { StaticLoginIdField } from "../../../login_id/input/x_preact/static"
import { AuthUserMemoField, AuthPermissionGrantedField } from "../../input/field/x_preact/input"
import { EditButton } from "../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../common/x_preact/button/edit_success_button"
import { ResetButton } from "../../../../../common/x_preact/button/reset_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"
import { ChangeButton } from "../../../../../common/x_preact/button/change_button"

import { remoteCommonErrorReason } from "../../../../../common/util/remote/x_error/reason"

import { ModifyAuthUserAccountAction } from "../action"
import { ApplicationState } from "../../../../../z_vendor/getto-application/action/action"
import { FocusState } from "../../../../../common/util/list/action"

import { ModifyAuthUserAccountError } from "../data"
import { AuthUserAccount } from "../../kernel/data"

type Props = Readonly<{
    focus: ApplicationState<FocusState<AuthUserAccount>>
    modify: ModifyAuthUserAccountAction
}>
export function ModifyAuthUserAccount(props: Props): VNode {
    const focusState = useApplicationState(props.focus)
    switch (focusState.type) {
        case "close":
        case "not-found":
        case "data-remove":
            return html``
    }

    const edit = { data: focusState.data, editable: props.modify.editable }

    return box({
        form: true,
        title: "基本情報",
        body: [
            h(StaticLoginIdField, { data: edit.data }),
            h(AuthUserMemoField, { edit, field: props.modify.memo }),
            h(AuthPermissionGrantedField, { edit, field: props.modify.granted }),
        ],
        footer: h(Footer, {}),
    })

    function Footer(_props: unknown): VNode {
        const editableState = useApplicationState(props.modify.editable.state)

        if (!editableState.isEditable) {
            return h(Edit, {})
        }
        return html`${[
            buttons({ left: h(Submit, {}), right: h(Reset, {}) }),
            h(ValidationMessage, props.modify.validate),
            h(Message, {}),
            buttons({ right: h(Close, {}) }),
        ]}`

        function Edit(_props: unknown): VNode {
            const modifyState = useApplicationState(props.modify.state)

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

        function Submit(_props: unknown): VNode {
            const modifyState = useApplicationState(props.modify.state)
            const validateState = useApplicationState(props.modify.validate.state)
            const observeState = useApplicationState(props.modify.observe.state)

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

        function Reset(): VNode {
            const observeState = useApplicationState(props.modify.observe.state)

            return h(ResetButton, { observeState, onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.modify.reset()
            }
        }

        function Close(_props: unknown): VNode {
            return h(CloseButton, { onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.modify.editable.close()
            }
        }

        function Message(_props: unknown): VNode {
            const modifyState = useApplicationState(props.modify.state)

            switch (modifyState.type) {
                case "initial":
                case "success":
                    return html``

                case "try":
                    if (modifyState.hasTakenLongtime) {
                        return takeLongtimeField("変更")
                    }
                    return html``

                case "failed":
                    return fieldHelp_error(modifyError(modifyState.err))
            }
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
