import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    fieldHelp_error,
    form,
} from "../../../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../../../z_vendor/getto-css/preact/design/box"

import { VNodeContent } from "../../../../../../../z_lib/ui/x_preact/common"

import {
    takeLongtimeField,
    validationMessage,
} from "../../../../../../../common/x_preact/design/form"

import { ResetTokenDestinationField } from "../../input/x_preact/input"
import { EditButton } from "../../../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../../../common/x_preact/button/edit_success_button"
import { ResetButton } from "../../../../../../../common/x_preact/button/reset_button"
import { ChangeButton } from "../../../../../../../common/x_preact/button/change_button"
import { CloseButton } from "../../../../../../../common/x_preact/button/close_button"

import { remoteCommonErrorReason } from "../../../../../../../z_lib/ui/remote/x_error/reason"

import { ChangeResetTokenDestinationAction } from "../action"

import { ChangeResetTokenDestinationError } from "../data"

type Props = Readonly<{
    change: ChangeResetTokenDestinationAction
}>
export function ChangeResetTokenDestination(props: Props): VNode {
    const state = useApplicationAction(props.change)
    const editableState = useApplicationAction(props.change.editable)
    const validateState = useApplicationAction(props.change.validate)
    const observeState = useApplicationAction(props.change.observe)

    const element = props.change.data()
    if (!element.isLoad) {
        return html``
    }

    const edit = { data: element.data, editable: props.change.editable }

    return form(
        box({
            title: "パスワードリセット",
            body: [h(ResetTokenDestinationField, { edit, field: props.change.destination })],
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
            props.change.editable.open()
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
            props.change.submit()
        }
    }

    function resetButton(): VNode {
        return h(ResetButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.change.reset()
        }
    }

    function closeButton(): VNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.change.editable.close()
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
                return [fieldHelp_error(changeError(state.err))]
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
