import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../node"

import { useAtom } from "../../../z_vendor/getto-atom/x_preact/hooks"

import { button_search } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_search, icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Atom } from "../../../z_vendor/getto-atom/atom"
import { ObserveBoardState } from "../../util/board/observe/action"

import { Icon } from "../../util/icon/data"
import { ConnectState } from "../../util/connect/data"

export function SearchButton({
    label,
    icon,
    connect,
    observe,
    onClick,
}: Readonly<{
    label?: PreactContent
    icon?: Icon
    connect: Atom<ConnectState>
    observe: Atom<ObserveBoardState>
    onClick: { (e: Event): void }
}>): PreactNode {
    const connectState = useAtom(connect)
    const observeState = useAtom(observe)

    if (connectState.isConnecting) {
        return button_search({ state: "connect", label: iconLabel(icon_spinner) })
    }

    return button_search({
        state: observeState.hasChanged ? "confirm" : "normal",
        label: iconLabel(icon || icon_search),
        onClick,
    })

    function iconLabel(icon: Icon): PreactNode {
        return html`${label || "検索"} ${iconHtml(icon)}`
    }
}
