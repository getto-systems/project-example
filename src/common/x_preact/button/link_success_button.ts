import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../vnode"

import { button_send } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_ok } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

export function LinkSuccessButton({
    label,
    onClick,
}: Readonly<{
    label?: PreactContent
    onClick: { (e: Event): void }
}>): PreactNode {
    return button_send({
        state: "normal",
        label: html`${label || "登録"} ${iconHtml(icon_ok)}`,
        onClick,
    })
}
