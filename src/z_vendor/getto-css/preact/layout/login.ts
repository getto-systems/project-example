import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../common"

import { form } from "../design/form"

import { SiteInfo } from "../../site"

export type LoginBoxContent = Readonly<{ title: PreactContent; body: PreactContent }> &
    Partial<{ footer: PreactContent; form: boolean }>
export function loginBox(siteInfo: SiteInfo, content: LoginBoxContent): PreactNode {
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

    function logo({ brand, title, subTitle }: SiteInfo): PreactNode {
        return html`<header class="loginBox__logo">
            <cite class="loginBox__logo__brand">${brand}</cite>
            <strong class="loginBox__logo__title">${title}</strong>
            <cite class="loginBox__logo__subTitle">${subTitle}</cite>
        </header>`
    }
    function body(): PreactContent {
        if (content.form) {
            return form(content.body)
        }
        return content.body
    }
    function footer(): PreactContent {
        if (content.footer) {
            return content.footer
        }
        return ""
    }
}
