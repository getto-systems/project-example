import { VNode } from "preact"
import { html } from "htm/preact"

import { Icon } from "../../../z_lib/ui/icon/data"
import { lnir } from "../../../z_lib/ui/icon/line_icon"

export function icon(icon: Icon): VNode {
    return html`<i class="${icon}"></i>`
}
export const spinner: VNode = icon(lnir(["spinner", "is-spinning"]))
