import { h } from "preact"
import { html } from "htm/preact"
import { PreactNode } from "../../../../../common/x_preact/node"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import { box, box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { DetailAuthUserAccount, DetailAuthUserAccountActions } from "../../kernel/x_preact/detail"
import { BackLink } from "../../../../../common/x_preact/button/back_link"

import { SearchAuthUserAccountAction } from "../action"

type Props = DetailAuthUserAccountActions &
    Readonly<{
        search: SearchAuthUserAccountAction
    }>
export function FocusAuthUserAccount(props: Props): PreactNode {
    return html`${[container([box({ body: backLink() })]), h(Content, {})]}`

    function Content(_props: unknown): PreactNode {
        const isFocused = useAtom(props.search.focus.isSomeEntryFocused)
        if (isFocused) {
            return h(DetailAuthUserAccount, props)
        } else {
            return container([
                box_grow({ body: notice_gray(["指定されたデータが見つかりませんでした"]) }),
            ])
        }
    }

    function backLink(): PreactNode {
        return h(BackLink, { onClick })

        function onClick() {
            props.search.focus.close({ y: document.getElementById("sidebar")?.scrollTop || 0 })
        }
    }
}
