import { h } from "preact"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../../../common/x_preact/node"

import { useAtom } from "../../../../../../z_vendor/getto-atom/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../../z_vendor/getto-css/preact/design/form"
import { icon_change } from "../../../../../../x_content/icon"
import { box } from "../../../../../../z_vendor/getto-css/preact/design/box"
import { notice_success } from "../../../../../../z_vendor/getto-css/preact/design/highlight"
import {
    takeLongtimeField,
    ValidateBoardMessage,
} from "../../../../../../common/x_preact/design/form"

import { remoteCommonErrorReason } from "../../../../../../common/util/remote/x_error/reason"

import { AuthUserLoginIdField } from "../../../../login_id/input/field/x_preact/input"
import { EditButton } from "../../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../../common/x_preact/button/edit_success_button"
import { SendButton } from "../../../../../../common/x_preact/button/send_button"
import { ClearChangesButton } from "../../../../../../common/x_preact/button/clear_changes_button"
import { CloseButton } from "../../../../../../common/x_preact/button/close_button"

import { RequestResetTokenAction } from "../action"

import { RequestResetTokenError } from "../data"

type Props = Readonly<{
    requestToken: RequestResetTokenAction
}>
export function RequestResetTokenProfile(props: Props): PreactNode {
    const editableState = useAtom(props.requestToken.editable.state)

    return box({
        form: true,
        title: "パスワードリセット",
        ...(editableState.isEditable
            ? {
                  body: h(AuthUserLoginIdField, {
                      field: props.requestToken.loginId,
                      help: ["確認のため、ログインIDを入力します"],
                  }),
                  footer: [
                      buttons({
                          left: h(Submit, {}),
                          right: h(Reset, {}),
                      }),
                      h(ValidateBoardMessage, { state: props.requestToken.validate }),
                      h(Message, {}),
                      buttons({
                          right: h(Close, {}),
                      }),
                  ],
              }
            : {
                  body: h(Edit, {}),
                  footer: h(SuccessMessage, {}),
              }),
    })

    function Edit(_props: unknown): PreactNode {
        const requestTokenState = useAtom(props.requestToken.state)

        const label = "トークン送信"
        if (requestTokenState.type === "success") {
            return h(EditSuccessButton, { label, onClick })
        } else {
            return h(EditButton, { label, onClick })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.editable.open()
        }
    }

    function Submit(_props: unknown): PreactNode {
        return h(SendButton, {
            label: "トークン送信",
            icon: icon_change,
            connect: props.requestToken.connect,
            validate: props.requestToken.validate,
            observe: props.requestToken.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.submit()
        }
    }

    function Reset(_props: unknown): PreactNode {
        return h(ClearChangesButton, {
            observe: props.requestToken.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.reset()
        }
    }
    function Close(_props: unknown): PreactNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.editable.close()
        }
    }

    function Message(_props: unknown): PreactNode {
        const requestTokenState = useAtom(props.requestToken.state)

        switch (requestTokenState.type) {
            case "initial":
            case "success":
                return html``

            case "try":
                if (requestTokenState.hasTakenLongtime) {
                    return takeLongtimeField("リセットトークン送信")
                }
                return html``

            case "failed":
                return fieldHelp_error(requestTokenError(requestTokenState.err))
        }
    }
    function SuccessMessage(_props: unknown): PreactNode {
        const requestTokenState = useAtom(props.requestToken.state)

        switch (requestTokenState.type) {
            case "success":
                return html`${notice_success([
                        html`パスワードリセットのための<br />
                            トークンをメールで送信しました`,
                    ])}
                    <p>
                        メールからパスワードリセットできます<br />
                        メールを確認してください
                    </p>`

            default:
                return html`<p>
                    パスワードリセットのためのトークンを<br />
                    登録されたメールアドレスに送信します
                </p>`
        }
    }
}

function requestTokenError(err: RequestResetTokenError): readonly PreactContent[] {
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
