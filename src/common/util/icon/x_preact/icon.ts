import { html } from "htm/preact"
import { PreactNode } from "../../../x_preact/vnode"

import { Icon } from "../data"

export function iconHtml(icon: Icon): PreactNode {
    return html`<i class="${icon}"></i>`
}
