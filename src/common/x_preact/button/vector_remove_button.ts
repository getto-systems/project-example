import { html } from "htm/preact"
import { PreactContent } from "../../../z_vendor/getto-css/preact/common"

import { button_delete } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_remove } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Icon } from "../../util/icon/data"
import { PreactNode } from "../vnode"

export function VectorRemoveButton({
    label,
    icon,
    onClick,
}: Readonly<{
    label?: PreactContent
    icon?: Icon
    onClick: { (e: Event): void }
}>): PreactNode {
    return button_delete({ state: "normal", label: buttonLabel(icon || icon_remove), onClick })

    function buttonLabel(icon: Icon): PreactNode {
        return html`${label || "削除"} ${iconHtml(icon)}`
    }
}
