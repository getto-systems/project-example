import { RemoteCommonError } from "../../../../../z_lib/ui/remote/data"

import { LoginId } from "../../../login_id/input/data"

export type RequestResetTokenFields = Readonly<{
    loginId: LoginId
}>

export type RequestResetTokenError =
    | Readonly<{ type: "validation-error" }>
    | RequestResetTokenRemoteError

export type RequestResetTokenRemoteError = RemoteCommonError | Readonly<{ type: "invalid-reset" }>
