import { VNode } from "preact"
import { html } from "htm/preact"

import { v_medium, v_small } from "../../../../../ui/vendor/getto-css/preact/design/alignment"
import {
    label_alert,
    label_pending,
} from "../../../../../ui/vendor/getto-css/preact/design/highlight"
import { box } from "../../../../../ui/vendor/getto-css/preact/design/box"
import { field } from "../../../../../ui/vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../z_lib/ui/x_preact/common"
import { icon } from "../../design/icon"

export function itemsSection(title: VNodeContent, list: readonly VNodeContent[]): VNode {
    return html`
        <p>${title}</p>
        ${v_small()} ${items(list)} ${v_medium()}
    `
}
export function items(list: readonly VNodeContent[]): VNode {
    return html`<ul>
        ${list.map(item)}
    </ul>`

    function item(content: VNodeContent): VNode {
        return html`<li>
            <small><i class="lnir lnir-chevron-right"></i></small>
            ${" "} ${content}
        </li>`
    }
}

export function negativeNote(content: VNodeContent, resolve: VNodeContent): VNode {
    return iconSection(icon("close"), content, resolve)
}
export function iconSection(icon: VNode, content: VNodeContent, note: VNodeContent): VNode {
    return html`
        <p>${icon} ${content}</p>
        <small><p>${note}</p></small>
        ${v_small()}
    `
}

export function pending(content: VNodeContent): VNode {
    return html`${content} ${label_pending("あとで")}`
}
export function validate(content: VNodeContent): VNode {
    return html`${label_alert("検証")} ${content}`
}

export const hr: VNode = html`<hr />`

const contentServer = html`${icon("database")} コンテンツサーバー`
const apiServer = html`${icon("cogs")} API サーバー`
const browser = html`${icon("display")} ブラウザ`
const textMessage = html`${icon("envelope")} テキストメッセージクライアント`

export function serverClients(): VNode {
    return box({
        title: "前提とするサーバー・クライアント",
        body: [
            iconSection(icon("database"), "コンテンツサーバー", "（CDN : CloudFront など）"),
            iconSection(icon("cogs"), "API サーバー", "（アプリケーションサーバー）"),
            iconSection(icon("display"), "http クライアント", "（ブラウザ、スマホアプリ）"),
            iconSection(icon("envelope"), "テキストメッセージクライアント", "（メール、slack）"),
        ],
    })
}

export function inBrowser(content: VNodeContent, help: readonly VNodeContent[]): VNode {
    return field({ title: browser, body: content, help })
}

export function toContentServer(content: VNodeContent, help: readonly VNodeContent[]): VNode {
    return toServer(contentServer, content, help)
}
export function inContentServer(content: VNodeContent, help: readonly VNodeContent[]): VNode {
    return field({ title: contentServer, body: content, help })
}

export function toApiServer(content: VNodeContent, help: readonly VNodeContent[]): VNode {
    return toServer(apiServer, content, help)
}
export function inApiServer(content: VNodeContent, help: readonly VNodeContent[]): VNode {
    return field({ title: apiServer, body: content, help })
}
export function fromApiServer(content: VNodeContent, help: readonly VNodeContent[]): VNode {
    return fromServer(apiServer, content, help)
}

export function toTextMessage(content: VNodeContent, help: readonly VNodeContent[]): VNode {
    return field({
        title: html`${apiServer} ${icon("arrow-right")} ${textMessage}`,
        body: content,
        help,
    })
}

function toServer(server: VNodeContent, content: VNodeContent, help: readonly VNodeContent[]) {
    return field({
        title: html`${browser} ${icon("arrow-right")} ${server}`,
        body: content,
        help,
    })
}
function fromServer(server: VNodeContent, content: VNodeContent, help: readonly VNodeContent[]) {
    return field({
        title: html`${server} ${icon("arrow-right")} ${browser}`,
        body: content,
        help,
    })
}
