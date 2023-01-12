import { RemoteCommonError } from "../../../../../common/util/remote/data"
import { LoginId } from "../../../login_id/kernel/data"
import { Password } from "../../input/data"

export type ResetToken = string & { ResetToken: never }

export type ResetPasswordFields = Readonly<{
    loginId: LoginId
    newPassword: Password
}>

export type ResetPasswordError = Readonly<{ type: "empty-reset-token" }> | ResetPasswordRemoteError

export type ResetPasswordRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "invalid-reset" }>
    | Readonly<{ type: "already-reset" }>
