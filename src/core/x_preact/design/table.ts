import { VNodeContent } from "../../../z_lib/ui/x_preact/common"
import { PagerOptionsContent, SortSign } from "../../../z_vendor/getto-css/preact/design/table"

import {
    iconHtml,
} from "./icon"
import {
    icon_edit,
    icon_edit_focused,
    icon_reload,
    icon_search,
    icon_spinner
} from "../../../x_content/icon"
import { lnir } from "../../../z_lib/ui/icon/init/line_icon"
import { box_grow, container } from "../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../z_vendor/getto-css/preact/design/highlight"

import { SearchPageResponse } from "../../../z_lib/ui/search/kernel/data"
import { VNode } from "preact"
import { html } from "htm/preact"

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
    if (isFocused) {
        return LIST_EDIT_FOCUSED_LABEL
    } else {
        return LIST_EDIT_LABEL
    }
}

export const EMPTY_TABLE: VNode = container(
    box_grow({ body: notice_gray("指定された条件で 1件も見つかりませんでした") }),
)

export const TAKE_LONGTIME_TO_SEARCH_TABLE: VNode = container(
    box_grow({
        body: notice_gray(
            html`検索中です<br />
                30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
        ),
    }),
)

// TODO なんかどこかでまとめた気がする: static, connect とか
export const SEARCH_BUTTON_STATIC: VNode = html`検索 ${iconHtml(icon_search)}`
export const SEARCH_BUTTON_CONNECT: VNode = html`検索 ${iconHtml(icon_spinner)}`

export const PAGER_BUTTON_STATIC: VNode = html`表示 ${iconHtml(icon_reload)}`
export const PAGER_BUTTON_CONNECT: VNode = html`表示 ${iconHtml(icon_spinner)}`

export const LIST_EDIT_LABEL: VNode = html`編集 ${iconHtml(icon_edit)}`
export const LIST_EDIT_FOCUSED_LABEL: VNode = html`編集 ${iconHtml(icon_edit_focused)}`

export const BACK_TO_LIST_BUTTON: VNode = html`一覧に戻る`

export function pageCountFormat(count: number): string {
    return Intl.NumberFormat("ja-JP").format(count)
}
