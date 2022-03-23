import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { GrantedRole, ResetTokenDestination } from "../input/data"

export type ModifyAuthUserAccountFields = Readonly<{
    grantedRoles: readonly GrantedRole[]
    resetTokenDestination: ResetTokenDestination
}>

export type ModifyAuthUserAccountError =
    | Readonly<{ type: "validation-error" }>
    | ModifyAuthUserAccountRemoteError

export type ModifyAuthUserAccountRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "conflict" }>
    | Readonly<{ type: "invalid-granted-role" }>
    | Readonly<{ type: "invalid-reset-token-destination-email" }>
