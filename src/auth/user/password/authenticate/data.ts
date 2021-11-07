import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { LoginID } from "../../login_id/input/data"
import { Password } from "../input/data"

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
