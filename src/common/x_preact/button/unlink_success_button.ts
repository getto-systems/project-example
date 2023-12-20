import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../vnode"

import { button_delete } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_ok } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

export function UnlinkSuccessButton({
    label,
    onClick,
}: Readonly<{
    label?: PreactContent
    onClick: { (e: Event): void }
}>): PreactNode {
    return button_delete({
        state: "normal",
        label: html`${label || "解除"} ${iconHtml(icon_ok)}`,
        onClick,
    })
}
