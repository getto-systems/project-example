import { h, VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { useApplicationAction } from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../../z_vendor/getto-css/preact/design/form"
import { icon_spinner } from "../../../../../../x_content/icon"
import { box } from "../../../../../../z_vendor/getto-css/preact/design/box"
import { notice_success } from "../../../../../../z_vendor/getto-css/preact/design/highlight"

import { remoteCommonErrorReason } from "../../../../../../z_lib/ui/remote/x_error/reason"

import { InputLoginId } from "../../../../login_id/input/x_preact/input"

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

    const content = {
        title: "パスワードリセット",
    }

    if (!editableState.isEditable) {
        return form(
            box({
                ...content,
                body: openButton(),
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
        )
    }

    return form(
        box({
            ...content,
            body: [
                h(InputLoginId, {
                    field: props.requestToken.loginId,
                    title: "ログインID",
                    help: ["確認のため、ログインIDを入力します"],
                }),
            ],
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
        }),
    )

    function openButton(): VNode {
        return button_send({ state: "normal", label: "トークン送信", onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.clear()
            props.editable.open()
        }
    }

    function submitButton(): VNode {
        const label = "トークン送信"

        switch (state.type) {
            case "initial":
            case "success":
            case "failed":
                switch (validateState) {
                    case "initial":
                        return button_send({ state: "normal", label, onClick })

                    case "valid":
                        return button_send({ state: "confirm", label, onClick })

                    case "invalid":
                        return button_disabled({ label })
                }
                break

            case "try":
            case "take-longtime":
                return button_send({
                    state: "connect",
                    label: html`リセットトークン送信中 ${icon_spinner}`,
                })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.submit().then(() => {
                props.editable.close()
            })
        }
    }

    function clearButton(): VNode {
        const label = "入力内容をクリア"
        switch (state.type) {
            case "initial":
            case "success":
            case "failed":
                switch (validateState) {
                    case "initial":
                        return button_disabled({ label })

                    case "invalid":
                    case "valid":
                        return button_undo({ label, onClick })
                }
                break

            case "try":
            case "take-longtime":
                return EMPTY_CONTENT
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.requestToken.clear()
        }
    }
    function closeButton(): VNode {
        return button_undo({ label: "閉じる", onClick })

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
                        html`${icon_spinner} リセットトークン送信中です`,
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

const EMPTY_CONTENT = html``
