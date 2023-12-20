import { h } from "preact"
import { html } from "htm/preact"
import { PreactNode } from "../../../../../common/x_preact/vnode"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, ValidateBoardMessage } from "../../../../../common/x_preact/design/form"

import { changePasswordError } from "./helper"
import { AuthUserPasswordField } from "../../input/field/x_preact/input"
import { ClearChangesButton } from "../../../../../common/x_preact/button/clear_changes_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"
import { ChangeButton } from "../../../../../common/x_preact/button/change_button"
import { EditButton } from "../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../common/x_preact/button/edit_success_button"

import { ChangePasswordAction } from "../action"

type Props = Readonly<{
    change: ChangePasswordAction
}>
export function ChangePassword(props: Props): PreactNode {
    const editableState = useAtom(props.change.editable.state)

    return box({
        form: true,
        title: "パスワード変更",
        ...(editableState.isEditable
            ? {
                  body: [
                      h(AuthUserPasswordField, {
                          field: props.change.currentPassword,
                          title: "現在のパスワード",
                          help: ["変更前のパスワードを入力します"],
                          autocomplete: "current-password",
                      }),
                      h(AuthUserPasswordField, {
                          field: props.change.newPassword,
                          title: "新しいパスワード",
                          help: ["今後はこのパスワードになります"],
                          autocomplete: "new-password",
                      }),
                  ],
                  footer: [
                      buttons({ left: h(Submit, {}), right: h(Reset, {}) }),
                      h(ValidateBoardMessage, { state: props.change.validate }),
                      h(Message, {}),
                      buttons({ right: h(Close, {}) }),
                  ],
              }
            : {
                  body: h(Edit, {}),
              }),
    })

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
        return h(ClearChangesButton, {
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
                return fieldHelp_error(changePasswordError(changeState.err))
        }
    }
}
