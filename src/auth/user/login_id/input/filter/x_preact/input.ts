import { h } from "preact"
import { PreactContent, PreactNode } from "../../../../../../common/x_preact/vnode"

import { label, search } from "../../../../../../z_vendor/getto-css/preact/design/form"

import { InputBoard } from "../../../../../../common/util/board/input/x_preact/input"

import { TextFilterBoard } from "../../../../../../common/util/board/filter/action"

import { AUTH_USER_ACCOUNT } from "../../../../account/kernel/data"

type Props = Readonly<{
    filter: TextFilterBoard
}> &
    Partial<{
        title: PreactContent
        help: readonly PreactContent[]
    }>

export function AuthUserLoginIdFilter(props: Props): PreactNode {
    return search({
        label: label,
        title: props.title || AUTH_USER_ACCOUNT["loginId"],
        help: props.help,
        body: h(InputBoard, { type: "text", input: props.filter.input }),
    })
}
