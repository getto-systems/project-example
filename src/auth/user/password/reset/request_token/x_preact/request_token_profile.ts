import { h, VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { useApplicationAction } from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldError, form } from "../../../../../../z_vendor/getto-css/preact/design/form"
import { icon_change, icon_spinner } from "../../../../../../x_content/icon"
import { box } from "../../../../../../z_vendor/getto-css/preact/design/box"
import { notice_success } from "../../../../../../z_vendor/getto-css/preact/design/highlight"
import { iconHtml } from "../../../../../../core/x_preact/design/icon"

import { remoteCommonErrorReason } from "../../../../../../z_lib/ui/remote/x_error/reason"

import { InputLoginId } from "../../../../login_id/input/x_preact/input"
import { EditButton } from "../../../../../../core/x_preact/button/edit_button"
import { SendButton } from "../../../../../../core/x_preact/button/send_button"
import { ClearChangesButton } from "../../../../../../core/x_preact/button/clear_changes_button"
import { CloseButton } from "../../../../../../core/x_preact/button/close_button"

import { RequestResetTokenAction } from "../action"
import { EditableBoardAction } from "../../../../../../z_vendor/getto-application/board/editable/action"

import { RequestResetTokenError } from "../data"

type Props = Readonly<{
    editable: EditableBoardAction
    requestToken: RequestResetTokenAction
}>
export function RequestResetTokenProfile(props: Props): VNode {
    const state = useApplicationAction(props.requestToken)
    const editableState = useApplicationAction(props.editable)
    const validateState = useApplicationAction(props.requestToken.validate)

    return form(
        box({
            title: "パスワードリセット",
            ...(editableState.isEditable
                ? {
                      body: h(InputLoginId, {
                          field: props.requestToken.loginId,
                          title: "ログインID",
                          help: ["確認のため、ログインIDを入力します"],
                      }),
                      footer: [
                          buttons({
                              left: submitButton(),
                              right: clearButton(),
                          }),
                          ...message(),
                          buttons({
                              right: closeButton(),
                          }),
                      ],
                  }
                : {
                      body: editButton(),
                      footer:
                          state.type === "success"
                              ? [
                                    notice_success([
                                        html`パスワードリセットのための<br />
                                            トークンをメールで送信しました`,
                                    ]),
                                    html`<p>
                                        メールからパスワードリセットできます<br />
                                        メールを確認してください
                                    </p>`,
                                ]
                              : undefined,
                  }),
        }),
    )

    function editButton(): VNode {
        return h(EditButton, {
            label: "トークン送信",
            isSuccess: state.type === "success",
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.clear()
            props.editable.open()
        }
    }

    function submitButton(): VNode {
        return h(SendButton, {
            label: "トークン送信",
            icon: icon_change,
            isConnecting: state.type === "try" || state.type === "take-longtime",
            validateState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.submit().then(() => {
                props.editable.close()
            })
        }
    }

    function clearButton(): VNode {
        return h(ClearChangesButton, { validateState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.clear()
        }
    }
    function closeButton(): VNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.clear()
            props.editable.close()
        }
    }

    function message(): readonly VNode[] {
        switch (state.type) {
            case "initial":
            case "success":
                switch (validateState) {
                    case "initial":
                    case "valid":
                        return []

                    case "invalid":
                        return [fieldError(["正しく入力されていません"])]
                }
                break

            case "try":
                return []

            case "take-longtime":
                return [
                    fieldError([
                        html`${iconHtml(icon_spinner)} リセットトークン送信に時間がかかっています`,
                        html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                    ]),
                ]

            case "failed":
                return [fieldError(requestTokenError(state.err))]
        }
    }
}

function requestTokenError(err: RequestResetTokenError): readonly VNodeContent[] {
    switch (err.type) {
        case "validation-error":
            return ["正しく入力してください"]

        case "invalid-reset":
            return ["ログインIDが登録されていないか、トークンの送信先が登録されていません"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}によりトークンの送信に失敗しました`,
                ...reason.detail,
            ])
    }
}
