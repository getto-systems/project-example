import { h } from "preact"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../../../../common/x_preact/vnode"

import { useAtom } from "../../../../../../../z_vendor/getto-atom/x_preact/hooks"

import {
    buttons,
    fieldHelp_error,
} from "../../../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../../../z_vendor/getto-css/preact/design/box"

import {
    takeLongtimeField,
    ValidateBoardMessage,
} from "../../../../../../../common/x_preact/design/form"
import { remoteCommonErrorReason } from "../../../../../../../common/util/remote/x_error/reason"

import { EditButton } from "../../../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../../../common/x_preact/button/edit_success_button"
import { ResetButton } from "../../../../../../../common/x_preact/button/reset_button"
import { ChangeButton } from "../../../../../../../common/x_preact/button/change_button"
import { CloseButton } from "../../../../../../../common/x_preact/button/close_button"
import { ResetTokenDestinationField } from "../../input/field/x_preact/input"

import { Atom } from "../../../../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../../../../common/util/load/data"
import { ChangeResetTokenDestinationAction } from "../action"

import { ChangeResetTokenDestinationError } from "../data"
import { AuthUserAccount } from "../../../../../account/kernel/data"

type Props = Readonly<{
    focus: Atom<LoadState<AuthUserAccount>>
    change: ChangeResetTokenDestinationAction
}>
export function ChangeResetTokenDestination(props: Props): PreactNode {
    const focusState = useAtom(props.focus)
    if (!focusState.isLoad) {
        return html``
    }

    const edit = { data: focusState.data, editable: props.change.editable }

    return box({
        form: true,
        title: "パスワードリセット",
        body: [h(ResetTokenDestinationField, { edit, field: props.change.destination })],
        footer: h(Footer, {}),
    })

    function Footer(_props: unknown): PreactNode {
        const editableState = useAtom(props.change.editable.state)

        if (!editableState.isEditable) {
            return h(Edit, {})
        }
        return html`${[
            buttons({ left: h(Submit, {}), right: h(Reset, {}) }),
            h(ValidateBoardMessage, { state: props.change.validate }),
            h(Message, {}),
            buttons({ right: h(Close, {}) }),
        ]}`

        function Edit(_props: unknown): PreactNode {
            const changeState = useAtom(props.change.state)

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

        function Submit(_props: unknown): PreactNode {
            return h(ChangeButton, {
                connect: props.change.connect,
                validate: props.change.validate,
                observe: props.change.observe,
                onClick,
            })

            function onClick(e: Event) {
                e.preventDefault()
                props.change.submit()
            }
        }

        function Reset(_props: unknown): PreactNode {
            return h(ResetButton, {
                observe: props.change.observe,
                onClick,
            })

            function onClick(e: Event) {
                e.preventDefault()
                props.change.reset()
            }
        }

        function Close(_props: unknown): PreactNode {
            return h(CloseButton, { onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.change.editable.close()
            }
        }

        function Message(_props: unknown): PreactNode {
            const changeState = useAtom(props.change.state)

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

function changeError(err: ChangeResetTokenDestinationError): readonly PreactContent[] {
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
