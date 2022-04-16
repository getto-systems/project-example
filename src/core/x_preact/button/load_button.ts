import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { button_search } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_search, icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "../design/icon"

import { Icon } from "../../../z_lib/ui/icon/data"

type Props = Readonly<{
    label?: VNodeContent
    icon?: Icon
    isConnecting: boolean
    onClick: { (e: Event): void }
}>
export function LoadButton({ isConnecting, label, icon, onClick }: Props): VNode {
    const buttonLabel = label || "表示"
    const buttonIcon = icon || icon_search

    if (isConnecting) {
        return button_search({ state: "connect", label: iconLabel(icon_spinner) })
    }

    return button_search({ state: "normal", label: iconLabel(buttonIcon), onClick })

    function iconLabel(icon: Icon): VNode {
        return html`${buttonLabel} ${iconHtml(icon)}`
    }
}
