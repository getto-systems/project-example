import { h } from "preact"
import { html } from "htm/preact"
import { PreactNode } from "../../../../../common/x_preact/vnode"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import { box, box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { DetailAuthUserAccount, DetailAuthUserAccountActions } from "../../kernel/x_preact/detail"
import { BackLink } from "../../../../../common/x_preact/button/back_link"

import { RegisterAuthUserAccountAction } from "../action"

type Props = DetailAuthUserAccountActions &
    Readonly<{
        register: RegisterAuthUserAccountAction
    }>
export function FocusRegisteredAuthUserAccount(props: Props): PreactNode {
    return html`${[container([box({ body: backLink() })]), h(Content, {})]}`

    function Content(_props: unknown): PreactNode {
        const isFocused = useAtom(props.register.focus.isSomeEntryFocused)
        if (isFocused) {
            return h(DetailAuthUserAccount, props)
        } else {
            return container([box_grow({ body: notice_gray(["このデータは削除されました"]) })])
        }
    }

    function backLink(): PreactNode {
        return h(BackLink, { onClick })

        function onClick() {
            props.register.focus.close()
        }
    }
}
