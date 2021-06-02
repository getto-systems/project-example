import { VNode } from "preact"
import { html } from "htm/preact"

export function noPaddedStory(content: VNode): VNode {
    return html`
        <style>
            .sb-main-padded {
                padding: 0 !important;
            }
        </style>
        ${content}
    `
}
