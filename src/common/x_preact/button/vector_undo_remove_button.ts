import { VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../../../z_vendor/getto-css/preact/common"

import { button_undo } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_undo } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Icon } from "../../util/icon/data"

type Props = Readonly<{
    label?: VNodeContent
    icon?: Icon
    onClick: { (e: Event): void }
}>
export function VectorUndoRemoveButton({ label, icon, onClick }: Props): VNode {
    return button_undo({ label: buttonLabel(icon || icon_undo), onClick })

    function buttonLabel(icon: Icon): VNode {
        return html`${label || "戻す"} ${iconHtml(icon)}`
    }
}
