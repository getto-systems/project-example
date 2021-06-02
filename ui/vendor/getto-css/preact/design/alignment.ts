import { VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../common"

export function small(content: VNodeContent): VNode {
    return html`<small>${content}</small>`
}
export function big(content: VNodeContent): VNode {
    return html`<big>${content}</big>`
}

type Size = "small" | "medium" | "large"

export function v_small(): VNode {
    return vertical("small")
}
export function v_medium(): VNode {
    return vertical("medium")
}
export function v_large(): VNode {
    return vertical("large")
}
function vertical(size: Size): VNode {
    return html`<div class="vertical vertical_${size}"></div>`
}
