import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../vnode"

import { button_search } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_search } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Icon } from "../../util/icon/data"

export function LoadButton({
    label,
    icon,
    onClick,
}: Readonly<{
    label?: PreactContent
    icon?: Icon
    onClick: { (e: Event): void }
}>): PreactNode {
    return button_search({ state: "normal", label: iconLabel(icon || icon_search), onClick })

    function iconLabel(icon: Icon): PreactNode {
        return html`${label || "表示"} ${iconHtml(icon)}`
    }
}
