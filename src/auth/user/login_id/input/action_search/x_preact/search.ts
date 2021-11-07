import { h, VNode } from "preact"

import { search } from "../../../../../../../ui/vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { InputBoardComponent } from "../../../../../../../ui/vendor/getto-application/board/action_input/x_preact/input"

import { SearchLoginIDResource } from "../resource"

import { loginIDLabel } from "../../data"

type SearchLoginIDOptions =
    | Readonly<{ title: VNodeContent; help: VNodeContent[] }>
    | Readonly<{ title: VNodeContent }>
    | Readonly<{ help: VNodeContent[] }>
    | {
          /* no props */
      }

type Props = SearchLoginIDResource & SearchLoginIDOptions
export function SearchLoginIDComponent(props: Props): VNode {
    return search({
        title: title(),
        body: h(InputBoardComponent, { type: "text", input: props.field.input }),
        help: help(),
    })

    function title(): VNodeContent {
        if ("title" in props) {
            return props.title
        }
        return loginIDLabel
    }
    function help(): VNodeContent[] {
        if ("help" in props) {
            return props.help
        }
        return []
    }
}
