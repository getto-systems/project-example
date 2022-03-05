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
export const icon_reload: Icon = lnir(["spinner-arrow"])
export const icon_edit: Icon = lnir(["pencil"])
export const icon_edit_focused: Icon = lnir(["pencil-alt"])
export const icon_save: Icon = lnir(["bolt"])
