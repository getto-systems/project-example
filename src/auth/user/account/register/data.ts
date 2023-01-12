import { RemoteCommonError } from "../../../../common/util/remote/data"

export type RegisterAuthUserAccountError = RegisterAuthUserAccountRemoteError

export type RegisterAuthUserAccountRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "login-id-already-registered" }>
    | Readonly<{ type: "invalid" }>
