import { html } from "htm/preact"
import { VNode } from "preact"
import { iconHtml } from "../../../../z_lib/ui/icon/x_preact/icon"

import { SignNavItem } from "../data"

export function signNav(nav: SignNavItem): VNode {
    return html`<a href="${nav.href}">${iconHtml(nav.icon)} ${nav.label}</a>`
}
