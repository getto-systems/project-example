import { PagerOptionsContent, SortSign } from "../../../../ui/vendor/getto-css/preact/design/data"

import { VNodeContent } from "./common"
import { icon } from "./icon"

export const sortSign: SortSign = {
    normal: icon("angle-double-down"),
    reverse: icon("angle-double-up"),
}

export function pagerCount(all: number): VNodeContent {
    return `全 ${pageCountFormat(all)} 件中`
}
export function pagerParams(all: number): PagerOptionsContent {
    return {
        all,
        step: 1000,
        content: ({ start, end }) => `${pageCountFormat(start)} ～ ${pageCountFormat(end)} 件`,
    }
}

export function pageCountFormat(count: number): string {
    return Intl.NumberFormat("ja-JP").format(count)
}
