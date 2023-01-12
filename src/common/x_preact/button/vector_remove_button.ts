import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../z_vendor/getto-css/preact/common"

import { button_delete } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_remove } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Icon } from "../../util/icon/data"

type Props = Readonly<{
    label?: VNodeContent
    icon?: Icon
    onClick: { (e: Event): void }
}>
export function VectorRemoveButton({ label, icon, onClick }: Props): VNode {
    return button_delete({ state: "normal", label: buttonLabel(icon || icon_remove), onClick })

    function buttonLabel(icon: Icon): VNode {
        return html`${label || "削除"} ${iconHtml(icon)}`
    }
}
