import { html } from "htm/preact"
import { PreactNode } from "../../../../common/x_preact/vnode"

import { iconHtml } from "../../../../common/util/icon/x_preact/icon"

import { SignNavItem } from "../data"

export function signNav(nav: SignNavItem): PreactNode {
    return html`<a href="${nav.href}">${iconHtml(nav.icon)} ${nav.label}</a>`
}
