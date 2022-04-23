import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { LoginId } from "../kernel/data"

export type OverrideLoginIdFields = Readonly<{
    newLoginId: LoginId
}>

export type ChangeLoginIdError = ChangeLoginIdRemoteError

export type ChangeLoginIdRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "not-found" }>
    | Readonly<{ type: "invalid" }>
    | Readonly<{ type: "already-registered" }>