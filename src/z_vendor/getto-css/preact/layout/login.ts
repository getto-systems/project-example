import { VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../common"

import { form } from "../design/form"

import { SiteInfo } from "../../site"

export type LoginBoxContent = Readonly<{ title: VNodeContent; body: VNodeContent }> &
    Partial<{ footer: VNodeContent; form: boolean }>
export function loginBox(siteInfo: SiteInfo, content: LoginBoxContent): VNode {
    return html`<aside class="layout__login">
        <section class="loginBox">
            ${logo(siteInfo)}
            <article class="loginBox__main">
                <header class="loginBox__main__header">
                    <h1 class="loginBox__main__title">${content.title}</h1>
                </header>
                <main class="loginBox__main__body">${body()}</main>
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
    function body(): VNodeContent {
        if (content.form) {
            return form(content.body)
        }
        return content.body
    }
    function footer(): VNodeContent {
        if (content.footer) {
            return content.footer
        }
        return ""
    }
}
