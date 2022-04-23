import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { button_delete } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_delete, icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "../design/icon"

import { Icon } from "../../../z_lib/ui/icon/data"

type Props = Readonly<{
    label?: VNodeContent
    icon?: Icon
    isConnecting: boolean
    onClick: { (e: Event): void }
}>
export function DeleteButton({ isConnecting, label, icon, onClick }: Props): VNode {
    if (isConnecting) {
        return button_delete({ state: "connect", label: buttonLabel(icon_spinner) })
    }

    return button_delete({ state: "confirm", label: buttonLabel(icon || icon_delete), onClick })

    function buttonLabel(icon: Icon): VNode {
        return html`${label || "削除"} ${iconHtml(icon)}`
    }
}
