import { h, VNode } from "preact"

import { field, pager } from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../x_preact/common"

import { SelectBoardComponent } from "../../../../../z_vendor/getto-application/board/input/x_preact/select"

import { markBoardValue } from "../../../../../z_vendor/getto-application/board/kernel/convert"

import { SearchOffsetAction } from "../action"

type Props = Readonly<{
    field: SearchOffsetAction
    count: VNodeContent
    selected: number
    options: readonly VNode[]
    button: VNode
}>
export function SearchOffsetComponent(props: Props): VNode {
    return field({
        title: props.count,
        body: [
            pager(
                h(SelectBoardComponent, {
                    input: props.field.input,
                    selected: markBoardValue(`${props.selected}`),
                    options: props.options,
                }),
            ),
            props.button,
        ],
    })
}
