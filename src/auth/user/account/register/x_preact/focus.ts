import { h, VNode } from "preact"
import { html } from "htm/preact"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { DetailAuthUserAccount, DetailAuthUserAccountActions } from "../../kernel/x_preact/detail"
import { BackToRegisterButton } from "../../../../../common/x_preact/button/back_to_register_button"

import { RegisterAuthUserAccountAction } from "../action"

type Props = DetailAuthUserAccountActions &
    Readonly<{
        register: RegisterAuthUserAccountAction
    }>
export function FocusRegisteredAuthUserAccount(props: Props): VNode {
    return html`${[
        container([box_grow({ body: backToRegisterButton() })]),
        h(DetailAuthUserAccount, props),
    ]}`

    function backToRegisterButton(): VNode {
        return h(BackToRegisterButton, { onClick })

        function onClick() {
            props.register.list.focus.close()
        }
    }
}
