import { h, VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { useApplicationAction } from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../../z_vendor/getto-css/preact/design/form"
import { icon_change } from "../../../../../../x_content/icon"
import { box } from "../../../../../../z_vendor/getto-css/preact/design/box"
import { notice_success } from "../../../../../../z_vendor/getto-css/preact/design/highlight"
import { takeLongtimeField, validationMessage } from "../../../../../../common/x_preact/design/form"

import { remoteCommonErrorReason } from "../../../../../../z_lib/ui/remote/x_error/reason"

import { LoginIdField } from "../../../../login_id/input/x_preact/input"
import { EditButton } from "../../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../../common/x_preact/button/edit_success_button"
import { SendButton } from "../../../../../../common/x_preact/button/send_button"
import { ClearChangesButton } from "../../../../../../common/x_preact/button/clear_changes_button"
import { CloseButton } from "../../../../../../common/x_preact/button/close_button"

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
    const observeState = useApplicationAction(props.requestToken.observe)

    return box({
        title: "パスワードリセット",
        ...(editableState.isEditable
            ? {
                  form: true,
                  body: h(LoginIdField, {
                      field: props.requestToken.loginId,
                      help: ["確認のため、ログインIDを入力します"],
                  }),
                  footer: [
                      buttons({
                          left: submitButton(),
                          right: clearButton(),
                      }),
                      ...validationMessage(validateState),
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
    })

    function editButton(): VNode {
        const label = "トークン送信"
        if (state.type === "success") {
            return h(EditSuccessButton, { label, onClick })
        } else {
            return h(EditButton, { label, onClick })
        }

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
            isConnecting: state.type === "try",
            validateState,
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.submit(onSuccess)

            function onSuccess() {
                props.editable.close()
            }
        }
    }

    function clearButton(): VNode {
        return h(ClearChangesButton, { observeState, onClick })

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
                return []

            case "try":
                if (state.hasTakenLongtime) {
                    return [takeLongtimeField("リセットトークン送信")]
                }
                return []

            case "failed":
                return [fieldHelp_error(requestTokenError(state.err))]
        }
    }
}

function requestTokenError(err: RequestResetTokenError): readonly VNodeContent[] {
    switch (err.type) {
        case "invalid-reset":
            return ["ログインIDが登録されていないか、トークンの送信先が登録されていません"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}によりトークンの送信に失敗しました`,
                ...reason.detail,
            ])
    }
}
