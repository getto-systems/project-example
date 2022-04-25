import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { AuthRole } from "../../kernel/data"
import { AuthUserMemo } from "../kernel/data"

export type ModifyAuthUserAccountFields = Readonly<{
    memo: AuthUserMemo
    grantedRoles: readonly AuthRole[]
}>

export type ModifyAuthUserAccountError = ModifyAuthUserAccountRemoteError

export type ModifyAuthUserAccountRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "conflict" }>
    | Readonly<{ type: "not-found" }>
    | Readonly<{ type: "invalid" }>
