import { h } from "preact"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../../common/x_preact/node"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, ValidateBoardMessage } from "../../../../../common/x_preact/design/form"

import { AuthUserLoginIdStaticField } from "../../../login_id/input/field/x_preact/input"
import { AuthPermissionGrantedField } from "../../../kernel/input/field/x_preact/input"
import { AuthUserMemoField } from "../../input/field/x_preact/input"
import { EditButton } from "../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../common/x_preact/button/edit_success_button"
import { ResetButton } from "../../../../../common/x_preact/button/reset_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"
import { ChangeButton } from "../../../../../common/x_preact/button/change_button"

import { remoteCommonErrorReason } from "../../../../../common/util/remote/x_error/reason"

import { Atom } from "../../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../../common/util/load/data"
import { ModifyAuthUserAccountAction } from "../action"

import { ModifyAuthUserAccountError } from "../data"
import { AuthUserAccount } from "../../kernel/data"

type Props = Readonly<{
    focus: Atom<LoadState<AuthUserAccount>>
    modify: ModifyAuthUserAccountAction
}>
export function ModifyAuthUserAccount(props: Props): PreactNode {
    const focusState = useAtom(props.focus)
    if (!focusState.isLoad) {
        return html``
    }

    const edit = { data: focusState.data, editable: props.modify.editable }

    return box({
        form: true,
        title: "基本情報",
        body: [
            h(AuthUserLoginIdStaticField, { data: edit.data }),
            h(AuthUserMemoField, { edit, field: props.modify.memo }),
            h(AuthPermissionGrantedField, { edit, field: props.modify.granted }),
        ],
        footer: h(Footer, {}),
    })

    function Footer(_props: unknown): PreactNode {
        const editableState = useAtom(props.modify.editable.state)

        if (!editableState.isEditable) {
            return h(Edit, {})
        }
        return html`${[
            buttons({ left: h(Submit, {}), right: h(Reset, {}) }),
            h(ValidateBoardMessage, { state: props.modify.validate }),
            h(Message, {}),
            buttons({ right: h(Close, {}) }),
        ]}`

        function Edit(_props: unknown): PreactNode {
            const modifyState = useAtom(props.modify.state)

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

        function Submit(_props: unknown): PreactNode {
            return h(ChangeButton, {
                connect: props.modify.connect,
                validate: props.modify.validate,
                observe: props.modify.observe,
                onClick,
            })

            function onClick(e: Event) {
                e.preventDefault()
                props.modify.submit()
            }
        }

        function Reset(): PreactNode {
            return h(ResetButton, {
                observe: props.modify.observe,
                onClick,
            })

            function onClick(e: Event) {
                e.preventDefault()
                props.modify.reset()
            }
        }

        function Close(_props: unknown): PreactNode {
            return h(CloseButton, { onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.modify.editable.close()
            }
        }

        function Message(_props: unknown): PreactNode {
            const modifyState = useAtom(props.modify.state)

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

function modifyError(err: ModifyAuthUserAccountError): readonly PreactContent[] {
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
