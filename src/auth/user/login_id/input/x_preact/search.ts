import { h, VNode } from "preact"

import { search } from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputBoardComponent } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { SearchLoginIDAction } from "../action"

type Props = Readonly<{ field: SearchLoginIDAction }> &
    Partial<{ title: VNodeContent; help: readonly VNodeContent[] }>
export function SearchLoginIDComponent(props: Props): VNode {
    return search({
        title: title(),
        body: h(InputBoardComponent, { type: "text", input: props.field.input }),
        help: help(),
    })

    function title(): VNodeContent {
        if (props.title) {
            return props.title
        }
        return "ログインID"
    }
    function help(): readonly VNodeContent[] {
        if (props.help) {
            return props.help
        }
        return []
    }
}
