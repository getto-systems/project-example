import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { Password } from "../input/data"

export type ChangePasswordFields = Readonly<{
    currentPassword: Password
    newPassword: Password
}>

export type OverridePasswordFields = Readonly<{
    newPassword: Password
}>

export type ChangePasswordError =
    | Readonly<{ type: "validation-error" }>
    | ChangePasswordRemoteError

export type ChangePasswordRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "invalid-password" }>
