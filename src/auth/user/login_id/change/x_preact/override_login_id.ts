import { h, VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"
import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { buttonLabel, icon_save, icon_spinner } from "../../../../../x_content/icon"

import { changeLoginIdError } from "./helper"
import { InputLoginIdEntry } from "../../input/x_preact/input"
import { SuccessButton } from "../../../../../core/x_preact/design/button"

import { OverrideLoginIdAction } from "../action"
import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"

import { LoginId } from "../../input/data"

type Props = Readonly<{
    user: Readonly<{ loginId: LoginId }>
    editable: EditableBoardAction
    override: OverrideLoginIdAction
}>
export function OverrideLoginId(props: Props): VNode {
    const state = useApplicationAction(props.override)
    const editableState = useApplicationAction(props.editable)
    const validateState = useApplicationAction(props.override.validate)

    return form(box({ title: "ログインID", ...content() }))

    type Content =
        | Readonly<{ body: VNodeContent }>
        | Readonly<{ body: VNodeContent; footer: VNodeContent }>
    function content(): Content {
        if (!editableState.isEditable) {
            return { body: openButton() }
        }
        return {
            body: [
                h(InputLoginIdEntry, {
                    field: props.override.newLoginId,
                    title: "新しいログインID",
                    help: ["管理者権限でログインIDを上書きします"],
                    autocomplete: "username",
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
        }
    }

    function openButton(): VNode {
        return h(SuccessButton, {
            label: LABEL_OVERRIDE.static,
            onClick,
            isSuccess: state.type === "succeed-to-override-login-id",
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.override.clear()
            props.editable.open()
        }
    }

    function submitButton(): VNode {
        switch (state.type) {
            case "initial-override-login-id":
            case "succeed-to-override-login-id":
            case "failed-to-override-login-id":
                switch (validateState) {
                    case "initial":
                    case "valid":
                        return button_send({
                            state: validateState === "initial" ? "normal" : "confirm",
                            label: LABEL_OVERRIDE.normal,
                            onClick,
                        })

                    case "invalid":
                        return button_disabled({ label: LABEL_OVERRIDE.normal })
                }
                break

            case "try-to-override-login-id":
            case "take-longtime-to-override-login-id":
                return button_send({ state: "connect", label: LABEL_OVERRIDE.connect })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.override.submit(props.user).then((state) => {
                switch (state.type) {
                    case "succeed-to-override-login-id":
                        props.editable.close()
                }
            })
        }
    }

    function clearButton(): VNode {
        switch (state.type) {
            case "initial-override-login-id":
            case "failed-to-override-login-id":
            case "succeed-to-override-login-id":
                switch (validateState) {
                    case "initial":
                        return button_disabled({ label: LABEL_CLEAR })

                    case "invalid":
                    case "valid":
                        return button_undo({ label: LABEL_CLEAR, onClick })
                }
                break

            case "try-to-override-login-id":
            case "take-longtime-to-override-login-id":
                return EMPTY_CONTENT
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.override.clear()
        }
    }
    function closeButton(): VNode {
        return button_undo({ label: "閉じる", onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.editable.close()
        }
    }

    function message(): readonly VNode[] {
        switch (state.type) {
            case "initial-override-login-id":
            case "succeed-to-override-login-id":
                switch (validateState) {
                    case "initial":
                    case "valid":
                        return []

                    case "invalid":
                        return [fieldError(["正しく入力されていません"])]
                }
                break

            case "failed-to-override-login-id":
                return [fieldError(changeLoginIdError(state.err))]

            case "try-to-override-login-id":
                return []

            case "take-longtime-to-override-login-id":
                return [
                    fieldError([
                        html`${icon_spinner} ログインID変更中です`,
                        html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                    ]),
                ]
        }
    }
}

const LABEL_OVERRIDE = buttonLabel("変更", icon_save)
const LABEL_CLEAR = "入力内容をクリア"

const EMPTY_CONTENT = html``
