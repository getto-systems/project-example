import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { DetailAuthUserAccount, DetailAuthUserAccountActions } from "../../kernel/x_preact/detail"
import { BackToRegisterButton } from "../../../../../common/x_preact/button/back_to_register_button"

import { RegisterAuthUserAccountAction } from "../action"

type Props = DetailAuthUserAccountActions &
    Readonly<{
        register: RegisterAuthUserAccountAction
    }>
export function FocusRegisteredAuthUserAccount(props: Props): VNode {
    return html`${[container([box_grow({ body: backToRegisterButton() })]), h(Content, {})]}`

    function Content(_props: unknown): VNode {
        const focusState = useApplicationState(props.register.list.focus.state)
        switch (focusState.type) {
            case "not-found":
            case "data-remove":
                return container([box_grow({ body: notice_gray(["このデータは削除されました"]) })])

            default:
                return h(DetailAuthUserAccount, props)
        }
    }

    function backToRegisterButton(): VNode {
        return h(BackToRegisterButton, { onClick })

        function onClick() {
            props.register.list.focus.close()
        }
    }
}
