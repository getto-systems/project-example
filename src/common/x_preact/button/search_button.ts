import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../vnode"

import { button_search } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_search, icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { ObserveBoardState } from "../../../z_vendor/getto-application/board/observe_board/action"

import { Icon } from "../../util/icon/data"

type Props = Readonly<{
    label?: VNodeContent
    icon?: Icon
    isConnecting: boolean
    observeState: ObserveBoardState
    onClick: { (e: Event): void }
}>
export function SearchButton({ isConnecting, observeState, label, icon, onClick }: Props): VNode {
    if (isConnecting) {
        return button_search({ state: "connect", label: iconLabel(icon_spinner) })
    }

    return button_search({
        state: observeState.hasChanged ? "confirm" : "normal",
        label: iconLabel(icon || icon_search),
        onClick,
    })

    function iconLabel(icon: Icon): VNode {
        return html`${label || "検索"} ${iconHtml(icon)}`
    }
}
