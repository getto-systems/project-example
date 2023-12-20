import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../common"

export function paragraph(content: PreactContent): PreactNode {
    return html`<div class="paragraph">${content}</div>`
}
