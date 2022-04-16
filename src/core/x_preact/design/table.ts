import { VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"
import { PagerOptionsContent, SortSign } from "../../../z_vendor/getto-css/preact/design/table"

import { iconHtml } from "./icon"
import { icon_edit, icon_edit_focused } from "../../../x_content/icon"
import { lnir } from "../../../z_lib/ui/icon/init/line_icon"
import { box_grow, container } from "../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../z_vendor/getto-css/preact/design/highlight"

import { SearchPageResponse } from "../../../z_lib/ui/search/kernel/data"

export const SORT_SIGN: SortSign = {
    normal: iconHtml(lnir(["angle-double-down"])),
    reverse: iconHtml(lnir(["angle-double-up"])),
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
