import { VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../common"
import { SiteInfo } from "../../site"

export type LoginBoxContent = LoginBoxContent_base | (LoginBoxContent_base & LoginBoxContent_footer)
type LoginBoxContent_base = Readonly<{ title: VNodeContent; body: VNodeContent }>
type LoginBoxContent_footer = Readonly<{ footer: VNodeContent }>

export function loginBox(siteInfo: SiteInfo, content: LoginBoxContent): VNode {
    return html`<aside class="layout__login">
        <section class="loginBox">
            ${logo(siteInfo)}
            <article class="loginBox__main">
                <header class="loginBox__main__header">
                    <h1 class="loginBox__main__title">${content.title}</h1>
                </header>
                <main class="loginBox__main__body">${content.body}</main>
                <footer class="loginBox__main__footer">${footer()}</footer>
            </article>
        </section>
    </aside>`

    function logo({ brand, title, subTitle }: SiteInfo): VNode {
        return html`<header class="loginBox__logo">
            <cite class="loginBox__logo__brand">${brand}</cite>
            <strong class="loginBox__logo__title">${title}</strong>
            <cite class="loginBox__logo__subTitle">${subTitle}</cite>
        </header>`
    }
    function footer(): VNodeContent {
        if ("footer" in content) {
            return content.footer
        }
        return ""
    }
}
