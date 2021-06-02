import { html } from "htm/preact"
import { VNode } from "preact"

import { SignNavItem } from "../data"

export function signNav(nav: SignNavItem): VNode {
    return html`<a href="${nav.href}"><i class="${nav.icon}"></i> ${nav.label}</a>`
}
