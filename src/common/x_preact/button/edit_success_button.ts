import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { button_edit } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_ok } from "../../../x_content/icon"
import { iconHtml } from "../design/icon"

type Props = Readonly<{
    label?: VNodeContent
    onClick: { (e: Event): void }
}>
export function EditSuccessButton({ label, onClick }: Props): VNode {
    return button_edit({
        state: "normal",
        label: html`${label || "変更"} ${iconHtml(icon_ok)}`,
        onClick,
    })
}
