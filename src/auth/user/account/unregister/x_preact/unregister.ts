import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    fieldHelp_error,
    form,
} from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField } from "../../../../../core/x_preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { DeleteConfirmButton } from "../../../../../z_lib/ui/button/delete_confirm_button"
import { DeleteButton } from "../../../../../z_lib/ui/button/delete_button"
import { CloseButton } from "../../../../../z_lib/ui/button/close_button"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { UnregisterAuthUserAccountAction } from "../action"

import { UnregisterAuthUserAccountError } from "../data"
import { LoginId } from "../../../login_id/kernel/data"
import { AuthRole } from "../../../kernel/data"
import { html } from "htm/preact"

type Props = Readonly<{
    user: Readonly<{ loginId: LoginId; grantedRoles: readonly AuthRole[] }>
    editable: EditableBoardAction
    unregister: UnregisterAuthUserAccountAction
    onSuccess: { (loginID: LoginId): void }
}>
export function UnregisterAuthUserAccount(props: Props): VNode {
    const state = useApplicationAction(props.unregister)
    const editableState = useApplicationAction(props.editable)

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
            props.editable.open()
        }
    }

    function submitButton(): VNode {
        return h(DeleteButton, {
            isConnecting: state.type === "try" || state.type === "take-longtime",
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.unregister.submit(props.user).then((state) => {
                if (state.type === "success") {
                    props.editable.close()
                    props.onSuccess(props.user.loginId)
                }
            })
        }
    }

    function closeButton(): VNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.editable.close()
        }
    }

    function message(): readonly VNode[] {
        switch (state.type) {
            case "initial":
            case "success":
            case "try":
                return []

            case "take-longtime":
                return [takeLongtimeField("変更")]

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
