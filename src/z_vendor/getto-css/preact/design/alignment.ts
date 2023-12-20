import { html } from "htm/preact"

import { PreactContent, PreactNode } from "../common"

export function small(content: PreactContent): PreactNode {
    return html`<small>${content}</small>`
}
export function big(content: PreactContent): PreactNode {
    return html`<big>${content}</big>`
}

type Size = "small" | "medium" | "large"

export function v_small(): PreactNode {
    return vertical("small")
}
export function v_medium(): PreactNode {
    return vertical("medium")
}
export function v_large(): PreactNode {
    return vertical("large")
}
function vertical(size: Size): PreactNode {
    return html`<div class="vertical vertical_${size}"></div>`
}
