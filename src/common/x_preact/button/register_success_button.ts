import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../vnode"

import { button_send } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_ok } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

type Props = Readonly<{
    label?: VNodeContent
    onClick: { (e: Event): void }
}>
export function RegisterSuccessButton({ label, onClick }: Props): VNode {
    return button_send({
        state: "normal",
        label: html`${label || "登録"} ${iconHtml(icon_ok)}`,
        onClick,
    })
}
