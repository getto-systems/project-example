import { h } from "preact"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../../common/x_preact/vnode"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField } from "../../../../../common/x_preact/design/form"

import { DeleteConfirmButton } from "../../../../../common/x_preact/button/delete_confirm_button"
import { DeleteButton } from "../../../../../common/x_preact/button/delete_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"

import { remoteCommonErrorReason } from "../../../../../common/util/remote/x_error/reason"

import { UnregisterAuthUserAccountAction } from "../action"

import { UnregisterAuthUserAccountError } from "../data"

type Props = Readonly<{
    unregister: UnregisterAuthUserAccountAction
}>
export function UnregisterAuthUserAccount(props: Props): PreactNode {
    const editableState = useAtom(props.unregister.editable.state)

    return box({
        form: true,
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
                          left: h(Submit, {}),
                          right: h(Close, {}),
                      }),
                      h(Message, {}),
                  ],
              }
            : {
                  body: h(DeleteConfirm, {}),
              }),
    })

    function DeleteConfirm(_props: unknown): PreactNode {
        return h(DeleteConfirmButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.unregister.editable.open()
        }
    }

    function Submit(_props: unknown): PreactNode {
        return h(DeleteButton, {
            connect: props.unregister.connect,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.unregister.submit()
        }
    }

    function Close(_props: unknown): PreactNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.unregister.editable.close()
        }
    }

    function Message(_props: unknown): PreactNode {
        const unregisterState = useAtom(props.unregister.state)

        switch (unregisterState.type) {
            case "initial":
            case "success":
                return html``

            case "try":
                if (unregisterState.hasTakenLongtime) {
                    return takeLongtimeField("変更")
                }
                return html``

            case "failed":
                return fieldHelp_error(modifyError(unregisterState.err))
        }
    }
}

function modifyError(err: UnregisterAuthUserAccountError): readonly PreactContent[] {
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
