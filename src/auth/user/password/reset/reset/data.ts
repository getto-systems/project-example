import { RemoteCommonError } from "../../../../../common/util/remote/data"
import { Password } from "../../input/field/data"

export type ResetToken = string & { ResetToken: never }

export type ResetPasswordFields = Readonly<{
    newPassword: Password
}>

export type ResetPasswordError = Readonly<{ type: "empty-reset-token" }> | ResetPasswordRemoteError

export type ResetPasswordRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "invalid-reset" }>
    | Readonly<{ type: "already-reset" }>
