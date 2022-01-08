import { h, VNode } from "preact"

import { field, pager } from "../../../../../../ui/vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../x_preact/common"

import { SelectBoardComponent } from "../../../../../../ui/vendor/getto-application/board/action_input/x_preact/select"

import { SearchOffsetAction } from "../action"

type Props = Readonly<{
    field: SearchOffsetAction
    count: VNodeContent
    options: VNode[]
    button: VNode
}>
export function SearchOffsetComponent(props: Props): VNode {
    return field({
        title: props.count,
        body: [
            pager(h(SelectBoardComponent, { input: props.field.input, options: props.options })),
            props.button,
        ],
    })
}
