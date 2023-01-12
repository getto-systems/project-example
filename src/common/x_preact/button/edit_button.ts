import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../vnode"

import { button_edit } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_edit } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

type Props = Readonly<{
    label?: VNodeContent
    onClick: { (e: Event): void }
}>
export function EditButton({ label, onClick }: Props): VNode {
    return button_edit({
        state: "normal",
        label: html`${label || "変更"} ${iconHtml(icon_edit)}`,
        onClick,
    })
}
