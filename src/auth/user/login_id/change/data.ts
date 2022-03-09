import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { LoginId } from "../input/data"

export type OverrideLoginIdFields = Readonly<{
    newLoginId: LoginId
}>

export type ChangeLoginIdError = Readonly<{ type: "validation-error" }> | ChangeLoginIdRemoteError

export type ChangeLoginIdRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "invalid-login-id" }>
    | Readonly<{ type: "already-registered" }>
