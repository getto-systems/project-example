import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../vnode"

import { button_edit } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_edit } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

export function EditButton({
    label,
    onClick,
}: Readonly<{
    label?: PreactContent
    onClick: { (e: Event): void }
}>): PreactNode {
    return button_edit({
        state: "normal",
        label: html`${label || "変更"} ${iconHtml(icon_edit)}`,
        onClick,
    })
}
