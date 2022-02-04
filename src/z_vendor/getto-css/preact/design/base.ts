import { VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../common"

export function paragraph(content: VNodeContent): VNode {
    return html`<div class="paragraph">${content}</div>`
}
