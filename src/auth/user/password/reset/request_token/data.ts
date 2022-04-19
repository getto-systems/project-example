import { RemoteCommonError } from "../../../../../z_lib/ui/remote/data"

import { LoginId } from "../../../login_id/kernel/data"

export type RequestResetTokenFields = Readonly<{
    loginId: LoginId
}>

export type RequestResetTokenError = RequestResetTokenRemoteError

export type RequestResetTokenRemoteError = RemoteCommonError | Readonly<{ type: "invalid-reset" }>
