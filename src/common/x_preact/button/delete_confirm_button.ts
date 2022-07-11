import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { button_delete } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_delete } from "../../../x_content/icon"
import { iconHtml } from "../../../z_lib/ui/icon/x_preact/icon"

import { Icon } from "../../../z_lib/ui/icon/data"

type Props = Readonly<{
    label?: VNodeContent
    icon?: Icon
    onClick: { (e: Event): void }
}>
export function DeleteConfirmButton({ label, icon, onClick }: Props): VNode {
    return button_delete({
        state: "normal",
        label: html`${label || "削除"} ${iconHtml(icon || icon_delete)}`,
        onClick,
    })
}
