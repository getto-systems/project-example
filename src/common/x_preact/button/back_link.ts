import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../vnode"

import { iconHtml } from "../../util/icon/x_preact/icon"
import { icon_back } from "../../../x_content/icon"

export function BackLink({
    label,
    onClick,
}: Readonly<{
    label?: PreactContent
    onClick: { (e: Event): void }
}>): PreactNode {
    return html`<a href="#" onClick=${onClick}>${label || html`${iconHtml(icon_back)} 戻る`}</a>`
}
