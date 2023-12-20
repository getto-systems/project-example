import { h } from "preact"
import { useAtom } from "../../../../../../z_vendor/getto-atom/x_preact/hooks"
import { PreactContent, PreactNode } from "../../../../../../common/x_preact/node"

import { label, search_double } from "../../../../../../z_vendor/getto-css/preact/design/form"
import { checkboxOptions } from "../../../../../../common/x_preact/design/checkbox"

import { CheckboxBoard } from "../../../../../../common/util/board/input/x_preact/checkbox"
import { authPermissionCheckboxContent } from "../../../kernel/x_preact/field"

import { MultipleFilterBoard } from "../../../../../../common/util/board/filter/action"

import { AUTH_USER_ACCOUNT } from "../../../kernel/data"
import { AuthPermission } from "../../../../kernel/data"

type MultipleProps<T, F> = Readonly<{
    filter: MultipleFilterBoard<T, F>
}> &
    Partial<{
        title: PreactContent
        help: readonly PreactContent[]
    }>

export function AuthPermissionGrantedFilter(
    props: MultipleProps<AuthPermission, AuthPermission>,
): PreactNode {
    const options = useAtom(props.filter.options)

    return search_double({
        label: label,
        title: props.title || AUTH_USER_ACCOUNT["granted"],
        help: props.help,
        body: h(CheckboxBoard, {
            input: props.filter.input,
            options: checkboxOptions(options, authPermissionCheckboxContent),
        }),
    })
}
