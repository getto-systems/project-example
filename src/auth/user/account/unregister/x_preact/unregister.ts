import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    fieldHelp_error,
    form,
} from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField } from "../../../../../common/x_preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { DeleteConfirmButton } from "../../../../../common/x_preact/button/delete_confirm_button"
import { DeleteButton } from "../../../../../common/x_preact/button/delete_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { UnregisterAuthUserAccountAction } from "../action"

import { UnregisterAuthUserAccountError } from "../data"

type Props = Readonly<{
    unregister: UnregisterAuthUserAccountAction
}>
export function UnregisterAuthUserAccount(props: Props): VNode {
    const state = useApplicationState(props.unregister.state)
    const editableState = useApplicationState(props.unregister.editable.state)

    return form(
        box({
            title: "ユーザー削除",
            ...(editableState.isEditable
                ? {
                      body: [
                          html`<p>このユーザーを削除します</p>`,
                          html`<p>
                              削除後は、すぐに利用できなくなります<br />
                              現在ログインしている場合、ログアウトされます
                          </p>`,
                          html`<p>削除した後に元に戻すことはできません</p>`,
                      ],
                      footer: [
                          buttons({
                              left: submitButton(),
                              right: closeButton(),
                          }),
                          ...message(),
                      ],
                  }
                : {
                      body: deleteConfirmButton(),
                  }),
        }),
    )

    function deleteConfirmButton(): VNode {
        return h(DeleteConfirmButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.unregister.editable.open()
        }
    }

    function submitButton(): VNode {
        return h(DeleteButton, {
            isConnecting: state.type === "try",
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.unregister.submit()
        }
    }

    function closeButton(): VNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.unregister.editable.close()
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

function modifyError(err: UnregisterAuthUserAccountError): readonly VNodeContent[] {
    switch (err.type) {
        case "invalid":
            return ["データが正しくありません", "一旦リロードしてやり直してください"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}により変更に失敗しました`,
                ...reason.detail,
            ])
    }
}
