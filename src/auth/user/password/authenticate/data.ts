import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { LoginId } from "../../login_id/kernel/data"
import { Password } from "../input/data"

export type AuthenticatePasswordFields = Readonly<{
    loginId: LoginId
    password: Password
}>

export type AuthenticatePasswordError = AuthenticatePasswordRemoteError

export type AuthenticatePasswordRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "invalid-password" }>
