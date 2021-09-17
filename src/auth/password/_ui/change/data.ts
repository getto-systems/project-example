import { RemoteCommonError } from "../../../../z_details/_ui/remote/data"
import { Password } from "../data"

export type ChangePasswordFields = Readonly<{
    currentPassword: Password
    newPassword: Password
}>

export type ChangePasswordError =
    | Readonly<{ type: "validation-error" }>
    | ChangePasswordRemoteError

export type ChangePasswordRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "invalid-password" }>
