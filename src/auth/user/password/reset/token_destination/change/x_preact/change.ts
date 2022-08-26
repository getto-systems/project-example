import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    fieldHelp_error,
} from "../../../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../../../z_vendor/getto-css/preact/design/box"

import { VNodeContent } from "../../../../../../../z_lib/ui/x_preact/common"

import {
    takeLongtimeField,
    ValidationMessage,
} from "../../../../../../../common/x_preact/design/form"

import { ResetTokenDestinationField } from "../../input/x_preact/input"
import { EditButton } from "../../../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../../../common/x_preact/button/edit_success_button"
import { ResetButton } from "../../../../../../../common/x_preact/button/reset_button"
import { ChangeButton } from "../../../../../../../common/x_preact/button/change_button"
import { CloseButton } from "../../../../../../../common/x_preact/button/close_button"

import { remoteCommonErrorReason } from "../../../../../../../z_lib/ui/remote/x_error/reason"

import { ChangeResetTokenDestinationAction } from "../action"
import { ApplicationState } from "../../../../../../../z_vendor/getto-application/action/action"
import { FocusState } from "../../../../../../../z_lib/ui/list/action"

import { ChangeResetTokenDestinationError } from "../data"
import { AuthUserAccount } from "../../../../../account/kernel/data"

type Props = Readonly<{
    focus: ApplicationState<FocusState<AuthUserAccount>>
    change: ChangeResetTokenDestinationAction
}>
export function ChangeResetTokenDestination(props: Props): VNode {
    const focusState = useApplicationState(props.focus)
    switch (focusState.type) {
        case "close":
        case "not-found":
        case "data-remove":
            return html``
    }

    const edit = { data: focusState.data, editable: props.change.editable }

    return box({
        form: true,
        title: "パスワードリセット",
        body: [h(ResetTokenDestinationField, { edit, field: props.change.destination })],
        footer: h(Footer, {}),
    })

    function Footer(_props: unknown): VNode {
        const editableState = useApplicationState(props.change.editable.state)

        if (!editableState.isEditable) {
            return h(Edit, {})
        }
        return html`${[
            buttons({ left: h(Submit, {}), right: h(Reset, {}) }),
            h(ValidationMessage, props.change.validate),
            h(Message, {}),
            buttons({ right: h(Close, {}) }),
        ]}`

        function Edit(_props: unknown): VNode {
            const changeState = useApplicationState(props.change.state)

            if (changeState.type === "success") {
                return h(EditSuccessButton, { onClick })
            } else {
                return h(EditButton, { onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.change.editable.open()
            }
        }

        function Submit(_props: unknown): VNode {
            const changeState = useApplicationState(props.change.state)
            const validateState = useApplicationState(props.change.validate.state)
            const observeState = useApplicationState(props.change.observe.state)

            return h(ChangeButton, {
                isConnecting: changeState.type === "try",
                validateState,
                observeState,
                onClick,
            })

            function onClick(e: Event) {
                e.preventDefault()
                props.change.submit()
            }
        }

        function Reset(_props: unknown): VNode {
            const observeState = useApplicationState(props.change.observe.state)

            return h(ResetButton, { observeState, onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.change.reset()
            }
        }

        function Close(_props: unknown): VNode {
            return h(CloseButton, { onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.change.editable.close()
            }
        }

        function Message(_props: unknown): VNode {
            const changeState = useApplicationState(props.change.state)

            switch (changeState.type) {
                case "initial":
                case "success":
                    return html``

                case "try":
                    if (changeState.hasTakenLongtime) {
                        return takeLongtimeField("変更")
                    }
                    return html``

                case "failed":
                    return fieldHelp_error(changeError(changeState.err))
            }
        }
    }
}

function changeError(err: ChangeResetTokenDestinationError): readonly VNodeContent[] {
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
