import { RemoteCommonError } from "../../../../../ui/vendor/getto-application/infra/remote/data"
import { LoginID } from "../../../login_id/_ui/data"
import { Password } from "../data"

export type AuthenticatePasswordFields = Readonly<{
    loginID: LoginID
    password: Password
}>

export type AuthenticatePasswordError =
    | Readonly<{ type: "validation-error" }>
    | AuthenticatePasswordRemoteError

export type AuthenticatePasswordRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "invalid-password" }>
