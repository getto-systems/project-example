import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { GrantedAuthRole } from "../input/data"

export type ModifyAuthUserAccountFields = Readonly<{
    grantedRoles: readonly GrantedAuthRole[]
}>

export type ModifyAuthUserAccountError =
    | Readonly<{ type: "validation-error" }>
    | ModifyAuthUserAccountRemoteError

export type ModifyAuthUserAccountRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "conflict" }>
    | Readonly<{ type: "not-found" }>
    | Readonly<{ type: "invalid" }>
