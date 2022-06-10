import { h, VNode } from "preact"
import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { label, search } from "../../../../../z_vendor/getto-css/preact/design/form"

import { InputBoard } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { TextFilterAction } from "../../../../../z_lib/ui/input/filter/text"

import { AUTH_USER_ACCOUNT } from "../../../account/kernel/data"

type Props = Readonly<{ field: TextFilterAction }> &
    Partial<{ title: VNodeContent; help: readonly VNodeContent[] }>
export function LoginIdFilter(props: Props): VNode {
    return search({
        label,
        title: props.title || AUTH_USER_ACCOUNT["loginId"],
        body: h(InputBoard, { type: "text", input: props.field.input }),
        help: props.help,
    })
}
