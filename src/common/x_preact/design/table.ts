import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../node"

import { PagerOptionsProps, SortSignContent } from "../../../z_vendor/getto-css/preact/design/table"

import { iconHtml } from "../../util/icon/x_preact/icon"
import { icon_edit, icon_sort_normal, icon_sort_reverse } from "../../../x_content/icon"
import { box_grow, container } from "../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../z_vendor/getto-css/preact/design/highlight"

import { SearchPageResponse } from "../../util/search/kernel/data"

export const SORT_SIGN: SortSignContent = {
    normal: () => iconHtml(icon_sort_normal),
    reverse: () => iconHtml(icon_sort_reverse),
}

export function pagerCount(all: number): PreactContent {
    return `全 ${pageCountFormat(all)} 件中`
}
export function pagerParams(page: SearchPageResponse): PagerOptionsProps {
    return {
        all: page.count,
        step: page.limit,
        content: ({ start, end }) => `${pageCountFormat(start)} ～ ${pageCountFormat(end)} 件`,
    }
}

export function focusClass(isFocused: boolean): string {
    if (isFocused) {
        return "focused"
    } else {
        return ""
    }
}
export function listEditLabel(): PreactNode {
    return html`編集 ${iconHtml(icon_edit)}`
}

export function emptyTable(): PreactNode {
    return container(box_grow({ body: notice_gray("指定された条件で 1件も見つかりませんでした") }))
}

export function emptyRegisteredTable(label?: PreactContent): PreactNode {
    return container(
        box_grow({ body: notice_gray(html`${label || "登録"}するとここに表示されます`) }),
    )
}

export function takeLongtimeTable(): PreactNode {
    return container(
        box_grow({
            body: notice_gray(
                html`検索に時間がかかっています<br />
                    30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
            ),
        }),
    )
}

export function pageCountFormat(count: number): string {
    return Intl.NumberFormat("ja-JP").format(count)
}
