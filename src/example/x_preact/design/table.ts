import { VNodeContent } from "./common"
import { icon } from "./icon"

import { PagerOptionsContent, SortSign } from "../../../../ui/vendor/getto-css/preact/design/data"
import { SearchPageResponse } from "../../../z_lib/ui/search/data"

export const siteSortSign: SortSign = {
    normal: icon("angle-double-down"),
    reverse: icon("angle-double-up"),
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

export function pageCountFormat(count: number): string {
    return Intl.NumberFormat("ja-JP").format(count)
}
