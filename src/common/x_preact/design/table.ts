import { VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"
import {
    PagerOptionsContent,
    SortSignContent,
} from "../../../z_vendor/getto-css/preact/design/table"

import { iconHtml } from "./icon"
import {
    icon_edit,
    icon_edit_focused,
    icon_sort_normal,
    icon_sort_reverse,
} from "../../../x_content/icon"
import { box_grow, container } from "../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../z_vendor/getto-css/preact/design/highlight"

import { SearchPageResponse } from "../../../z_lib/ui/search/kernel/data"

export const SORT_SIGN: SortSignContent = {
    normal: () => iconHtml(icon_sort_normal),
    reverse: () => iconHtml(icon_sort_reverse),
}

export function pagerCount(all: number): VNodeContent {
    return `全 ${pageCountFormat(all)} 件中`
}
export function pagerParams(page: SearchPageResponse): PagerOptionsContent {
    return {
        all: page.all,
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
export function listEditLabel(isFocused: boolean): VNode {
    return html`編集 ${iconHtml(isFocused ? icon_edit_focused : icon_edit)}`
}

export function emptyTable(): VNode {
    return container(box_grow({ body: notice_gray("指定された条件で 1件も見つかりませんでした") }))
}

export function emptyRegisteredTable(label?: VNodeContent): VNode {
    return container(
        box_grow({ body: notice_gray(html`${label || "登録"}するとここに表示されます`) }),
    )
}

export function takeLongtimeTable(): VNode {
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