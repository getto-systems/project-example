import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../vnode"

import { button_send } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_add } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Icon } from "../../util/icon/data"

export function VectorAddButton({
    label,
    icon,
    onClick,
}: Readonly<{
    label?: PreactContent
    icon?: Icon
    onClick: { (e: Event): void }
}>): PreactNode {
    return button_send({
        state: "normal",
        submit: false,
        label: buttonLabel(icon || icon_add),
        onClick,
    })

    function buttonLabel(icon: Icon): PreactNode {
        return html`${label || "項目を追加"} ${iconHtml(icon)}`
    }
}
