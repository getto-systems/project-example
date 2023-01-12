import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../vnode"

import { button_send } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_add } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Icon } from "../../util/icon/data"

type Props = Readonly<{
    label?: VNodeContent
    icon?: Icon
    onClick: { (e: Event): void }
}>
export function VectorAddButton({ label, icon, onClick }: Props): VNode {
    return button_send({
        state: "normal",
        submit: false,
        label: buttonLabel(icon || icon_add),
        onClick,
    })

    function buttonLabel(icon: Icon): VNode {
        return html`${label || "項目を追加"} ${iconHtml(icon)}`
    }
}
