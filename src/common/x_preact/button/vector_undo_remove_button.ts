import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../node"

import { button_undo } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_undo } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Icon } from "../../util/icon/data"

export function VectorUndoRemoveButton({
    label,
    icon,
    onClick,
}: Readonly<{
    label?: PreactContent
    icon?: Icon
    onClick: { (e: Event): void }
}>): PreactNode {
    return button_undo({ label: buttonLabel(icon || icon_undo), onClick })

    function buttonLabel(icon: Icon): PreactNode {
        return html`${label || "戻す"} ${iconHtml(icon)}`
    }
}
