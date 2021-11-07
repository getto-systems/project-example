import { VNode } from "preact"
import { html } from "htm/preact"

import { lniClass, lnir } from "../../../z_lib/ui/icon/line_icon"

export function icon(name: string): VNode {
    return html`<i class="${lniClass(lnir(name))}"></i>`
}
export const spinner: VNode = html`<i class="lnir lnir-spinner lnir-is-spinning"></i>`
