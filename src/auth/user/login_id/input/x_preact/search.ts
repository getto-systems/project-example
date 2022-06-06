import { h, VNode } from "preact"

import { label, search } from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputBoard } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { FilterLoginIdAction } from "../action"
import { AUTH_USER_ACCOUNT } from "../../../account/kernel/data"

type Props = Readonly<{ field: FilterLoginIdAction }> &
    Partial<{ title: VNodeContent; help: readonly VNodeContent[] }>
export function SearchLoginIdField(props: Props): VNode {
    return search({
        label,
        title: props.title || AUTH_USER_ACCOUNT["loginId"],
        body: h(InputBoard, { type: "text", input: props.field.input }),
        help: props.help,
    })
}
