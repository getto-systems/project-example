import { html } from "htm/preact"
import { VNode } from "preact"

import { box } from "../../../z_vendor/getto-css/preact/design/box"
import {
    label_alert,
    label_info,
    label_success,
    label_warning,
    notice_info,
} from "../../../z_vendor/getto-css/preact/design/highlight"
import { lnir } from "../../../z_lib/ui/icon/line_icon"
import { icon } from "../../../core/x_preact/design/icon"

import {
    DocsAction,
    DocsActionContent,
    DocsData,
    DocsDomain,
    DocsUsecase,
} from "../../../z_vendor/getto-application/docs/data"
import { field } from "../../../z_vendor/getto-css/preact/design/form"

export function docsDomainBox(docs: DocsDomain): readonly VNode[] {
    return [
        box({
            title: "目的",
            body: purpose(docs).map(notice_info),
        }),
        box({
            title: "項目",
            body: usecase(docs.usecase),
        }),
    ]

    function purpose(docs: DocsDomain): readonly string[] {
        return docs.usecase
            .flatMap((usecase) => usecase.purpose)
            .reduce((acc, purpose) => {
                if (!acc.includes(purpose)) {
                    acc.push(purpose)
                }
                return acc
            }, <string[]>[])
    }
    function usecase(docs: readonly DocsUsecase[]) {
        return html`<section class="paragraph">
            <ul>
                ${docs.map(li)}
            </ul>
        </section>`

        function li(usecase: DocsUsecase): VNode {
            return html`<li>${icon(lnir(["angle-double-right"]))} ${usecase.title}</li>`
        }
    }
}

export function docsUsecaseAbstractBox(docs: DocsUsecase): VNode {
    return box({
        title: docs.title,
        body: action(docs.action),
    })

    function action(docs: readonly DocsAction[]) {
        return html`<section class="paragraph">
            <ul>
                ${docs.map(li)}
            </ul>
        </section>`

        function li(action: DocsAction): VNode {
            return html`<li>${icon(lnir(["chevron-right"]))} ${action.title}</li>`
        }
    }
}

export function docsUsecaseBox(docs: DocsUsecase): readonly VNode[] {
    return [
        box({
            title: "目的",
            body: docs.purpose.map(notice_info),
        }),
        box({
            title: "操作",
            body: action(docs.action),
        }),
    ]

    function action(docs: readonly DocsAction[]) {
        return html`<section class="paragraph">
            <ul>
                ${docs.map(li)}
            </ul>
        </section>`

        function li(action: DocsAction): VNode {
            return html`<li>${icon(lnir(["chevron-right"]))} ${action.title}</li>`
        }
    }
}

export function docsActionBox(docs: DocsAction): VNode {
    return box({
        title: docs.title,
        body: docs.action.map(docsActionField),
    })
}

export function docsActionField(action: DocsActionContent): VNode {
    return field({
        title: itemType(action),
        body: ul(content(action)),
        help: action.help,
    })

    function itemType(action: DocsActionContent): VNode {
        switch (action.type) {
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
    function content(action: DocsActionContent): readonly string[] {
        switch (action.type) {
            case "input":
                return action.content

            case "check":
                return action.check

            case "success":
                return action.action

            case "error":
                return action.err
        }
    }

    function ul(content: readonly string[]) {
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

export function docsDataBox(docs: DocsData): VNode {
    return box({
        title: docs.title,
        body: docs.data.map((data) =>
            field({
                title: "",
                body: data.data,
                help: data.help ? data.help : [],
            }),
        ),
    })
}
