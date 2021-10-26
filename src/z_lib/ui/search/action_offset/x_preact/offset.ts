import { h, VNode } from "preact"

import { field, pager } from "../../../../../../ui/vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../example/x_preact/design/common"

import { SelectBoardComponent } from "../../../../../../ui/vendor/getto-application/board/action_input/x_preact/select"

import { SearchOffsetResource } from "../resource"

type SearchOptions = Readonly<{ title: VNodeContent; options: VNode[]; button: VNode }>

type Props = SearchOffsetResource & SearchOptions
export function SearchOffsetComponent(props: Props): VNode {
    return field({
        title: props.title,
        body: [
            pager(h(SelectBoardComponent, { input: props.field.input, options: props.options })),
            props.button,
        ],
    })
}
