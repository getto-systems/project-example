import { VNode } from "preact"
import { html } from "htm/preact"

import { Icon } from "../data"

export function iconHtml(icon: Icon): VNode {
    return html`<i class="${icon}"></i>`
}

