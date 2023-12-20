import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../common"

export function container(content: PreactContent): PreactNode {
    return html`<section class="container">${content}</section>`
}
export function container_top(content: PreactContent): PreactNode {
    return html`<section class="container container_top">${content}</section>`
}

export type BoxContent = Readonly<{ body: PreactContent }> &
    Partial<{
        form: boolean
        title: PreactContent
        footer: PreactContent
    }>

type BoxClass = "single" | "double" | "grow"
function mapBoxClass(boxClass: BoxClass): string {
    switch (boxClass) {
        case "single":
            return ""

        default:
            return `box_${boxClass}`
    }
}

export function box(content: BoxContent): PreactNode {
    return boxContent("single", content)
}
export function box_double(content: BoxContent): PreactNode {
    return boxContent("double", content)
}
export function box_grow(content: BoxContent): PreactNode {
    return boxContent("grow", content)
}

export function box_transparent(content: PreactContent): PreactNode {
    return boxTransparent("single", content)
}
export function box_double_transparent(content: PreactContent): PreactNode {
    return boxTransparent("double", content)
}
export function box_grow_transparent(content: PreactContent): PreactNode {
    return boxTransparent("grow", content)
}

function boxContent(boxClass: BoxClass, content: BoxContent): PreactNode {
    if (content.form) {
        return html`<form class="${classAttribute()}">${inner()}</form>`
    } else {
        return html`<article class="${classAttribute()}">${inner()}</article>`
    }

    function classAttribute(): string {
        return `box ${mapBoxClass(boxClass)}`
    }
    function inner(): PreactNode {
        return html`
            <main>${header()} ${boxBody(content.body)}</main>
            ${footer()}
        `
    }

    function header(): PreactContent {
        if (content.title) {
            return boxHeader(content.title)
        }
        return ""
    }
    function footer() {
        if (content.footer) {
            return boxFooter(content.footer)
        }
        return ""
    }
}
function boxTransparent(boxClass: BoxClass, content: PreactContent): PreactNode {
    return html`<article class="box box_transparent ${mapBoxClass(boxClass)}">${content}</article>`
}

function boxHeader(title: PreactContent) {
    return html`<header class="box__header">
        <h2>${title}</h2>
    </header>`
}
function boxBody(body: PreactContent) {
    return html`<section class="box__body">${body}</section>`
}
function boxFooter(footer: PreactContent) {
    return html`<footer class="box__footer">${footer}</footer>`
}

export type ModalContent = Readonly<{
    title: PreactContent
    body: PreactContent
    footer: PreactContent
}>

export function modalBox({ title, body, footer }: ModalContent): PreactNode {
    return html`<aside class="modal">
        <aside class="modal__container">
            <section class="modal__box">
                ${modalHeader(title)} ${modalBody(body)} ${modalFooter(footer)}
            </section>
        </aside>
    </aside>`
}

export type ModalFixedContent = Readonly<{
    title: PreactContent
    body: PreactContent
}>

export function modalBoxFixed({ title, body }: ModalFixedContent): PreactNode {
    return html`<aside class="modal">
        <aside class="modal__container">
            <section class="modal__box_fixed">${modalHeader(title)} ${modalBody(body)}</section>
        </aside>
    </aside>`
}

function modalHeader(title: PreactContent) {
    return html`<header class="modal__header">
        <h3 class="modal__title">${title}</h3>
    </header>`
}
function modalBody(content: PreactContent) {
    return html`<section class="modal__body">${content}</section>`
}
function modalFooter(footer: PreactContent) {
    return html`<footer class="modal__footer">${footer}</footer>`
}
