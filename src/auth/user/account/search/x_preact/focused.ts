import { h, VNode } from "preact"
import { html } from "htm/preact"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { DetailAuthUserAccount, DetailAuthUserAccountActions } from "../../kernel/x_preact/detail"
import { BackToListButton } from "../../../../../common/x_preact/button/back_to_list_button"

import { FocusedAuthUserAccountAction } from "../action"

import { AuthUserAccount } from "../../kernel/data"

type Props = DetailAuthUserAccountActions &
    Readonly<{
        focused: FocusedAuthUserAccountAction
        user: Readonly<{ found: false }> | Readonly<{ found: true; user: AuthUserAccount }>
    }>
export function FocusedAuthUserAccount(props: Props): VNode {
    return html`${[container([box_grow({ body: backToListButton() })]), content()]}`

    function content(): VNode {
        if (!props.user.found) {
            return container([
                box_grow({ body: notice_gray(["指定されたユーザーが見つかりませんでした"]) }),
            ])
        }

        return h(DetailAuthUserAccount, {
            ...props,
            user: props.user.user,
            onModify: (loginId, user) => {
                props.focused.update(loginId, user)
            },
            onUnregister: (loginId) => {
                props.focused.remove(loginId)
            },
        })
    }

    function backToListButton(): VNode {
        return h(BackToListButton, { onClick })

        function onClick() {
            props.focused.close()
        }
    }
}
