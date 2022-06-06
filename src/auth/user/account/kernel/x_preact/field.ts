import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { AuthUserField, TypeAuthUser } from "../data"

type Props<K extends AuthUserField> = Readonly<{ [key in K]: TypeAuthUser<K> }>

export function authUserMemo(data: Props<"memo">): VNodeContent {
    return data.memo
}
