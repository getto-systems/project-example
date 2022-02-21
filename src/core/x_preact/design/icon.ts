import { VNode } from "preact"
import { html } from "htm/preact"

import { Icon } from "../../../z_lib/ui/icon/data"
import { lnir } from "../../../z_lib/ui/icon/init/line_icon"

export function icon(icon: Icon): VNode {
    return html`<i class="${icon}"></i>`
}
export const home: Icon = lnir(["flags"])
export const spinner: Icon = lnir(["spinner", "is-spinning"])
