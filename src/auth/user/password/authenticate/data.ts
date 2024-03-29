import { RemoteCommonError } from "../../../../common/util/remote/data"
import { LoginId } from "../../login_id/kernel/data"
import { Password } from "../input/field/data"

export type AuthenticatePasswordFields = Readonly<{
    loginId: LoginId
    password: Password
}>

export type AuthenticatePasswordError = AuthenticatePasswordRemoteError

export type AuthenticatePasswordRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "invalid-password" }>
