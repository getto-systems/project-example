import { VNode } from "preact"

import { field } from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { LoginId } from "../../kernel/data"
import { AUTH_USER_ACCOUNT } from "../../../account/kernel/data"

type Props = Readonly<{
    user: Readonly<{ loginId: LoginId }>
}> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
    }>
export function StaticLoginIdField(props: Props): VNode {
    return field({
        title: props.title || AUTH_USER_ACCOUNT["login-id"],
        body: props.user.loginId,
        help: props.help,
    })
}
