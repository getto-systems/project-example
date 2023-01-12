import { RemoteCommonError } from "../../../../common/util/remote/data"
import { Password } from "../input/data"

export type ChangePasswordFields = Readonly<{
    currentPassword: Password
    newPassword: Password
}>

export type OverwritePasswordFields = Readonly<{
    newPassword: Password
}>

export type ChangePasswordError = ChangePasswordRemoteError

export type ChangePasswordRemoteError = RemoteCommonError | Readonly<{ type: "invalid-password" }>
