import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../../ui/vendor/getto-css/preact/common"

import { v_small } from "../../../../ui/vendor/getto-css/preact/design/alignment"
import { box, box_double, container } from "../../../../ui/vendor/getto-css/preact/design/box"
import {
    label_alert,
    label_pending,
    notice_info,
} from "../../../../ui/vendor/getto-css/preact/design/highlight"
import { field } from "../../../../ui/vendor/getto-css/preact/design/form"

import { icon } from "../../../example/_ui/x_preact/design/icon"

import {
    DocsAction,
    DocsActionContent,
    DocsActionTargetType,
    DocsContent,
    DocsDescription,
    DocsNegativeNote,
    DocsSection,
} from "../../../../ui/vendor/getto-application/docs/data"

export function docsArticle(contents: DocsSection[][][]): VNode {
    return paddingVSpace(
        contents.map((sectionsArr) =>
            sectionsArr.map((sections) =>
                container(
                    sections.map((section) => {
                        const content = {
                            title: docsSectionTitle(section),
                            body: paddingVSpace(section.body.map(docsSectionBody), v_small()),
                        }

                        switch (section.type) {
                            case "normal":
                            case "pending":
                                return box(content)

                            case "double":
                                return box_double(content)
                        }
                    }),
                ),
            ),
        ),
        v_small(),
    )

    function paddingVSpace(contents: VNodeContent[], space: VNode): VNode {
        return html`${pad()}`

        function pad(): VNodeContent[] {
            return contents.reduce((acc, content) => {
                if (acc.length > 0) {
                    acc.push(space)
                }
                acc.push(content)
                return acc
            }, <VNodeContent[]>[])
        }
    }
}

export function docsSectionTitle(section: DocsSection): VNodeContent {
    switch (section.type) {
        case "normal":
        case "double":
            return section.title

        case "pending":
            return html`${section.title} ${label_pending("あとで")}`
    }
}
export function docsSectionBody(content: DocsContent): VNodeContent {
    switch (content.type) {
        case "purpose":
            return purpose(content.content)

        case "module":
            return module(content.content)

        case "item":
            return item(content.title, content.content)

        case "description":
            return description(content.content)

        case "explanation":
            return explanation(content.target)

        case "negativeNote":
            return negativeNote(content.content)

        case "action":
            return action(content.content)

        case "note":
            return note(content.content)
    }
}

function purpose(content: string[]): VNodeContent {
    return content.map(notice_info)
}
function module(content: string[]): VNodeContent {
    return html`<section class="paragraph">
        <ul>
            ${content.map(li)}
        </ul>
    </section>`

    function li(message: string): VNode {
        return html`<li>${icon("angle-double-right")} ${message}</li>`
    }
}
function item(title: string, content: string[]): VNodeContent {
    return [
        html`<p>${title}</p>`,
        html`<section class="paragraph">
            <ul>
                ${content.map(li)}
            </ul>
        </section>`,
    ]

    function li(message: string): VNode {
        return html`<li>${icon("chevron-right")} ${message}</li>`
    }
}
function description(contents: DocsDescription[]): VNodeContent {
    return contents.map((content) =>
        field({
            title: content.title,
            body: body(content.body),
            help: content.help,
        }),
    )

    function body(messages: string[]): VNodeContent {
        return html`<ul>
            ${messages.map(li)}
        </ul>`

        function li(message: string): VNode {
            return html`<li>${message}</li>`
        }
    }
}
function explanation(targets: DocsActionTargetType[]): VNodeContent {
    return targets.map((action) => {
        const content = target(action)
        return html`<p>
            ${content.icon} ${content.label}<br />
            <small>（${content.help}）</small>
        </p>`
    })
}
function negativeNote(contents: DocsNegativeNote[]): VNodeContent {
    return contents.map(
        (content) => html`<p>
            ${icon("close")} ${content.message}<br />
            <small>（${content.help}）</small>
        </p>`,
    )
}
function action(contents: DocsAction[]): VNodeContent {
    return contents.map(action)

    function action(action: DocsAction): VNode {
        return field({
            title: title(),
            body: body(action.content.body),
            help: action.content.help,
        })

        function title(): VNodeContent {
            switch (action.type) {
                case "request":
                    return html`${label(action.content.from)} ${" "} ${icon("arrow-right")} ${" "}
                    ${label(action.content.to)}`

                case "action":
                    return label(action.content.on)
            }

            function label(type: DocsActionTargetType): VNode {
                const content = target(type)
                return html`${content.icon} ${content.label}`
            }
        }
        function body(contents: DocsActionContent[]): VNodeContent {
            return html`<ul>
                ${contents.map((content) => li(message(content)))}
            </ul>`

            function li(message: VNodeContent): VNode {
                return html`<li>${message}</li>`
            }
            function message(content: DocsActionContent): VNodeContent {
                switch (content.type) {
                    case "normal":
                        return content.message

                    case "validate":
                        return html`${label_alert("検証")} ${content.message}`
                }
            }
        }
    }
}
function note(contents: string[]): VNodeContent {
    return html`<hr />
        <section class="paragraph">
            <ul>
                ${contents.map(li)}
            </ul>
        </section>`

    function li(content: string): VNode {
        return html`<li>${content}</li>`
    }
}

type Explanation = Readonly<{
    label: string
    icon: VNode
    help: string
}>
function target(target: DocsActionTargetType): Explanation {
    switch (target) {
        case "content-server":
            return {
                label: "コンテンツサーバー",
                icon: icon("database"),
                help: "CDN : CloudFront など",
            }

        case "api-server":
            return {
                label: "APIサーバー",
                icon: icon("cogs"),
                help: "アプリケーションサーバー",
            }

        case "http-client":
            return {
                label: "ブラウザ",
                icon: icon("display"),
                help: "ブラウザ、スマホアプリ",
            }

        case "text-client":
            return {
                label: "メール",
                icon: icon("envelope"),
                help: "メール、slack",
            }
    }
}
