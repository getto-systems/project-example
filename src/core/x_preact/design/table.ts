import { VNodeContent } from "../../../z_lib/ui/x_preact/common"
import { PagerOptionsContent, SortSign } from "../../../z_vendor/getto-css/preact/design/table"

import { iconHtml } from "./icon"
import { lnir } from "../../../z_lib/ui/icon/init/line_icon"
import { box_grow, container } from "../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../z_vendor/getto-css/preact/design/highlight"

import { SearchPageResponse } from "../../../z_lib/ui/search/data"
import { VNode } from "preact"

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

export const EMPTY_TABLE: VNode = container(
    box_grow({ body: notice_gray("指定された条件で 1件も見つかりませんでした") }),
)

export function pageCountFormat(count: number): string {
    return Intl.NumberFormat("ja-JP").format(count)
}
