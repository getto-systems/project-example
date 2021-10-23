import { h, VNode } from "preact"

import { field } from "../../../../../../ui/vendor/getto-css/preact/design/form"
import { tableViewColumns } from "../../../../../../ui/vendor/getto-css/preact/design/data"

import { VNodeContent } from "../../../../../example/x_preact/design/common"

import { CheckboxBoardComponent, CheckboxBoardContent } from "../../../../../../ui/vendor/getto-application/board/action_input/x_preact/checkbox"

import { SearchColumnsResource } from "../resource"

type SearchOptions =
    | Readonly<{ title: VNodeContent; options: CheckboxBoardContent[] }>
    | Readonly<{ title: VNodeContent; options: CheckboxBoardContent[]; block: boolean }>

type Props = SearchColumnsResource & SearchOptions
export function SearchColumnsComponent(props: Props): VNode {
    return field({
        title: props.title,
        body: [tableViewColumns(h(CheckboxBoardComponent, { input: props.field.input, ...props }))],
    })
}
