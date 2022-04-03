import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { LoginId } from "../input/data"

export type OverrideLoginIdFields = Readonly<{
    newLoginId: LoginId
}>

export type ChangeLoginIdError = Readonly<{ type: "validation-error" }> | ChangeLoginIdRemoteError

export type ChangeLoginIdRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "not-found" }>
    | Readonly<{ type: "invalid" }>
    | Readonly<{ type: "already-registered" }>
