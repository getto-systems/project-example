import { VNode } from "preact"
import { html } from "htm/preact"

import { Icon } from "../../../z_lib/ui/icon/data"
import { lnir } from "../../../z_lib/ui/icon/init/line_icon"

export function iconHtml(icon: Icon): VNode {
    return html`<i class="${icon}"></i>`
}
export const icon_home: Icon = lnir(["flags"])
export const icon_spinner: Icon = lnir(["spinner", "is-spinning"])
export const icon_search: Icon = lnir(["search"])
