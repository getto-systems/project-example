import { html } from "htm/preact"
import { VNode } from "preact"

import { box } from "../../../../ui/vendor/getto-css/preact/design/box"
import { label_alert, label_info, label_success, label_warning, notice_info } from "../../../../ui/vendor/getto-css/preact/design/highlight"
import { icon } from "../../../example/_ui/x_preact/design/icon"

import {
    DocsAction,
    DocsActionItem,
    DocsActionItemType,
    DocsData,
    DocsDomainContent,
    DocsUsecaseContent,
} from "../../../../ui/vendor/getto-application/docs/data"
import { field,  } from "../../../../ui/vendor/getto-css/preact/design/form"

export function domainBox(docs: DocsDomainContent): VNode {
    return box({
        title: docs.title,
        body: [...docs.purpose.map(notice_info), usecase(docs.usecase)],
    })

    function usecase(docs: DocsUsecaseContent[]) {
        return html`<section class="paragraph">
            <ul>
                ${docs.map(li)}
            </ul>
        </section>`

        function li(usecase: DocsUsecaseContent): VNode {
            return html`<li>${icon("angle-double-right")} ${usecase.title}</li>`
        }
    }
}

export function usecaseBox(docs: DocsUsecaseContent): VNode {
    return box({
        title: docs.title,
        body: [...docs.purpose.map(notice_info), action(docs.action)],
    })

    function action(docs: DocsAction[]) {
        return html`<section class="paragraph">
            <ul>
                ${docs.map(li)}
            </ul>
        </section>`

        function li(action: DocsAction): VNode {
            return html`<li>${icon("chevron-right")} ${action.title}</li>`
        }
    }
}

export function actionBox(docs: DocsAction): VNode {
    return box({
        title: docs.title,
        body: docs.item.map(item),
    })

    function item(item: DocsActionItem): VNode {
        return field({
            title: itemType(item.type),
            body: content(item.content),
            help: item.help,
        })
    }
    function itemType(type: DocsActionItemType): VNode {
        switch (type) {
            case "input":
                return label_info("入力")

            case "check":
                return label_warning("確認")

            case "success":
                return label_success("完了")

            case "error":
                return label_alert("エラー")
        }
    }
    function content(content: string[]) {
        return html`<section class="paragraph">
            <ul>
                ${content.map(li)}
            </ul>
        </section>`

        function li(description: string): VNode {
            return html`<li>${description}</li>`
        }
    }
}

export function dataBox(docs: DocsData): VNode {
    return box({
        title: docs.title,
        body: docs.data.map((data) =>
            field({
                title: "",
                body: data.description,
                help: data.help,
            }),
        ),
    })
}
