import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"

export type RegisterAuthUserAccountError = RegisterAuthUserAccountRemoteError

export type RegisterAuthUserAccountRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "login-id-already-registered" }>
    | Readonly<{ type: "invalid" }>
