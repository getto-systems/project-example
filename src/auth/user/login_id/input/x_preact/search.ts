import { h, VNode } from "preact"

import { search } from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputBoard } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { SearchLoginIdAction } from "../action"
import { AUTH_USER_ACCOUNT } from "../../../account/kernel/data"

type Props = Readonly<{ field: SearchLoginIdAction }> &
    Partial<{ title: VNodeContent; help: readonly VNodeContent[] }>
export function SearchLoginIdField(props: Props): VNode {
    // TODO searchField にして label で囲むようにしたい
    return search({
        title: props.title || AUTH_USER_ACCOUNT["login-id"],
        body: h(InputBoard, { type: "text", input: props.field.input }),
        help: props.help,
    })
}
