import { h } from "preact"
import { html } from "htm/preact"
import { PreactNode } from "../../../../../common/x_preact/node"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, ValidateBoardMessage } from "../../../../../common/x_preact/design/form"

import { AuthUserPasswordField } from "../../input/field/x_preact/input"
import { EditButton } from "../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../common/x_preact/button/edit_success_button"
import { ClearChangesButton } from "../../../../../common/x_preact/button/clear_changes_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"
import { ChangeButton } from "../../../../../common/x_preact/button/change_button"

import { changePasswordError } from "./helper"

import { OverwritePasswordAction } from "../action"

type Props = Readonly<{
    overwrite: OverwritePasswordAction
}>
export function OverwritePassword(props: Props): PreactNode {
    const editableState = useAtom(props.overwrite.editable.state)

    return box({
        form: true,
        title: "パスワード変更",
        ...(editableState.isEditable
            ? {
                  body: [
                      h(AuthUserPasswordField, {
                          field: props.overwrite.newPassword,
                          title: "新しいパスワード",
                          help: ["管理者権限でパスワードを上書きします"],
                          autocomplete: "new-password",
                      }),
                  ],
                  footer: [
                      buttons({ left: h(Submit, {}), right: h(Reset, {}) }),
                      h(ValidateBoardMessage, { state: props.overwrite.validate }),
                      h(Message, {}),
                      buttons({ right: h(Close, {}) }),
                  ],
              }
            : {
                  body: h(Edit, {}),
              }),
    })

    function Edit(_props: unknown): PreactNode {
        const overwriteState = useAtom(props.overwrite.state)

        if (overwriteState.type === "success") {
            return h(EditSuccessButton, { onClick })
        } else {
            return h(EditButton, { onClick })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.editable.open()
        }
    }

    function Submit(_props: unknown): PreactNode {
        return h(ChangeButton, {
            connect: props.overwrite.connect,
            validate: props.overwrite.validate,
            observe: props.overwrite.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.submit()
        }
    }

    function Reset(_props: unknown): PreactNode {
        return h(ClearChangesButton, {
            observe: props.overwrite.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.reset()
        }
    }
    function Close(_props: unknown): PreactNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.editable.close()
        }
    }

    function Message(_props: unknown): PreactNode {
        const overwriteState = useAtom(props.overwrite.state)

        switch (overwriteState.type) {
            case "initial":
            case "success":
                return html``

            case "try":
                if (overwriteState.hasTakenLongtime) {
                    return takeLongtimeField("変更")
                }
                return html``

            case "failed":
                return fieldHelp_error(changePasswordError(overwriteState.err))
        }
    }
}
