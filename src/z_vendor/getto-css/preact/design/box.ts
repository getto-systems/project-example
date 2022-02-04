import { VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../common"

export function container(content: VNodeContent): VNode {
    return html`<section class="container">${content}</section>`
}
export function container_top(content: VNodeContent): VNode {
    return html`<section class="container container_top">${content}</section>`
}

export type BoxContent = Readonly<{ body: VNodeContent }> &
    Partial<{
        form: boolean
        title: VNodeContent
        footer: VNodeContent
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

export function box(content: BoxContent): VNode {
    return boxContent("single", content)
}
export function box_double(content: BoxContent): VNode {
    return boxContent("double", content)
}
export function box_grow(content: BoxContent): VNode {
    return boxContent("grow", content)
}

export function box_transparent(content: VNodeContent): VNode {
    return boxTransparent("single", content)
}
export function box_double_transparent(content: VNodeContent): VNode {
    return boxTransparent("double", content)
}
export function box_grow_transparent(content: VNodeContent): VNode {
    return boxTransparent("grow", content)
}

function boxContent(boxClass: BoxClass, content: BoxContent): VNode {
    if (content.form) {
        return html`<form class="${classAttribute()}">${inner()}</form>`
    } else {
        return html`<article class="${classAttribute()}">${inner()}</article>`
    }

    function classAttribute(): string {
        return `box ${mapBoxClass(boxClass)}`
    }
    function inner(): VNode {
        return html`
            <main>${header()} ${boxBody(content.body)}</main>
            ${footer()}
        `
    }

    function header(): VNodeContent {
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
function boxTransparent(boxClass: BoxClass, content: VNodeContent): VNode {
    return html`<article class="box box_transparent ${mapBoxClass(boxClass)}">${content}</article>`
}

function boxHeader(title: VNodeContent) {
    return html`<header class="box__header">
        <h2>${title}</h2>
    </header>`
}
function boxBody(body: VNodeContent) {
    return html`<section class="box__body">${body}</section>`
}
function boxFooter(footer: VNodeContent) {
    return html`<footer class="box__footer">${footer}</footer>`
}

export type ModalContent = Readonly<{
    title: VNodeContent
    body: VNodeContent
    footer: VNodeContent
}>

export function modalBox({ title, body, footer }: ModalContent): VNode {
    return html`<aside class="modal">
        <section class="modal__box">
            ${modalHeader(title)} ${modalBody(body)} ${modalFooter(footer)}
        </section>
    </aside>`
}

function modalHeader(title: VNodeContent) {
    return html`<header class="modal__header">
        <h3 class="modal__title">${title}</h3>
    </header>`
}
function modalBody(content: VNodeContent) {
    return html`<section class="modal__body">${content}</section>`
}
function modalFooter(footer: VNodeContent) {
    return html`<footer class="modal__footer">${footer}</footer>`
}
