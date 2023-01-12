import { RemoteCommonError } from "../../../../common/util/remote/data"
import { LoginId } from "../kernel/data"

export type OverwriteLoginIdFields = Readonly<{
    newLoginId: LoginId
}>

export type ChangeLoginIdError = ChangeLoginIdRemoteError

export type ChangeLoginIdRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "not-found" }>
    | Readonly<{ type: "invalid" }>
    | Readonly<{ type: "already-registered" }>
