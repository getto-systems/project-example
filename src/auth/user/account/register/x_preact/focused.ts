import { h, VNode } from "preact"
import { html } from "htm/preact"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { DetailAuthUserAccount, DetailAuthUserAccountActions } from "../../kernel/x_preact/detail"
import { BackToRegisterButton } from "../../../../../core/x_preact/button/back_to_register_button"

import { FocusedRegisteredAuthUserAccountAction } from "../action"

import { AuthUserAccount } from "../../kernel/data"

type Props = DetailAuthUserAccountActions &
    Readonly<{
        focused: FocusedRegisteredAuthUserAccountAction
        user: AuthUserAccount
    }>
export function FocusedRegisteredAuthUserAccount(props: Props): VNode {
    return html`${[
        container([box_grow({ body: backToRegisterButton() })]),
        h(DetailAuthUserAccount, {
            ...props,
            onModify: (loginId, user) => {
                props.focused.update(loginId, user)
            },
            onUnregister: (loginId) => {
                props.focused.remove(loginId)
            }
        }),
    ]}`

    function backToRegisterButton(): VNode {
        return h(BackToRegisterButton, { onClick })

        function onClick() {
            props.focused.close()
        }
    }
}
