import { RemoteCommonError } from "../../../../z_details/_ui/remote/data"
import { LoginID } from "../../../login_id/_ui/data"
import { Password } from "../../input/data"

export type ResetPasswordFields = Readonly<{
    loginID: LoginID
    password: Password
}>

export type ResetPasswordError =
    | Readonly<{ type: "validation-error" }>
    | Readonly<{ type: "empty-reset-token" }>
    | ResetPasswordRemoteError

export type ResetPasswordRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "invalid-reset" }>
    | Readonly<{ type: "already-reset" }>
