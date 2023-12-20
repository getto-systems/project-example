import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../common"

import { buttons } from "../design/form"

import { SiteInfo } from "../../site"

export type AppLayoutContent = Readonly<{
    siteInfo: SiteInfo
    header: readonly PreactContent[]
    main: PreactContent
    menu: PreactContent
}> &
    Partial<{
        sidebar: PreactContent
        sidebar_double: PreactContent
    }>

export function appLayout(content: AppLayoutContent): PreactNode {
    const { siteInfo, header, main, menu } = content
    if (content.sidebar) {
        return layoutContent("sidebar_single", siteInfo, header, [
            appBodyContainer([main, content.sidebar]),
            menu,
        ])
    }
    if (content.sidebar_double) {
        return layoutContent("sidebar_double", siteInfo, header, [
            appBodyContainer([main, content.sidebar_double]),
            menu,
        ])
    }
    return layoutContent("normal", siteInfo, header, [appBodyContainer([main]), menu])
}

type AppLayoutType = "normal" | "sidebar_single" | "sidebar_double"
function toAppLayoutClass(type: AppLayoutType): string {
    switch (type) {
        case "normal":
            return ""

        case "sidebar_single":
        case "sidebar_double":
            return `layout__app__${type}`
    }
}

function layoutContent(
    type: AppLayoutType,
    siteInfo: SiteInfo,
    header: readonly PreactContent[],
    content: readonly PreactContent[],
) {
    return html`<main class="layout__app ${toAppLayoutClass(type)}">
        ${appHeader(siteInfo, header)}
        <section class="layout__app__body">${content}</section>
    </main>`
}

function appHeader(
    { brand, title, subTitle }: SiteInfo,
    header: readonly PreactContent[],
): PreactNode {
    return html`<header class="app__header">${logo()} ${header.map(box)}</header>`

    function logo() {
        return html`<aside class="app__logo">
            <cite class="app__logo__brand">${brand}</cite>
            <strong class="app__logo__title">${title}</strong>
            <cite class="app__logo__subTitle">${subTitle}</cite>
        </aside>`
    }
    function box(content: PreactContent) {
        return html`<section class="app__header__box">${content}</section>`
    }
}
function appBodyContainer(content: readonly PreactContent[]): PreactNode {
    return html`<section class="layout__app__body__container">${content}</section>`
}

export type MainLayoutContent = Readonly<{
    header: PreactContent
    body: PreactContent
    copyright: PreactContent
}>
export function appMain({ header, body, copyright }: MainLayoutContent): PreactNode {
    return html`<article>${header} ${body} ${mainFooter(copyright)}</article>`
}
export function appSidebar({ header, body, copyright }: MainLayoutContent): PreactNode {
    return html`<aside class="sidebar">${header} ${body} ${mainFooter(copyright)}</aside>`
}
export function appMenu(content: PreactContent): PreactNode {
    return html`<aside class="menu">${content}</aside>`
}

export function mainHeader(content: PreactContent): PreactNode {
    return html`<header class="main__header">${content}</header>`
}

export function mainTitle(content: PreactContent): PreactNode {
    return html`<h1 class="main__title">${content}</h1>`
}
export function mainTitleWithSidebarButton({
    title,
    button,
}: Readonly<{ title: PreactContent; button: PreactContent }>): PreactNode {
    return mainTitle(
        buttons({
            left: title,
            right: html`<div class="sidebar__button">${button}</div>`,
        }),
    )
}

export function mainBody(content: PreactContent): PreactNode {
    return html`<section class="main__body">${content}</section>`
}

export type SidebarBodyProps = Partial<{
    id: string
}>
export function sidebarBody(content: PreactContent, props: SidebarBodyProps = {}): PreactNode {
    return html`<section id="${props.id}" class="sidebar__body">${content}</section>`
}
export function sidebarBody_grow(content: PreactContent, props: SidebarBodyProps = {}): PreactNode {
    return html`<section id="${props.id}" class="sidebar__body sidebar__body_grow">
        ${content}
    </section>`
}

export function mainFooter(copyright: PreactContent): PreactNode {
    return html`<footer class="main__footer">
        <p class="main__footer__message">${copyright}</p>
    </footer>`
}

export function menuBox(content: PreactContent): PreactNode {
    return html`<section class="menu__box">${content}</section>`
}

export function menuBody(id: string, content: PreactContent): PreactNode {
    return html`<nav id=${id} class="menu__body">${content}</nav>`
}

export type MenuCategoryContent = Readonly<{
    isExpand: boolean
    label: string
    show: Handler<Event>
    hide: Handler<Event>
    badge: PreactContent
    children: PreactContent
}>
export function menuCategory({
    isExpand,
    label,
    show,
    hide,
    badge,
    children,
}: MenuCategoryContent): PreactNode {
    return html`<details class="menu__nav" open=${isExpand} key=${label}>
        <summary class="menu__nav__summary" onClick=${isExpand ? hide : show}>
            <span class="menu__nav__summary__container">
                <span class="menu__nav__summary__label">${label}</span>
                <span class="menu__nav__summary__badge">${badge}</span>
            </span>
        </summary>
        <ul class="menu__nav__items">
            ${children}
        </ul>
    </details>`
}

export type MenuItemContent = Readonly<{
    isActive: boolean
    href: string
    content: PreactContent
    badge: PreactContent
}>
export function menuItem({ isActive, href, content, badge }: MenuItemContent): PreactNode {
    const activeClass = isActive ? "menu__nav__item_active" : ""

    return html`<li class="menu__nav__item" key=${href}>
        <a class="menu__nav__link ${activeClass}" href=${href}>
            <span class="menu__nav__item__label">${content}</span>
            <span class="menu__nav__item__badge">${badge}</span>
        </a>
    </li>`
}

export function menuFooter(poweredBy: readonly string[]): PreactNode {
    return html`<footer class="menu__footer">
        <p class="menu__footer__message">
            powered by :<br />
            <span class="noWrap">${poweredBy.join(" / ")}</span>
        </p>
    </footer>`
}

export function mainBreadcrumbList(content: PreactContent): PreactNode {
    return html`<aside class="main__breadcrumb">${content}</aside>`
}
export function mainBreadcrumbLink(href: string, content: PreactContent): PreactNode {
    return html`<a class="main__breadcrumb__item" href="${href}">${content}</a>`
}
export function mainBreadcrumbSeparator(content: PreactContent): PreactNode {
    return html`<span class="main__breadcrumb__separator">${content}</span>`
}

interface Handler<T> {
    (event: T): void
}
