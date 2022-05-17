import { html } from "htm/preact"
import { VNode } from "preact"
import { iconHtml } from "../../../../common/x_preact/design/icon"

import { SignNavItem } from "../data"

export function signNav(nav: SignNavItem): VNode {
    return html`<a href="${nav.href}">${iconHtml(nav.icon)} ${nav.label}</a>`
}
