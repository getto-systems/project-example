import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { button_search } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_search } from "../../../x_content/icon"
import { iconHtml } from "../../../z_lib/ui/icon/x_preact/icon"

import { Icon } from "../../../z_lib/ui/icon/data"

type Props = Readonly<{
    label?: VNodeContent
    icon?: Icon
    onClick: { (e: Event): void }
}>
export function LoadButton({ label, icon, onClick }: Props): VNode {
    return button_search({ state: "normal", label: iconLabel(icon || icon_search), onClick })

    function iconLabel(icon: Icon): VNode {
        return html`${label || "表示"} ${iconHtml(icon)}`
    }
}
